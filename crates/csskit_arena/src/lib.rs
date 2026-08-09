//! A contiguous memory Arena for zero-copy access in binding layers.
//!
//! For "raw transfer" (reading objects directly from a binding layer without any serialisation), non-Rust code can
//! reinterpret the low 32 bits of every native pointer as a byte offset into a single contiguous arena buffer. For this
//! to work every interior pointer must share the same upper 32 bits, which is guaranteed if the whole arena lives
//! inside a single region that is:
//!
//! - **4 GiB aligned** (so the base address has zero in its low 32 bits), and
//! - **at most 2 GiB** in size (so no allocation crosses a 4 GiB boundary).
//!
//! This concept is shamelessly stolen from
//! [oxc_allocator](https://github.com/oxc-project/oxc/tree/main/crates/oxc_allocator) which introduces this concept for
//! their JS parser. Credit to the oxc project for this technique.
//!
//! [`Arena`] can be used in several modes, predominantly as a single-chunk bump allocator that satisfies both
//! constraints. In this mode it never grows or relocates. For systems without virtual memory, or for 32-bit systems, it
//! can devolve into a chunk based allocator, or can borrow a block of memory to allocate into.
//!
//! # Ownership modes
//!
//! - [`Arena::new`] / [`Arena::default`]: self-allocated, as large a first chunk as the target allows.
//! - [`Arena::with_capacity`]: self-allocated, exactly the given size, never grows. Useful for tests.
//! - [`Arena::with_initial_capacity`]: self-allocated. The first chunk has the given size. The arena adds more chunks.
//! - [`Arena::from_raw_parts`]: borrows memory from the caller. Use this if a different allocator owns the block.
//!
//! All modes yield the same [`Arena`] type; they differ only in where the first chunk comes from and whether the arena
//! may add another.
use allocator_api2::alloc::{AllocError, Allocator};
use std::alloc::Layout;
use std::cell::Cell;
use std::ptr::NonNull;

mod vm;

/// Required alignment of the arena region (4 GiB), so that `ptr as u32` equals the byte offset
/// within the region. Where nothing is reserved or 32-bit targets that already express 4gb, this is `1`.
#[cfg(all(target_pointer_width = "64", any(unix, windows)))]
pub const BLOCK_ALIGN: usize = 4 * 1024 * 1024 * 1024;
#[cfg(not(all(target_pointer_width = "64", any(unix, windows))))]
pub const BLOCK_ALIGN: usize = 1;

/// Maximum usable size of the arena region (just under 2 GiB), so that no allocation crosses a
/// 4 GiB boundary and every offset fits in a `u32`.
pub const MAX_BLOCK_SIZE: usize = 2 * 1024 * 1024 * 1024 - 16;

/// Smallest step the arena takes when it needs more room: bytes committed the first time a Windows arena is written
/// to, and the size of the first chunk where nothing can be reserved. Growth doubles from there.
const INITIAL_COMMIT: usize = 64 * 1024;

/// Alignment of a chunk from the global allocator: enough for any fundamental type, and raised to the requested
/// alignment when a single allocation needs more.
const CHUNK_ALIGN: usize = 16;

/// How a chunk's memory was obtained, which determines what (if anything) happens on drop.
#[derive(Clone, Copy)]
enum Backing {
	/// Address space reserved by this crate. `reservation` and `reserved` are the base and length of the whole
	/// reservation, both of which are needed to release it.
	Owned { reservation: NonNull<u8>, reserved: usize },
	/// A block from the global allocator, for targets that cannot reserve address space lazily.
	Heap(Layout),
	/// Memory owned by the caller (e.g. a V8 `ArrayBuffer`). Never freed by the arena.
	Borrowed,
}

/// A chunk the arena has bumped past, kept in a chain so every chunk is released when the arena is.
struct Chunk {
	base: NonNull<u8>,
	size: usize,
	backing: Backing,
	prev: Option<NonNull<Chunk>>,
}

/// Where the target allows it the region begins at a 4 GiB-aligned address and is at most [`MAX_BLOCK_SIZE`] bytes, so
/// the low 32 bits of any interior pointer equal its byte offset within the region. Allocation is a pointer bump;
/// deallocation of individual items is a no-op (every chunk is freed at once on drop; or never if using borrowed
/// memory).
pub struct Arena {
	/// Base of the usable region of the chunk being bumped from.
	base: Cell<NonNull<u8>>,
	/// Usable size of that chunk in bytes.
	size: Cell<usize>,
	/// Bump cursor: byte offset of the next free position within the chunk.
	cursor: Cell<usize>,
	/// Bytes of the chunk backed by physical memory. Windows has no lazy commit, so the arena commits as it bumps.
	#[cfg(windows)]
	committed: Cell<usize>,
	backing: Cell<Backing>,
	/// Chunks the arena has bumped past. `None` unless it ran out of room and had to add one.
	prev: Cell<Option<NonNull<Chunk>>>,
	/// Bytes handed out from, and total usable size of, the chunks in `prev`.
	prev_used: Cell<usize>,
	prev_size: Cell<usize>,
	/// Whether a full chunk may be retired in favour of a fresh, larger one.
	growable: bool,
}

impl Arena {
	/// Create a self-allocated arena.
	///
	/// Where address space can be reserved lazily the first chunk is the full [`MAX_BLOCK_SIZE`] region, costing no
	/// physical memory until written to, so one non-relocating chunk serves arbitrarily large parses. Elsewhere every
	/// byte of a chunk is paid for up front, so the arena starts small and adds chunks as it fills.
	#[inline]
	pub fn new() -> Self {
		let size = if vm::RESERVABLE { MAX_BLOCK_SIZE } else { INITIAL_COMMIT };
		let (base, backing) = Self::new_chunk(size).expect("arena backing reservation failed");
		Self::from_chunk(base, size, backing, true)
	}

	/// Create a self-allocated arena of exactly `size` usable bytes (clamped to [`MAX_BLOCK_SIZE`]).
	///
	/// The arena never grows: once `size` bytes are handed out, allocation fails.
	///
	/// # Panics
	/// Panics if the backing allocation fails.
	pub fn with_capacity(size: usize) -> Self {
		let size = size.clamp(1, MAX_BLOCK_SIZE);
		let (base, backing) = Self::new_chunk(size).expect("arena backing reservation failed");
		Self::from_chunk(base, size, backing, false)
	}

	/// Create a self-allocated arena. The first chunk has `size` usable bytes (clamped to [`MAX_BLOCK_SIZE`]).
	///
	/// [`Arena::with_capacity`] does not grow, but this arena adds more chunks when the first chunk is full. `size` is
	/// the initial size and not a limit. Use this function if you can calculate an approximate size from the input, but
	/// the arena must be able to use more memory.
	///
	/// # Panics
	/// Panics if the backing allocation fails.
	pub fn with_initial_capacity(size: usize) -> Self {
		let size = size.clamp(1, MAX_BLOCK_SIZE);
		let (base, backing) = Self::new_chunk(size).expect("arena backing reservation failed");
		Self::from_chunk(base, size, backing, true)
	}

	/// Take `size` usable bytes for a chunk: reserved address space where the target has it, a block from the global
	/// allocator otherwise.
	fn new_chunk(size: usize) -> Option<(NonNull<u8>, Backing)> {
		vm::reserve(size)
			.map(|reservation| {
				debug_assert!(
					(reservation.base.as_ptr() as usize).is_multiple_of(BLOCK_ALIGN),
					"arena base must be 4 GiB aligned"
				);
				let backing = Backing::Owned { reservation: reservation.reservation, reserved: reservation.reserved };
				(reservation.base, backing)
			})
			.or_else(|| Self::heap_chunk(size, CHUNK_ALIGN))
	}

	/// Take `size` bytes from the global allocator for a chunk, aligned to at least `align`.
	///
	/// `None` for a zero-sized or otherwise unrepresentable request: a chunk with no room is no use to the arena, and
	/// the global allocator may not be asked for zero bytes.
	fn heap_chunk(size: usize, align: usize) -> Option<(NonNull<u8>, Backing)> {
		let layout = Layout::from_size_align(size, align).ok().filter(|layout| layout.size() > 0)?;
		// SAFETY: the layout is not zero sized.
		let base = NonNull::new(unsafe { std::alloc::alloc(layout) })?;
		Some((base, Backing::Heap(layout)))
	}

	fn from_chunk(base: NonNull<u8>, size: usize, backing: Backing, growable: bool) -> Self {
		Self {
			base: Cell::new(base),
			size: Cell::new(size),
			cursor: Cell::new(0),
			#[cfg(windows)]
			committed: Cell::new(if matches!(backing, Backing::Owned { .. }) { 0 } else { size }),
			backing: Cell::new(backing),
			prev: Cell::new(None),
			prev_used: Cell::new(0),
			prev_size: Cell::new(0),
			growable,
		}
	}

	/// Create an arena over caller-owned memory.
	///
	/// Intended for bindings where the original caller owns the memory, e.g. NAPI JS where `ArrayBuffer` is owned and
	/// already allocated.
	///
	/// # Safety
	/// `ptr` must be the base of a live, writable region of at least `size` bytes that outlives the arena, aligned to
	/// [`BLOCK_ALIGN`] and no larger than [`MAX_BLOCK_SIZE`], and it must not be handed to another allocator.
	pub unsafe fn from_raw_parts(ptr: NonNull<u8>, size: usize) -> Self {
		debug_assert!((ptr.as_ptr() as usize).is_multiple_of(BLOCK_ALIGN), "borrowed arena base must be 4 GiB aligned");
		debug_assert!(size <= MAX_BLOCK_SIZE, "borrowed arena must not exceed MAX_BLOCK_SIZE");
		Self::from_chunk(ptr, size, Backing::Borrowed, false)
	}

	/// The base address of the usable region.
	#[inline]
	pub fn base_ptr(&self) -> NonNull<u8> {
		self.base.get()
	}

	/// Whether every allocation lives in one region starting at [`Arena::base_ptr`], and so whether the low 32 bits of
	/// every pointer handed out is its offset into that region.
	///
	/// - Over a reserved region this is [`Arena::base_ptr`], until the arena has to add a chunk: there is then no
	///   single region left to be an offset into, and this becomes `None`.
	/// - Where pointers are 32 bits wide they already are their own offsets, so the base is `0` however many chunks
	///   the arena holds: the buffer is the whole address space, which on wasm32 is the linear memory the binding
	///   layer already has.
	/// - On a 64 bit target with nothing to reserve - wasm64 under `memory64`, whose linear memory may exceed 4 GiB -
	///   a pointer's upper half is unconstrained, so its low 32 bits mean nothing and this is always `None`.
	#[inline]
	pub fn transfer_base(&self) -> Option<usize> {
		if !cfg!(target_pointer_width = "64") {
			return Some(0);
		}
		// `BLOCK_ALIGN` is 1 without a reservation, so the alignment alone would wave anything through; and a
		// reservation that fell back to the global allocator is unaligned even though `RESERVABLE` holds.
		let base = self.base.get().as_ptr() as usize;
		(vm::RESERVABLE && self.prev.get().is_none() && base.is_multiple_of(BLOCK_ALIGN)).then_some(base)
	}

	/// Number of bytes handed out so far.
	#[inline]
	pub fn used_bytes(&self) -> usize {
		self.prev_used.get() + self.cursor.get()
	}

	/// Total usable capacity of every chunk in bytes.
	#[inline]
	pub fn capacity(&self) -> usize {
		self.prev_size.get() + self.size.get()
	}

	/// Release every allocation at once by rewinding the bump cursor to the start of the first chunk, freeing any chunk
	/// the arena had to add.
	///
	/// Takes `&mut self` so no allocation can outlive the reset. The first chunk's memory is retained.
	pub fn reset(&mut self) {
		while let Some(node) = self.prev.get() {
			// SAFETY: every chunk in the chain came from `Box::into_raw`, and `&mut self` proves nothing allocated from
			// the chunk being dropped is still live.
			let chunk = *unsafe { Box::from_raw(node.as_ptr()) };
			// SAFETY: as above.
			unsafe { release(self.base.get(), self.backing.replace(chunk.backing)) };
			self.base.set(chunk.base);
			self.size.set(chunk.size);
			self.prev.set(chunk.prev);
			// Every chunk the arena added comes from the global allocator, so is backed in full.
			#[cfg(windows)]
			self.committed.set(chunk.size);
		}
		self.prev_used.set(0);
		self.prev_size.set(0);
		self.cursor.set(0);
	}
}

/// Give a chunk's memory back.
///
/// # Safety
/// `base` and `backing` must be exactly what the chunk was built with, and nothing allocated from it may outlive the
/// call.
unsafe fn release(base: NonNull<u8>, backing: Backing) {
	match backing {
		// SAFETY: `reservation`/`reserved` are exactly what `vm::reserve` returned.
		Backing::Owned { reservation, reserved } => unsafe { vm::release(reservation, reserved) },
		// SAFETY: `base`/`layout` are exactly what the global allocator was asked for.
		Backing::Heap(layout) => unsafe { std::alloc::dealloc(base.as_ptr(), layout) },
		Backing::Borrowed => {}
	}
}

/// Bytes to skip from `addr` to reach the next multiple of `align`.
#[inline]
fn pad_to(addr: usize, align: usize) -> usize {
	debug_assert!(align.is_power_of_two(), "Layout alignment is always a power of two");
	(align - (addr & (align - 1))) & (align - 1)
}

/// How far to commit when `end` bytes are needed and `committed` are already backed, given a region of `size` bytes.
///
/// Growth doubles so that a parse does not pay a syscall per bump, but never overshoots the chunk nor undershoots
/// what was asked for. Only Windows commits, but the arithmetic is compiled everywhere so it can be tested everywhere.
#[inline]
#[cfg(any(test, windows))]
fn commit_target(end: usize, committed: usize, size: usize) -> usize {
	end.max(committed.saturating_mul(2)).max(INITIAL_COMMIT).min(size).max(end)
}

impl Arena {
	/// Ensure the chunk is backed by physical memory up to `end`, which must be within the chunk.
	#[cfg(windows)]
	#[cold]
	#[inline(never)]
	fn commit_to(&self, end: usize) -> bool {
		let committed = self.committed.get();
		let target = commit_target(end, committed, self.size.get());
		// SAFETY: `committed <= target <= size`, so the range lies within the reservation.
		let ptr = unsafe { NonNull::new_unchecked(self.base.get().as_ptr().add(committed)) };
		// SAFETY: ditto.
		if !unsafe { vm::commit(ptr, target - committed) } {
			return false;
		}
		self.committed.set(target);
		true
	}

	/// Retire the current chunk and bump from a fresh, larger one able to serve `layout`.
	#[cold]
	#[inline(never)]
	fn grow_chunk(&self, layout: Layout) -> Option<NonNull<[u8]>> {
		if !self.growable {
			return None;
		}
		let total = self.capacity();
		// The arena stays within MAX_BLOCK_SIZE however many chunks it takes, so every offset it hands out still fits
		// in a u32.
		let spare = MAX_BLOCK_SIZE.checked_sub(total)?;
		// Double, so growth costs a logarithmic number of allocations, but never less than the request nor more than
		// the budget leaves.
		let size = layout.size().max(total.max(INITIAL_COMMIT).min(spare));
		if size > spare {
			return None;
		}
		// Aligning the chunk to the request means the allocation fits at its base.
		let (base, backing) = Self::heap_chunk(size, layout.align().max(CHUNK_ALIGN))?;
		let retired = Chunk {
			base: self.base.get(),
			size: self.size.get(),
			backing: self.backing.replace(backing),
			prev: self.prev.get(),
		};
		self.prev_used.set(self.prev_used.get() + self.cursor.get());
		self.prev_size.set(total);
		// SAFETY: `Box::into_raw` never returns null.
		self.prev.set(Some(unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(retired))) }));
		self.base.set(base);
		self.size.set(size);
		self.cursor.set(0);
		// A chunk from the global allocator is backed in full.
		#[cfg(windows)]
		self.committed.set(size);
		// The fresh chunk is big enough and aligned for `layout`, so this cannot recurse again.
		self.bump(layout)
	}

	/// Bump-allocate `layout.size()` bytes aligned to `layout.align()`.
	///
	/// Returns `None` if the arena is exhausted.
	#[inline]
	fn bump(&self, layout: Layout) -> Option<NonNull<[u8]>> {
		let align = layout.align();
		let bytes = layout.size();
		debug_assert!(align.is_power_of_two(), "Layout alignment is always a power of two");
		let base = self.base.get();
		let start = self.cursor.get();
		debug_assert!(start <= self.size.get(), "cursor must never exceed the chunk size");
		// Round the cursor up to the requested alignment. It is the address that is rounded rather than the offset: a
		// chunk from the global allocator is only CHUNK_ALIGN aligned, so an aligned offset within it need not be one.
		let aligned = start + pad_to(base.as_ptr() as usize + start, align);
		let end = aligned.checked_add(bytes)?;
		if end > self.size.get() {
			return self.grow_chunk(layout);
		}
		#[cfg(windows)]
		if end > self.committed.get() && !self.commit_to(end) {
			return None;
		}
		debug_assert!(aligned >= start && end >= aligned, "bump must advance the cursor monotonically");
		self.cursor.set(end);
		// SAFETY: `aligned + bytes <= size`, so the range is within the chunk.
		let ptr = unsafe { NonNull::new_unchecked(base.as_ptr().add(aligned)) };
		debug_assert_eq!(ptr.as_ptr() as usize % align, 0, "returned pointer must satisfy the requested alignment");
		Some(NonNull::slice_from_raw_parts(ptr, bytes))
	}

	/// A zero-length allocation at the cursor.
	///
	/// Nothing is written, so the cursor does not move and the pages need not be committed, but the pointer is still an
	/// interior pointer of the chunk: handing out an out-of-region dangling pointer would break raw transfer, whose
	/// offsets are the low 32 bits of every pointer.
	#[inline]
	fn empty(&self, align: usize) -> Option<NonNull<[u8]>> {
		let base = self.base.get();
		let start = self.cursor.get();
		let aligned = start + pad_to(base.as_ptr() as usize + start, align);
		if aligned > self.size.get() {
			return None;
		}
		// SAFETY: `aligned <= size`, so the address is within the chunk.
		let ptr = unsafe { NonNull::new_unchecked(base.as_ptr().add(aligned)) };
		Some(NonNull::slice_from_raw_parts(ptr, 0))
	}
}

impl std::fmt::Debug for Arena {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Arena").field("capacity", &self.capacity()).field("used", &self.used_bytes()).finish()
	}
}

impl Default for Arena {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl Drop for Arena {
	fn drop(&mut self) {
		// SAFETY: `&mut self` proves every allocation is dead.
		unsafe { release(self.base.get(), self.backing.get()) };
		let mut node = self.prev.get();
		while let Some(chunk) = node {
			// SAFETY: every chunk in the chain came from `Box::into_raw` and has not been freed.
			let chunk = *unsafe { Box::from_raw(chunk.as_ptr()) };
			node = chunk.prev;
			// SAFETY: as above; every allocation from the chunk is dead.
			unsafe { release(chunk.base, chunk.backing) };
		}
	}
}

unsafe impl Allocator for &Arena {
	#[inline]
	fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		if layout.size() == 0 {
			return self.empty(layout.align()).ok_or(AllocError);
		}
		self.bump(layout).ok_or(AllocError)
	}

	#[inline]
	unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
		// Bump allocator: individual deallocation is a no-op.
	}

	#[inline]
	unsafe fn grow(
		&self,
		ptr: NonNull<u8>,
		old_layout: Layout,
		new_layout: Layout,
	) -> Result<NonNull<[u8]>, AllocError> {
		if new_layout.size() == 0 {
			// Zero-sized: `ptr` holds nothing to copy, so hand back a fresh empty allocation.
			return self.empty(new_layout.align()).ok_or(AllocError);
		}
		if old_layout.size() == 0 {
			// `ptr` is the empty allocation handed out for a zero-sized request; nothing to copy.
			return self.bump(new_layout).ok_or(AllocError);
		}
		debug_assert!(new_layout.size() >= old_layout.size(), "grow must not shrink");
		// If `ptr` is the most recent allocation, extend it in place by bumping the cursor, avoiding
		// stranding the old bytes. This is the common case for a `Vec` growing while it is the last
		// thing allocated. A pointer from a chunk the arena has bumped past falls through to the copy
		// below: chunks never overlap, so its offset within this one cannot also be the live cursor.
		let addr = ptr.as_ptr() as usize;
		let base = self.base.get().as_ptr() as usize;
		let offset = addr.wrapping_sub(base);
		if new_layout.align() <= old_layout.align()
			&& offset < self.size.get()
			&& offset + old_layout.size() == self.cursor.get()
			&& offset + new_layout.size() <= self.size.get()
		{
			let end = offset + new_layout.size();
			#[cfg(windows)]
			if end > self.committed.get() && !self.commit_to(end) {
				return Err(AllocError);
			}
			self.cursor.set(end);
			return Ok(NonNull::slice_from_raw_parts(ptr, new_layout.size()));
		}
		// Otherwise allocate fresh and copy the old bytes over.
		let new = self.bump(new_layout).ok_or(AllocError)?;
		// SAFETY: `ptr` holds `old_layout.size()` initialised bytes; `new` has room for at least that
		// many; the regions do not overlap (fresh allocation).
		unsafe {
			std::ptr::copy_nonoverlapping(ptr.as_ptr(), new.as_ptr() as *mut u8, old_layout.size());
		}
		Ok(new)
	}
}

#[cfg(test)]
mod test {
	use crate::{Arena, BLOCK_ALIGN, INITIAL_COMMIT, MAX_BLOCK_SIZE, commit_target, vm};
	use allocator_api2::alloc::Allocator;
	use std::alloc::Layout;

	/// A growing arena over a chunk from the global allocator, which unlike a reservation is only `CHUNK_ALIGN` aligned
	/// and small enough to outgrow. This is what [`Arena::new`] builds where nothing can be reserved.
	fn growing(size: usize) -> Arena {
		let (base, backing) = Arena::heap_chunk(size, crate::CHUNK_ALIGN).unwrap();
		Arena::from_chunk(base, size, backing, true)
	}

	#[test]
	fn commit_target_covers_the_request_and_doubles() {
		let size = MAX_BLOCK_SIZE;
		// Nothing committed yet: take the initial slab, even for a one byte write.
		assert_eq!(commit_target(1, 0, size), INITIAL_COMMIT);
		// Doubling, once the initial slab is outgrown.
		assert_eq!(commit_target(INITIAL_COMMIT + 1, INITIAL_COMMIT, size), INITIAL_COMMIT * 2);
		// A request larger than double wins over doubling.
		assert_eq!(commit_target(9 << 20, 1 << 20, size), 9 << 20);
		// Never past the end of the region, even when doubling would overshoot.
		assert_eq!(commit_target(size, size - 1, size), size);
		// A region smaller than the initial slab commits only what it has.
		assert_eq!(commit_target(64, 0, 128), 128);
		// ...but the request still wins, so the committed range always covers what was handed out.
		assert_eq!(commit_target(size, 0, size), size);
	}

	#[test]
	fn base_is_4gib_aligned() {
		if !vm::RESERVABLE {
			return;
		}
		let arena = Arena::new();
		assert!((arena.base_ptr().as_ptr() as usize).is_multiple_of(BLOCK_ALIGN));
		let fixed = Arena::with_capacity(4096);
		assert!((fixed.base_ptr().as_ptr() as usize).is_multiple_of(BLOCK_ALIGN));
	}

	#[test]
	fn ptr_low_32_equals_offset() {
		let arena = Arena::new();
		let base = arena.base_ptr().as_ptr() as usize;
		let layout = Layout::array::<u32>(2).unwrap();
		let alloc = (&arena).allocate(layout).unwrap();
		let data = alloc.as_ptr() as *mut u8 as usize;
		let offset = data - base;
		assert!(offset <= MAX_BLOCK_SIZE);
		if vm::RESERVABLE {
			// A 4 GiB aligned base puts the region offset in the low 32 bits of every pointer into it.
			assert_eq!(arena.transfer_base(), Some(base));
			assert_eq!(data as u32 as usize, offset, "ptr low 32 bits must equal arena offset");
		} else if cfg!(target_pointer_width = "64") {
			// wasm64: linear memory may exceed 4 GiB and nothing constrains a pointer's upper half.
			assert_eq!(arena.transfer_base(), None);
		} else {
			// Nothing to align, and no need: a 32 bit pointer already is the offset a binding layer reads, into the
			// whole address space (linear memory, on wasm32) rather than the region.
			assert_eq!(arena.transfer_base(), Some(0));
			assert_eq!(data as u32 as usize, data);
		}
	}

	#[test]
	fn an_unaligned_region_promises_no_offsets() {
		// The shape a 64 bit target with nothing to reserve is stuck with, and the one a failed reservation falls back
		// to: a single chunk, but from the global allocator, so its base has none of the alignment offsets need.
		let arena = growing(1024);
		if cfg!(target_pointer_width = "64") {
			assert_eq!(arena.transfer_base(), None);
		} else {
			assert_eq!(arena.transfer_base(), Some(0));
		}
	}

	#[test]
	fn over_aligned_allocation_is_aligned() {
		// A chunk from the global allocator is only CHUNK_ALIGN aligned, so an allocation wanting more has to be padded
		// to an absolute boundary, not to an offset within the chunk.
		let arena = growing(INITIAL_COMMIT);
		let _ = (&arena).allocate(Layout::from_size_align(1, 1).unwrap()).unwrap();
		let a = (&arena).allocate(Layout::from_size_align(64, 64).unwrap()).unwrap();
		assert_eq!(a.as_ptr() as *mut u8 as usize % 64, 0);
	}

	#[test]
	fn bump_alignment_and_accounting() {
		let arena = Arena::new();
		let base = arena.base_ptr().as_ptr() as usize;
		// A 1-byte alloc followed by an 8-aligned alloc: the second must be padded to 8-byte alignment.
		let a = (&arena).allocate(Layout::from_size_align(1, 1).unwrap()).unwrap();
		assert_eq!(a.as_ptr() as *mut u8 as usize - base, 0);
		let b = (&arena).allocate(Layout::from_size_align(8, 8).unwrap()).unwrap();
		assert_eq!((b.as_ptr() as *mut u8 as usize - base) % 8, 0);
		assert!(arena.used_bytes() >= 16);
	}

	#[test]
	fn zero_sized_allocation_points_into_the_region() {
		let arena = Arena::new();
		let a = (&arena).allocate(Layout::from_size_align(0, 4).unwrap()).unwrap();
		assert_eq!(a.len(), 0);
		// A zero-sized allocation must still be an aligned interior pointer, so its low 32 bits are a valid offset.
		let offset = a.as_ptr() as *mut u8 as usize - arena.base_ptr().as_ptr() as usize;
		assert_eq!(offset, 0);
		assert_eq!(a.as_ptr() as *mut u8 as usize % 4, 0);
		if vm::RESERVABLE {
			assert_eq!(a.as_ptr() as *mut u8 as usize as u32 as usize, offset);
		}
	}

	#[test]
	fn growing_a_zero_sized_allocation_stays_empty() {
		let arena = Arena::new();
		let zst = Layout::from_size_align(0, 1).unwrap();
		let a = (&arena).allocate(zst).unwrap();
		// Growing zero bytes to zero bytes must not touch the cursor nor dereference the empty allocation.
		let b = unsafe { (&arena).grow(a.cast::<u8>(), zst, zst) }.unwrap();
		assert_eq!(b.len(), 0);
		assert_eq!(arena.used_bytes(), 0);
		// Growing from zero bytes to a real allocation copies nothing and bumps from the region.
		let c = unsafe { (&arena).grow(b.cast::<u8>(), zst, Layout::from_size_align(32, 8).unwrap()) }.unwrap();
		assert_eq!(c.len(), 32);
		assert_eq!(c.as_ptr() as *mut u8 as usize, arena.base_ptr().as_ptr() as usize);
	}

	#[test]
	fn from_raw_parts_over_borrowed_memory_never_frees() {
		let backing = Arena::with_capacity(8192);
		let borrowed = unsafe { Arena::from_raw_parts(backing.base_ptr(), 4096) };
		let a = (&borrowed).allocate(Layout::from_size_align(256, 1).unwrap()).unwrap();
		assert_eq!(a.len(), 256);
		// Borrowed memory is exactly as big as the caller says, so the arena must not grow past it.
		assert!((&borrowed).allocate(Layout::from_size_align(8192, 1).unwrap()).is_err());
		drop(borrowed);
		// `backing` owns the memory, so dropping the borrow left it alone: it still hands out bytes, and they are still
		// writable.
		assert_eq!(backing.capacity(), 8192);
		let b = (&backing).allocate(Layout::from_size_align(8192, 1).unwrap()).unwrap();
		// SAFETY: the allocation is live and exclusively owned here.
		unsafe { b.cast::<u8>().write_bytes(0x44, 8192) };
	}

	#[test]
	fn tiny_alloc_commits_little_physical_memory() {
		if !vm::RESERVABLE {
			return;
		}
		let arena = Arena::new();
		let _ = (&arena).allocate(Layout::new::<u64>()).unwrap();
		assert!(arena.used_bytes() < 4096, "tiny alloc used {} bytes", arena.used_bytes());
		assert!(arena.capacity() >= MAX_BLOCK_SIZE - 16);
	}

	#[test]
	fn many_live_arenas_reserve_without_exhausting_memory() {
		// Reserving must not charge real memory (Windows has no overcommit), so a test binary's worth of concurrently
		// live full-size arenas has to fit.
		let arenas: Vec<Arena> = (0..64).map(|_| Arena::new()).collect();
		for arena in &arenas {
			let a = (&arena).allocate(Layout::from_size_align(64, 8).unwrap()).unwrap();
			// SAFETY: 64 bytes were just handed out for exclusive use.
			unsafe { a.cast::<u8>().write_bytes(0xAB, 64) };
		}
		assert_eq!(arenas.len(), 64);
	}

	#[test]
	fn allocation_spanning_many_pages_is_usable() {
		let arena = Arena::with_capacity(8 * 1024 * 1024);
		let layout = Layout::from_size_align(4 * 1024 * 1024, 8).unwrap();
		let a = (&arena).allocate(layout).unwrap();
		// SAFETY: the allocation is live and exclusively owned here.
		unsafe { a.cast::<u8>().write_bytes(0xCD, layout.size()) };
		// SAFETY: ditto; every byte was just initialised.
		let bytes = unsafe { a.as_ref() };
		assert!(bytes.iter().all(|b| *b == 0xCD));
	}

	#[test]
	fn exhaustion_returns_alloc_error() {
		let arena = Arena::with_capacity(128);
		// First alloc fits.
		assert!((&arena).allocate(Layout::from_size_align(64, 1).unwrap()).is_ok());
		// Second overflows the 128-byte region, which was asked for a fixed size and so cannot grow.
		assert!((&arena).allocate(Layout::from_size_align(128, 1).unwrap()).is_err());
	}

	#[test]
	fn an_initial_capacity_sets_the_first_chunk_size_but_not_a_limit() {
		let arena = Arena::with_initial_capacity(128);
		assert_eq!(arena.capacity(), 128, "the first chunk has the requested size");
		assert!((&arena).allocate(Layout::from_size_align(64, 1).unwrap()).is_ok());
		assert!((&arena).allocate(Layout::from_size_align(128, 1).unwrap()).is_ok());
		assert!(arena.capacity() > 128, "the arena added a chunk");
		assert_eq!(arena.used_bytes(), 192, "the count includes the bytes in the full chunk");
	}

	#[test]
	fn growing_the_last_allocation_extends_it_in_place() {
		let arena = Arena::new();
		let old = Layout::from_size_align(64, 8).unwrap();
		let a = (&arena).allocate(old).unwrap();
		let new = Layout::from_size_align(4096, 8).unwrap();
		let b = unsafe { (&arena).grow(a.cast::<u8>(), old, new) }.unwrap();
		assert_eq!(a.as_ptr() as *mut u8, b.as_ptr() as *mut u8, "the last allocation grows without moving");
		assert_eq!(arena.used_bytes(), 4096, "growing in place strands nothing");
	}

	#[test]
	fn a_growing_arena_adds_chunks_and_keeps_counting() {
		let arena = growing(1024);
		let layout = Layout::from_size_align(256, 8).unwrap();
		let allocs: Vec<_> = (0..64)
			.map(|_| {
				let a = (&arena).allocate(layout).unwrap();
				// SAFETY: the allocation is live and exclusively owned here.
				unsafe { a.cast::<u8>().write_bytes(0xEE, layout.size()) };
				a
			})
			.collect();
		assert!(arena.capacity() > 1024, "the arena outgrew its first chunk");
		assert_eq!(arena.used_bytes(), 64 * 256, "allocations in retired chunks are still counted");
		if cfg!(target_pointer_width = "64") {
			assert_eq!(arena.transfer_base(), None, "more than one chunk leaves nothing for offsets to be relative to");
		} else {
			assert_eq!(arena.transfer_base(), Some(0), "a 32 bit pointer is its own offset whatever the chunking");
		}
		// Every allocation from every chunk is still live and holds what was written to it.
		for a in &allocs {
			// SAFETY: each allocation is live and every byte was initialised above.
			assert!(unsafe { a.as_ref() }.iter().all(|b| *b == 0xEE));
		}
	}

	#[test]
	fn a_growing_arena_serves_an_allocation_larger_than_a_chunk() {
		let arena = growing(1024);
		let layout = Layout::from_size_align(3 * 1024 * 1024, 8).unwrap();
		let a = (&arena).allocate(layout).unwrap();
		// SAFETY: the allocation is live and exclusively owned here.
		unsafe { a.cast::<u8>().write_bytes(0x11, layout.size()) };
		assert!(arena.capacity() >= layout.size());
	}

	#[test]
	fn growth_stops_at_the_offset_budget() {
		let arena = growing(1024);
		// A request that cannot fit within MAX_BLOCK_SIZE has to fail rather than hand out an offset too big for a u32.
		assert!((&arena).allocate(Layout::from_size_align(MAX_BLOCK_SIZE, 8).unwrap()).is_err());
		// The arena is untouched and still usable.
		assert!((&arena).allocate(Layout::from_size_align(64, 8).unwrap()).is_ok());
	}

	#[test]
	fn reset_rewinds_to_the_first_chunk() {
		let mut arena = growing(1024);
		let layout = Layout::from_size_align(256, 8).unwrap();
		for _ in 0..64 {
			let _ = (&arena).allocate(layout).unwrap();
		}
		assert!(arena.capacity() > 1024, "the arena outgrew its first chunk");
		arena.reset();
		assert_eq!(arena.used_bytes(), 0);
		assert_eq!(arena.capacity(), 1024, "chunks added since construction are freed");
		// Still usable, from the top of the first chunk.
		let a = (&arena).allocate(layout).unwrap();
		assert_eq!(a.as_ptr() as *mut u8, arena.base_ptr().as_ptr());
	}

	#[test]
	fn reset_reuses_the_backing_region() {
		let mut arena = Arena::with_capacity(2 * 1024 * 1024);
		let base = arena.base_ptr();
		let layout = Layout::from_size_align(1 << 20, 8).unwrap();
		let a = (&arena).allocate(layout).unwrap();
		// SAFETY: the allocation is live and exclusively owned here.
		unsafe { a.cast::<u8>().write_bytes(0x22, layout.size()) };
		arena.reset();
		assert_eq!(arena.used_bytes(), 0);
		assert_eq!(arena.base_ptr(), base, "reset keeps the backing region");
		// The bytes are handed out again, and are still writable: a committed page stays committed.
		let b = (&arena).allocate(layout).unwrap();
		assert_eq!(b.as_ptr() as *mut u8, base.as_ptr());
		// SAFETY: the allocation is live and exclusively owned here.
		unsafe { b.cast::<u8>().write_bytes(0x33, layout.size()) };
	}
}
