use crate::arena_vec::TryReserveError;
use std::alloc::{Allocator, Layout};
use std::ptr::NonNull;

/// The raw view of a growable slice: a native pointer, a length, and a capacity, with a stable repr(C) layout.
///
/// `len` and `cap` are counts of `T`, not bytes.
#[repr(C)]
pub(crate) struct RawVec<T> {
	pub(crate) ptr: NonNull<T>,
	pub(crate) len: u32,
	pub(crate) cap: u32,
}

impl<T> RawVec<T> {
	/// An empty `RawVec` with a dangling pointer; allocates nothing.
	#[inline]
	pub(crate) const fn new() -> Self {
		Self { ptr: NonNull::dangling(), len: 0, cap: 0 }
	}

	#[inline]
	fn layout(cap: u32) -> Layout {
		Layout::array::<T>(cap as usize).expect("Vec capacity overflow")
	}

	#[inline]
	fn needs_to_grow(&self, len: usize, additional: usize) -> bool {
		additional > (self.cap as usize).wrapping_sub(len)
	}

	/// Ensure room for `len + additional` elements, at least doubling the capacity when it grows.
	#[inline]
	pub(crate) fn reserve<A: Allocator>(&mut self, len: usize, additional: usize, alloc: &A) {
		if let Err(error) = self.try_reserve(len, additional, alloc) {
			error.raise();
		}
	}

	/// Grow by at least one element, for a `push` into a full buffer.
	#[inline(never)]
	pub(crate) fn grow_one<A: Allocator>(&mut self, alloc: &A) {
		if let Err(error) = self.grow_amortized(self.cap as usize, 1, alloc) {
			error.raise();
		}
	}

	pub(crate) fn try_reserve<A: Allocator>(
		&mut self,
		len: usize,
		additional: usize,
		alloc: &A,
	) -> Result<(), TryReserveError> {
		if self.needs_to_grow(len, additional) { self.grow_amortized(len, additional, alloc) } else { Ok(()) }
	}

	pub(crate) fn reserve_exact<A: Allocator>(&mut self, len: usize, additional: usize, alloc: &A) {
		if let Err(error) = self.try_reserve_exact(len, additional, alloc) {
			error.raise();
		}
	}

	pub(crate) fn try_reserve_exact<A: Allocator>(
		&mut self,
		len: usize,
		additional: usize,
		alloc: &A,
	) -> Result<(), TryReserveError> {
		if self.needs_to_grow(len, additional) { self.grow_exact(len, additional, alloc) } else { Ok(()) }
	}

	/// Shrink the backing allocation to hold exactly `cap` elements, which must be at least `len`.
	pub(crate) fn shrink_to_fit<A: Allocator>(&mut self, cap: u32, alloc: &A) {
		debug_assert!(cap <= self.cap && cap >= self.len, "shrink must reduce capacity without losing elements");
		if cap == self.cap {
			return;
		}
		// SAFETY: `ptr` was allocated by `alloc` with the layout of `self.cap` elements, and the new layout is smaller.
		let new_ptr = unsafe { alloc.shrink(self.ptr.cast::<u8>(), Self::layout(self.cap), Self::layout(cap)) }
			.unwrap_or_else(|_| TryReserveError::AllocError.raise());
		self.ptr = new_ptr.cast::<T>();
		self.cap = cap;
	}

	fn required(len: usize, additional: usize) -> Result<u32, TryReserveError> {
		len.checked_add(additional)
			.and_then(|required| u32::try_from(required).ok())
			.ok_or(TryReserveError::CapacityOverflow)
	}

	fn grow_amortized<A: Allocator>(
		&mut self,
		len: usize,
		additional: usize,
		alloc: &A,
	) -> Result<(), TryReserveError> {
		let required = Self::required(len, additional)?;
		self.finish_grow(required.max(self.cap.saturating_mul(2)), alloc)
	}

	fn grow_exact<A: Allocator>(&mut self, len: usize, additional: usize, alloc: &A) -> Result<(), TryReserveError> {
		let required = Self::required(len, additional)?;
		self.finish_grow(required, alloc)
	}

	/// Uses [`Allocator::grow`] so a bump allocator can extend the allocation in place when this is the most recent
	/// allocation (avoiding stranding the old buffer); otherwise it falls back to a fresh allocation plus a copy.
	#[inline(never)]
	fn finish_grow<A: Allocator>(&mut self, cap: u32, alloc: &A) -> Result<(), TryReserveError> {
		debug_assert!(cap > self.cap, "grow must strictly increase capacity");
		let new_layout = Layout::array::<T>(cap as usize).map_err(|_| TryReserveError::CapacityOverflow)?;
		let new_ptr = if self.cap == 0 {
			alloc.allocate(new_layout)
		} else {
			// SAFETY: `ptr` was allocated by `alloc` with the layout of `self.cap` elements, and the new layout is larger.
			unsafe { alloc.grow(self.ptr.cast::<u8>(), Self::layout(self.cap), new_layout) }
		}
		.map_err(|_| TryReserveError::AllocError)?;
		self.ptr = new_ptr.cast::<T>();
		self.cap = cap;
		Ok(())
	}
}
