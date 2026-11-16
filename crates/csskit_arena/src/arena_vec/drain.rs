use super::Vec;
use crate::Arena;
use std::alloc::Allocator;
use std::fmt;
use std::iter::FusedIterator;
use std::mem::{self, ManuallyDrop};
use std::ptr::{self, NonNull};
use std::slice;

/// A draining iterator for [`Vec`], created by [`Vec::drain`].
pub struct Drain<'v, 'a, T, A: Allocator = &'a Arena> {
	/// Index of the tail to preserve.
	pub(super) tail_start: usize,
	/// Length of the tail.
	pub(super) tail_len: usize,
	/// The elements of the drained range not yet yielded.
	pub(super) iter: slice::Iter<'v, T>,
	pub(super) vec: &'v mut Vec<'a, T, A>,
}

impl<'v, 'a, T: fmt::Debug, A: Allocator> fmt::Debug for Drain<'v, 'a, T, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("Drain").field(&self.iter.as_slice()).finish()
	}
}

impl<'v, 'a, T, A: Allocator> Drain<'v, 'a, T, A> {
	/// The elements not yet yielded, as a slice.
	#[inline]
	pub fn as_slice(&self) -> &[T] {
		self.iter.as_slice()
	}

	/// The allocator backing the source vector.
	#[inline]
	pub fn allocator(&self) -> &A {
		self.vec.allocator()
	}

	/// Stop draining, keeping the elements not yet yielded in the source vector.
	pub fn keep_rest(self) {
		let mut this = ManuallyDrop::new(self);
		let start = this.vec.len();
		let tail = this.tail_start;
		let tail_len = this.tail_len;
		let unyielded_len = this.iter.len();
		let unyielded_ptr = this.iter.as_slice().as_ptr();
		// SAFETY: the un-yielded elements and the tail are live and lie at or after the vector's length, so moving them down in order stays in bounds.
		unsafe {
			if size_of::<T>() != 0 {
				let start_ptr = this.vec.as_mut_ptr().add(start);
				if unyielded_ptr != start_ptr {
					ptr::copy(unyielded_ptr, start_ptr, unyielded_len);
				}
				if tail != start + unyielded_len {
					ptr::copy(this.vec.as_ptr().add(tail), start_ptr.add(unyielded_len), tail_len);
				}
			}
			this.vec.set_len(start + unyielded_len + tail_len);
		}
	}
}

impl<'v, 'a, T, A: Allocator> AsRef<[T]> for Drain<'v, 'a, T, A> {
	fn as_ref(&self) -> &[T] {
		self.as_slice()
	}
}

impl<'v, 'a, T, A: Allocator> Iterator for Drain<'v, 'a, T, A> {
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<T> {
		// SAFETY: each element of the drained range is yielded at most once and is no longer counted by the vector.
		self.iter.next().map(|element| unsafe { ptr::read(element) })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}

impl<'v, 'a, T, A: Allocator> DoubleEndedIterator for Drain<'v, 'a, T, A> {
	#[inline]
	fn next_back(&mut self) -> Option<T> {
		// SAFETY: as in `next`.
		self.iter.next_back().map(|element| unsafe { ptr::read(element) })
	}
}

impl<'v, 'a, T, A: Allocator> Drop for Drain<'v, 'a, T, A> {
	fn drop(&mut self) {
		struct DropGuard<'r, 'v, 'a, T, A: Allocator>(&'r mut Drain<'v, 'a, T, A>);

		impl<'r, 'v, 'a, T, A: Allocator> Drop for DropGuard<'r, 'v, 'a, T, A> {
			fn drop(&mut self) {
				let drain = &mut *self.0;
				if drain.tail_len > 0 {
					let start = drain.vec.len();
					let tail = drain.tail_start;
					// SAFETY: the tail is live at `tail_start` and the vector's length marks the end of the kept prefix, so moving the tail down closes the gap.
					unsafe {
						if tail != start {
							ptr::copy(drain.vec.as_ptr().add(tail), drain.vec.as_mut_ptr().add(start), drain.tail_len);
						}
						drain.vec.set_len(start + drain.tail_len);
					}
				}
			}
		}

		let iter = mem::take(&mut self.iter);
		let drop_len = iter.len();

		if size_of::<T>() == 0 {
			// SAFETY: zero-sized values occupy no storage, so the tail needs no move and any aligned pointer addresses the remaining values.
			unsafe {
				self.vec.set_len(self.vec.len() + self.tail_len);
				ptr::drop_in_place(ptr::slice_from_raw_parts_mut(NonNull::<T>::dangling().as_ptr(), drop_len));
			}
			return;
		}

		let vec_ptr = self.vec.as_mut_ptr();
		let drop_ptr = iter.as_slice().as_ptr();
		let _guard = DropGuard(self);
		if drop_len == 0 {
			return;
		}
		// SAFETY: `drop_ptr..drop_ptr + drop_len` lies inside the vector's buffer and holds un-yielded live elements.
		unsafe {
			let drop_offset = drop_ptr.offset_from_unsigned(vec_ptr);
			ptr::drop_in_place(ptr::slice_from_raw_parts_mut(vec_ptr.add(drop_offset), drop_len));
		}
	}
}

impl<'v, 'a, T, A: Allocator> ExactSizeIterator for Drain<'v, 'a, T, A> {}

impl<'v, 'a, T, A: Allocator> FusedIterator for Drain<'v, 'a, T, A> {}
