//! Virtual memory reservation for the [`Arena`][crate::Arena] region.
//!
//! The region is reserved at its full size up front, because a single-chunk arena can never grow. Reserving it through
//! the system allocator would ask for real memory: Windows has no overcommit and charges every byte of a `HeapAlloc`
//! against the commit limit, so a handful of live arenas exhausts memory. Reserving address space directly costs
//! nothing until the pages are touched.
use std::ptr::NonNull;

/// A reserved region of address space, plus the 4 GiB-aligned window inside it that the arena allocates from.
pub(crate) struct Reservation {
	/// Start of the whole reservation, needed to hand the address space back.
	pub(crate) reservation: NonNull<u8>,
	/// Length of the whole reservation, needed to hand the address space back.
	pub(crate) reserved: usize,
	/// The 4 GiB-aligned base of the usable region.
	pub(crate) base: NonNull<u8>,
}

#[inline]
fn align_up(addr: usize, align: usize) -> usize {
	(addr + align - 1) & !(align - 1)
}

#[cfg(unix)]
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
		Some(Reservation { reservation: base, reserved: len, base })
	}

	/// # Safety
	/// `reservation` and `reserved` must be exactly what [`reserve`] returned, and no allocation may outlive the call.
	pub(crate) unsafe fn release(reservation: NonNull<u8>, reserved: usize) {
		// SAFETY: the caller guarantees this is the mapping `reserve` kept.
		unsafe { libc::munmap(reservation.as_ptr().cast(), reserved) };
	}
}

#[cfg(windows)]
mod imp {
	use super::{Reservation, align_up};
	use crate::BLOCK_ALIGN;
	use std::ptr::{self, NonNull};
	use windows_sys::Win32::System::Memory::{
		MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, VirtualAlloc, VirtualFree,
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
		Some(Reservation { reservation, reserved: over, base })
	}

	/// Back `len` bytes at `ptr` with physical memory. Committing an already-committed page is a no-op.
	///
	/// # Safety
	/// The range must lie within a reservation made by [`reserve`].
	pub(crate) unsafe fn commit(ptr: NonNull<u8>, len: usize) -> bool {
		// SAFETY: the caller guarantees the range is inside the reservation.
		!unsafe { VirtualAlloc(ptr.as_ptr().cast(), len, MEM_COMMIT, PAGE_READWRITE) }.is_null()
	}

	/// # Safety
	/// `reservation` must be exactly what [`reserve`] returned, and no allocation may outlive the call.
	pub(crate) unsafe fn release(reservation: NonNull<u8>, _reserved: usize) {
		// SAFETY: `MEM_RELEASE` requires the base of the reservation and a zero size.
		unsafe { VirtualFree(reservation.as_ptr().cast(), 0, MEM_RELEASE) };
	}
}

#[cfg(windows)]
pub(crate) use imp::commit;
pub(crate) use imp::{release, reserve};
