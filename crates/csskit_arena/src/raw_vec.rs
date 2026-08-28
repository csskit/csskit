use allocator_api2::alloc::{Allocator, Layout};
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

	/// Grow the backing allocation to hold at least `required` elements.
	///
	/// Uses [`Allocator::grow`] so a bump allocator can extend the allocation in place when this is the most recent
	/// allocation (avoiding stranding the old buffer); otherwise it falls back to a fresh allocation plus a copy.
	pub(crate) fn grow<A: Allocator>(&mut self, required: u32, alloc: &A) {
		debug_assert!(required > self.cap, "grow can only be called when more capacity is needed");
		let new_cap = required.max(self.cap * 2);
		debug_assert!(new_cap >= required && new_cap > self.cap, "grow must strictly increase capacity");
		let new_layout = Self::layout(new_cap);
		let new_ptr = if self.cap == 0 {
			alloc.allocate(new_layout).expect("arena exhausted").cast::<T>()
		} else {
			let old_layout = Self::layout(self.cap);
			unsafe { alloc.grow(self.ptr.cast::<u8>(), old_layout, new_layout).expect("arena exhausted").cast::<T>() }
		};
		self.ptr = new_ptr;
		self.cap = new_cap;
	}
}
