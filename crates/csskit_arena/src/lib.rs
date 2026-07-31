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
//! [`Arena`] is a single-chunk bump allocator that satisfies both constraints. It never grows or relocates.
//!
//! # Ownership modes
//!
//! - [`Arena::new`] / [`Arena::default`]: self-allocated, full 2 GiB reservation.
//! - [`Arena::with_capacity`]: self-allocated, reserving a given size, useful for tests.
//! - [`Arena::from_raw_parts`]: borrows caller-owned memory (useful for e.g. NAPI where V8 owns the `ArrayBuffer`).
//!
//! All modes yield the same [`Arena`] type with identical layout and offset semantics.
use allocator_api2::alloc::{AllocError, Allocator};
use std::alloc::Layout;
use std::cell::Cell;
use std::ptr::NonNull;

mod vm;

#[cfg(not(target_pointer_width = "64"))]
compile_error!("csskit_arena requires a 64 bit target, as BLOCK_ALIGN does not fit in a smaller pointer");

#[cfg(not(any(unix, windows)))]
compile_error!("csskit_arena requires the virtual memory APIs of unix or windows");

/// Required alignment of the arena region (4 GiB), so that `ptr as u32` equals the byte offset
/// within the region.
pub const BLOCK_ALIGN: usize = 4 * 1024 * 1024 * 1024;

/// Maximum usable size of the arena region (just under 2 GiB), so that no allocation crosses a
/// 4 GiB boundary and every offset fits in a `u32`.
pub const MAX_BLOCK_SIZE: usize = 2 * 1024 * 1024 * 1024 - 16;

/// Bytes committed the first time a Windows arena is written to; growth doubles from there.
#[cfg(any(test, windows))]
const INITIAL_COMMIT: usize = 64 * 1024;

/// How the arena's backing memory was obtained, which determines what (if anything) happens on drop.
enum Backing {
	/// Address space reserved by this crate. `reservation` and `reserved` are the base and length of the whole
	/// reservation, both of which are needed to release it.
	Owned { reservation: NonNull<u8>, reserved: usize },
	/// Memory owned by the caller (e.g. a V8 `ArrayBuffer`). Never freed by the arena.
	Borrowed,
}

/// A single-chunk Allocator.
///
/// The region begins at a 4 GiB-aligned address and is at most [`MAX_BLOCK_SIZE`] bytes, so the low 32 bits of any
/// interior pointer equal its byte offset within the region. Allocation is a pointer bump; deallocation of individual
/// items is a no-op (the whole region is freed at once on drop; or never if using borrowed memory).
pub struct Arena {
	/// Base of the usable, 4 GiB-aligned region.
	base: NonNull<u8>,
	/// Usable size of the region in bytes.
	size: usize,
	/// Bump cursor: byte offset of the next free position within the region.
	cursor: Cell<usize>,
	/// Bytes of the region backed by physical memory. Windows has no lazy commit, so the arena commits as it bumps.
	#[cfg(windows)]
	committed: Cell<usize>,
	backing: Backing,
}

impl Arena {
	/// Create a self-allocated arena.
	///
	/// The region reserves the full [`MAX_BLOCK_SIZE`] address space so that a single non-growing bump arena can serve
	/// arbitrarily large parses without exhausting (the arena never relocates, so it cannot grow after construction).
	#[inline]
	pub fn new() -> Self {
		Self::with_capacity(MAX_BLOCK_SIZE)
	}

	/// Create a self-allocated arena reserving `size` usable bytes (clamped to [`MAX_BLOCK_SIZE`]).
	///
	/// # Panics
	/// Panics if the backing allocation fails.
	pub fn with_capacity(size: usize) -> Self {
		let size = size.clamp(1, MAX_BLOCK_SIZE);
		let reservation = vm::reserve(size).expect("arena backing reservation failed");
		debug_assert_eq!(reservation.base.as_ptr() as usize % BLOCK_ALIGN, 0, "arena base must be 4 GiB aligned");
		Self {
			base: reservation.base,
			size,
			cursor: Cell::new(0),
			#[cfg(windows)]
			committed: Cell::new(0),
			backing: Backing::Owned { reservation: reservation.reservation, reserved: reservation.reserved },
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
		debug_assert_eq!(ptr.as_ptr() as usize % BLOCK_ALIGN, 0, "borrowed arena base must be 4 GiB aligned");
		debug_assert!(size <= MAX_BLOCK_SIZE, "borrowed arena must not exceed MAX_BLOCK_SIZE");
		Self {
			base: ptr,
			size,
			cursor: Cell::new(0),
			// Borrowed memory is already backed by its owner.
			#[cfg(windows)]
			committed: Cell::new(size),
			backing: Backing::Borrowed,
		}
	}

	/// The base address of the usable region.
	#[inline]
	pub fn base_ptr(&self) -> NonNull<u8> {
		self.base
	}

	/// Number of bytes handed out so far.
	#[inline]
	pub fn used_bytes(&self) -> usize {
		self.cursor.get()
	}

	/// Total usable capacity of the region in bytes.
	#[inline]
	pub fn capacity(&self) -> usize {
		self.size
	}

	/// Release every allocation at once by rewinding the bump cursor to the start of the region.
	///
	/// Takes `&mut self` so no allocation can outlive the reset. The backing memory is retained.
	#[inline]
	pub fn reset(&mut self) {
		self.cursor.set(0);
	}
}

/// How far to commit when `end` bytes are needed and `committed` are already backed, given a region of `size` bytes.
///
/// Growth doubles so that a parse does not pay a syscall per bump, but never overshoots the region nor undershoots
/// what was asked for. Only Windows commits, but the arithmetic is compiled everywhere so it can be tested everywhere.
#[inline]
#[cfg(any(test, windows))]
fn commit_target(end: usize, committed: usize, size: usize) -> usize {
	end.max(committed.saturating_mul(2)).max(INITIAL_COMMIT).min(size).max(end)
}

impl Arena {
	/// Ensure the region is backed by physical memory up to `end`, which must be within the region.
	#[cfg(windows)]
	#[cold]
	#[inline(never)]
	fn commit_to(&self, end: usize) -> bool {
		let committed = self.committed.get();
		let target = commit_target(end, committed, self.size);
		// SAFETY: `committed <= target <= size`, so the range lies within the reservation.
		let ptr = unsafe { NonNull::new_unchecked(self.base.as_ptr().add(committed)) };
		// SAFETY: ditto.
		if !unsafe { vm::commit(ptr, target - committed) } {
			return false;
		}
		self.committed.set(target);
		true
	}

	/// Bump-allocate `layout.size()` bytes aligned to `layout.align()`.
	///
	/// Returns `None` if the region is exhausted.
	#[inline]
	fn bump(&self, layout: Layout) -> Option<NonNull<[u8]>> {
		let align = layout.align();
		let bytes = layout.size();
		debug_assert!(align.is_power_of_two(), "Layout alignment is always a power of two");
		let start = self.cursor.get();
		debug_assert!(start <= self.size, "cursor must never exceed the region size");
		// Round the cursor up to the requested alignment.
		let aligned = (start + align - 1) & !(align - 1);
		let end = aligned.checked_add(bytes)?;
		if end > self.size {
			return None;
		}
		#[cfg(windows)]
		if end > self.committed.get() && !self.commit_to(end) {
			return None;
		}
		debug_assert!(aligned >= start && end >= aligned, "bump must advance the cursor monotonically");
		self.cursor.set(end);
		// SAFETY: `aligned + bytes <= size`, so the range is within the region.
		let ptr = unsafe { NonNull::new_unchecked(self.base.as_ptr().add(aligned)) };
		debug_assert_eq!(ptr.as_ptr() as usize % align, 0, "returned pointer must satisfy the requested alignment");
		Some(NonNull::slice_from_raw_parts(ptr, bytes))
	}

	/// A zero-length allocation at the cursor.
	///
	/// Nothing is written, so the cursor does not move and the pages need not be committed, but the pointer is still an
	/// interior pointer of the region: handing out an out-of-region dangling pointer would break raw transfer, whose
	/// offsets are the low 32 bits of every pointer.
	#[inline]
	fn empty(&self, align: usize) -> Option<NonNull<[u8]>> {
		let aligned = (self.cursor.get() + align - 1) & !(align - 1);
		if aligned > self.size {
			return None;
		}
		// SAFETY: `aligned <= size`, so the address is within the region.
		let ptr = unsafe { NonNull::new_unchecked(self.base.as_ptr().add(aligned)) };
		Some(NonNull::slice_from_raw_parts(ptr, 0))
	}
}

impl std::fmt::Debug for Arena {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Arena").field("capacity", &self.size).field("used", &self.cursor.get()).finish()
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
		if let Backing::Owned { reservation, reserved } = self.backing {
			// SAFETY: `reservation`/`reserved` are exactly what `vm::reserve` returned, and every allocation is dead.
			unsafe { vm::release(reservation, reserved) };
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
		// thing allocated.
		let addr = ptr.as_ptr() as usize;
		let base = self.base.as_ptr() as usize;
		debug_assert!(addr >= base, "grown pointer must lie within this arena's region");
		let offset = addr - base;
		debug_assert!(offset + old_layout.size() <= self.size, "grown allocation must lie within the region");
		if new_layout.align() <= old_layout.align()
			&& offset + old_layout.size() == self.cursor.get()
			&& offset + new_layout.size() <= self.size
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
	use crate::{Arena, BLOCK_ALIGN, INITIAL_COMMIT, MAX_BLOCK_SIZE, commit_target};
	use allocator_api2::alloc::Allocator;
	use std::alloc::Layout;

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
		let arena = Arena::new();
		assert_eq!(arena.base_ptr().as_ptr() as usize % BLOCK_ALIGN, 0);
		let fixed = Arena::new();
		assert_eq!(fixed.base_ptr().as_ptr() as usize % BLOCK_ALIGN, 0);
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
		assert_eq!(data as u32 as usize, offset, "ptr low 32 bits must equal arena offset");
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
		assert_eq!(a.as_ptr() as *mut u8 as usize as u32 as usize, offset);
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
		let backing = Arena::new();
		let borrowed = unsafe { Arena::from_raw_parts(backing.base_ptr(), 4096) };
		let a = (&borrowed).allocate(Layout::from_size_align(256, 1).unwrap()).unwrap();
		assert_eq!(a.len(), 256);
		drop(borrowed);
		// Still valid: backing owns the memory and is untouched.
		assert_eq!(backing.base_ptr().as_ptr() as usize % BLOCK_ALIGN, 0);
	}

	#[test]
	fn tiny_alloc_commits_little_physical_memory() {
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
		// Second overflows the 128-byte region.
		assert!((&arena).allocate(Layout::from_size_align(128, 1).unwrap()).is_err());
	}
}
