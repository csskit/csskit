//! Virtual memory reservation for the [`Arena`][crate::Arena] region.
//!
//! The region is reserved at its full size up front, because a chunk can never grow. Reserving it through the system
//! allocator would ask for real memory: Windows has no overcommit and charges every byte of a `HeapAlloc` against the
//! commit limit, so a handful of live arenas exhausts memory. Reserving address space directly costs nothing until the
//! pages are touched.
//!
//! Reserving needs a 64 bit address space (a 4 GiB alignment does not fit in a 32 bit pointer) and the unix or windows
//! APIs. Where either is missing (wasm32 above all) [`reserve`] always fails and the arena falls back to chunks from
//! the global allocator, which it has to grow into as it fills.
use std::ptr::NonNull;

/// Whether this target can reserve address space lazily, and so hold a whole arena in one chunk.
pub(crate) const RESERVABLE: bool = cfg!(all(target_pointer_width = "64", any(unix, windows)));

/// A reserved region of address space, plus the 4 GiB-aligned window inside it that the arena allocates from.
#[derive(Clone, Copy)]
pub(crate) struct Reservation {
	/// Start of the whole reservation, needed to hand the address space back.
	pub(crate) reservation: NonNull<u8>,
	/// Length of the whole reservation, needed to hand the address space back.
	pub(crate) reserved: usize,
	/// The 4 GiB-aligned base of the usable region.
	pub(crate) base: NonNull<u8>,
	/// Usable bytes at [`Reservation::base`], which is what was asked for.
	pub(crate) size: usize,
}

/// Only the reserving implementations round anything up.
#[cfg(all(target_pointer_width = "64", any(unix, windows)))]
#[inline]
fn align_up(addr: usize, align: usize) -> usize {
	(addr + align - 1) & !(align - 1)
}

#[cfg(all(target_pointer_width = "64", unix))]
mod imp {
	use super::{Reservation, align_up};
	use crate::BLOCK_ALIGN;
	use std::ptr::{self, NonNull};

	/// Linux accounts for mappings up front unless told the pages may never be touched.
	#[cfg(any(target_os = "android", target_os = "linux"))]
	const NORESERVE: libc::c_int = libc::MAP_NORESERVE;
	#[cfg(not(any(target_os = "android", target_os = "linux")))]
	const NORESERVE: libc::c_int = 0;

	fn page_size() -> usize {
		// SAFETY: `sysconf` takes no pointers and `_SC_PAGESIZE` is always supported.
		unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
	}

	/// Map `size` bytes of lazily-faulted address space at a 4 GiB-aligned base.
	pub(crate) fn reserve(size: usize) -> Option<Reservation> {
		let len = align_up(size, page_size());
		// Over-map by the alignment so a 4 GiB boundary is guaranteed to fall within the mapping.
		let over = len.checked_add(BLOCK_ALIGN)?;
		// SAFETY: a null hint lets the kernel place the mapping; anonymous mappings ignore the fd and offset.
		let ptr = unsafe {
			libc::mmap(
				ptr::null_mut(),
				over,
				libc::PROT_READ | libc::PROT_WRITE,
				libc::MAP_PRIVATE | libc::MAP_ANON | NORESERVE,
				-1,
				0,
			)
		};
		if ptr == libc::MAP_FAILED {
			return None;
		}
		let addr = ptr as usize;
		let aligned = align_up(addr, BLOCK_ALIGN);
		// Give back the slack either side, so the reservation is exactly the usable region.
		let head = aligned - addr;
		if head > 0 {
			// SAFETY: `head` bytes at `ptr` are page aligned and part of the mapping made above.
			unsafe { libc::munmap(ptr, head) };
		}
		let tail = over - head - len;
		if tail > 0 {
			// SAFETY: `aligned + len` is page aligned and the remaining bytes are part of the same mapping.
			unsafe { libc::munmap((aligned + len) as *mut libc::c_void, tail) };
		}
		// SAFETY: `mmap` succeeded, so `aligned` is a live, non-null address within the mapping.
		let base = unsafe { NonNull::new_unchecked(aligned as *mut u8) };
		// `size` is what was asked for, and so what the pool keeps the reservation under; `len` is the page-rounded
		// mapping, which is what `munmap` needs.
		Some(Reservation { reservation: base, reserved: len, base, size })
	}

	/// Tell the kernel the first `len` bytes at `base` hold nothing worth keeping.
	///
	/// The pages stay mapped and stay usable, so the next write to one costs no fault, but the kernel may take them
	/// back under memory pressure. Where `MADV_FREE` is missing the pages go back at once instead, which costs a fault
	/// to touch again but is still correct.
	///
	/// # Safety
	/// The range must lie within a live reservation, and nothing allocated from it may still be in use.
	pub(crate) unsafe fn discard(base: NonNull<u8>, len: usize) {
		#[cfg(any(target_os = "android", target_os = "linux", target_os = "macos", target_os = "freebsd"))]
		let advice = libc::MADV_FREE;
		#[cfg(not(any(target_os = "android", target_os = "linux", target_os = "macos", target_os = "freebsd")))]
		let advice = libc::MADV_DONTNEED;
		// SAFETY: the caller guarantees the range is a live mapping whose contents are dead.
		unsafe { libc::madvise(base.as_ptr().cast(), len, advice) };
	}

	/// # Safety
	/// `reservation` and `reserved` must be exactly what [`reserve`] returned, and no allocation may outlive the call.
	pub(crate) unsafe fn release(reservation: NonNull<u8>, reserved: usize) {
		// SAFETY: the caller guarantees this is the mapping `reserve` kept.
		unsafe { libc::munmap(reservation.as_ptr().cast(), reserved) };
	}
}

#[cfg(all(target_pointer_width = "64", windows))]
mod imp {
	use super::{Reservation, align_up};
	use crate::BLOCK_ALIGN;
	use std::ptr::{self, NonNull};
	use windows_sys::Win32::System::Memory::{
		MEM_COMMIT, MEM_DECOMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, VirtualAlloc, VirtualFree,
	};

	/// Reserve `size` bytes of address space at a 4 GiB-aligned base, committing none of it.
	pub(crate) fn reserve(size: usize) -> Option<Reservation> {
		// Windows cannot release part of a reservation, so the alignment slack is kept for the arena's lifetime. It is
		// address space only: `MEM_RESERVE` is not charged against the commit limit.
		let over = size.checked_add(BLOCK_ALIGN)?;
		// SAFETY: a null base lets the OS place the reservation.
		let ptr = unsafe { VirtualAlloc(ptr::null(), over, MEM_RESERVE, PAGE_READWRITE) };
		let reservation = NonNull::new(ptr.cast::<u8>())?;
		let aligned = align_up(ptr as usize, BLOCK_ALIGN);
		// SAFETY: the reservation spans `size + BLOCK_ALIGN` bytes, so `aligned` and the `size` bytes after it are
		// inside it.
		let base = unsafe { NonNull::new_unchecked(aligned as *mut u8) };
		Some(Reservation { reservation, reserved: over, base, size })
	}

	/// Back `len` bytes at `ptr` with physical memory. Committing an already-committed page is a no-op.
	///
	/// # Safety
	/// The range must lie within a reservation made by [`reserve`].
	pub(crate) unsafe fn commit(ptr: NonNull<u8>, len: usize) -> bool {
		// SAFETY: the caller guarantees the range is inside the reservation.
		!unsafe { VirtualAlloc(ptr.as_ptr().cast(), len, MEM_COMMIT, PAGE_READWRITE) }.is_null()
	}

	/// Hand the physical memory behind the first `len` bytes at `base` back, leaving the address space reserved.
	///
	/// The next write to one of those pages commits it again, which [`commit`] does as the arena bumps.
	///
	/// # Safety
	/// The range must lie within a live reservation, and nothing allocated from it may still be in use.
	pub(crate) unsafe fn discard(base: NonNull<u8>, len: usize) {
		// SAFETY: the caller guarantees the range is inside a live reservation whose contents are dead.
		unsafe { VirtualFree(base.as_ptr().cast(), len, MEM_DECOMMIT) };
	}

	/// # Safety
	/// `reservation` must be exactly what [`reserve`] returned, and no allocation may outlive the call.
	pub(crate) unsafe fn release(reservation: NonNull<u8>, _reserved: usize) {
		// SAFETY: `MEM_RELEASE` requires the base of the reservation and a zero size.
		unsafe { VirtualFree(reservation.as_ptr().cast(), 0, MEM_RELEASE) };
	}
}

/// No lazy reservation here, so [`reserve`] always fails and [`Arena`][crate::Arena] allocates its chunks from the
/// global allocator instead. The rest exists only so its callers need no `cfg` of their own; none of it runs.
#[cfg(not(all(target_pointer_width = "64", any(unix, windows))))]
mod imp {
	use super::Reservation;
	use std::ptr::NonNull;

	pub(crate) fn reserve(_size: usize) -> Option<Reservation> {
		None
	}

	/// # Safety
	/// Never call this: [`reserve`] never succeeds here, so there is nothing to commit.
	#[cfg(windows)]
	pub(crate) unsafe fn commit(_ptr: NonNull<u8>, _len: usize) -> bool {
		false
	}

	/// # Safety
	/// Never call this: [`reserve`] never succeeds here, so there is nothing to discard.
	pub(crate) unsafe fn discard(_base: NonNull<u8>, _len: usize) {}

	/// # Safety
	/// Never call this: [`reserve`] never succeeds here, so there is nothing to release.
	pub(crate) unsafe fn release(_reservation: NonNull<u8>, _reserved: usize) {}
}

#[cfg(windows)]
pub(crate) use imp::commit;
pub(crate) use imp::{discard, release, reserve};
