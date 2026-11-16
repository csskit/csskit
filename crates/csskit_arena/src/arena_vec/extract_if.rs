use super::{Vec, slice_range};
use crate::Arena;
use std::alloc::Allocator;
use std::fmt;
use std::ops::{Range, RangeBounds};
use std::ptr;

/// An iterator which uses a closure to determine if an element should be removed, created by [`Vec::extract_if`].
pub struct ExtractIf<'v, 'a, T, F, A: Allocator = &'a Arena> {
	vec: &'v mut Vec<'a, T, A>,
	/// The index of the item that will be inspected by the next call to `next`.
	index: usize,
	/// Elements at and beyond this point will be retained. Must be equal or smaller than `old_len`.
	end: usize,
	/// The number of items that have been drained (removed) thus far.
	deleted: usize,
	/// The original length of `vec` prior to draining.
	old_len: usize,
	/// The filter test predicate.
	predicate: F,
}

impl<'v, 'a, T, F, A: Allocator> ExtractIf<'v, 'a, T, F, A> {
	#[track_caller]
	pub(super) fn new<R: RangeBounds<usize>>(vec: &'v mut Vec<'a, T, A>, predicate: F, range: R) -> Self {
		let old_len = vec.len();
		let Range { start, end } = slice_range(range, old_len);
		// SAFETY: a zero length hides every element, so a leaked `ExtractIf` cannot expose moved-out values.
		unsafe { vec.set_len(0) };
		ExtractIf { vec, index: start, deleted: 0, end, old_len, predicate }
	}

	/// The allocator backing the source vector.
	#[inline]
	pub fn allocator(&self) -> &A {
		self.vec.allocator()
	}
}

impl<'v, 'a, T, F: FnMut(&mut T) -> bool, A: Allocator> Iterator for ExtractIf<'v, 'a, T, F, A> {
	type Item = T;

	fn next(&mut self) -> Option<T> {
		while self.index < self.end {
			let i = self.index;
			// SAFETY: `i < end <= old_len` is live and has not been moved; the length of 0 keeps it unaliased.
			let current = unsafe { &mut *self.vec.as_mut_ptr().add(i) };
			let drained = (self.predicate)(current);
			self.index += 1;
			if drained {
				self.deleted += 1;
				// SAFETY: the element is no longer counted as kept, so it is read out exactly once.
				return Some(unsafe { ptr::read(current) });
			} else if self.deleted > 0 {
				// SAFETY: `i - deleted` is a slot vacated by an extracted or shifted element.
				unsafe {
					let hole_slot = self.vec.as_mut_ptr().add(i - self.deleted);
					ptr::copy_nonoverlapping(current, hole_slot, 1);
				}
			}
		}
		None
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, Some(self.end - self.index))
	}
}

impl<'v, 'a, T, F, A: Allocator> Drop for ExtractIf<'v, 'a, T, F, A> {
	fn drop(&mut self) {
		if self.deleted > 0 {
			// SAFETY: `index..old_len` are live untested or tail elements; shifting them down by `deleted` closes the gap.
			unsafe {
				ptr::copy(
					self.vec.as_ptr().add(self.index),
					self.vec.as_mut_ptr().add(self.index - self.deleted),
					self.old_len - self.index,
				);
			}
		}
		// SAFETY: the first `old_len - deleted` slots now hold the kept elements.
		unsafe { self.vec.set_len(self.old_len - self.deleted) };
	}
}

impl<'v, 'a, T: fmt::Debug, F, A: Allocator> fmt::Debug for ExtractIf<'v, 'a, T, F, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let start = self.vec.as_ptr();
		// SAFETY: `0..index - deleted` are the kept elements and `index..old_len` are live untested or tail elements.
		let (retained, valid_tail) = unsafe {
			(
				std::slice::from_raw_parts(start, self.index - self.deleted),
				std::slice::from_raw_parts(start.add(self.index), self.old_len - self.index),
			)
		};
		let (remainder, skipped_tail) = valid_tail.split_at(self.end - self.index);
		f.debug_struct("ExtractIf")
			.field("retained", &retained)
			.field("remainder", &remainder)
			.field("skipped_tail", &skipped_tail)
			.finish_non_exhaustive()
	}
}
