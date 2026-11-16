use std::fmt;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

/// An iterator that moves out of a vector, created by [`Vec::into_iter`](super::Vec::into_iter).
pub struct IntoIter<'a, T: 'a> {
	ptr: NonNull<T>,
	/// Index of the next element to yield from the front.
	index: usize,
	/// One past the next element to yield from the back.
	end: usize,
	marker: PhantomData<&'a mut T>,
}

impl<'a, T> IntoIter<'a, T> {
	pub(super) fn new(elements: &'a mut [T]) -> Self {
		Self { end: elements.len(), ptr: NonNull::from(elements).cast(), index: 0, marker: PhantomData }
	}

	/// The elements not yet yielded, as a slice.
	pub fn as_slice(&self) -> &[T] {
		// SAFETY: `remaining` covers the live elements not yet yielded.
		unsafe { &*self.remaining() }
	}

	/// The elements not yet yielded, as a mutable slice.
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		// SAFETY: `remaining` covers the live elements not yet yielded, owned by this iterator.
		unsafe { &mut *self.remaining() }
	}

	fn remaining(&self) -> *mut [T] {
		ptr::slice_from_raw_parts_mut(self.ptr.as_ptr().wrapping_add(self.index), self.len())
	}
}

impl<'a, T: fmt::Debug> fmt::Debug for IntoIter<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("IntoIter").field(&self.as_slice()).finish()
	}
}

impl<'a, T> AsRef<[T]> for IntoIter<'a, T> {
	fn as_ref(&self) -> &[T] {
		self.as_slice()
	}
}

impl<'a, T> Iterator for IntoIter<'a, T> {
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<T> {
		if self.index == self.end {
			return None;
		}
		self.index += 1;
		// SAFETY: the element at the old `index` is live and no longer covered by `index..end`.
		Some(unsafe { self.ptr.add(self.index - 1).read() })
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let exact = self.end - self.index;
		(exact, Some(exact))
	}
}

impl<'a, T> DoubleEndedIterator for IntoIter<'a, T> {
	#[inline]
	fn next_back(&mut self) -> Option<T> {
		if self.index == self.end {
			return None;
		}
		self.end -= 1;
		// SAFETY: the element at the new `end` is live and no longer covered by `index..end`.
		Some(unsafe { self.ptr.add(self.end).read() })
	}
}

impl<'a, T> ExactSizeIterator for IntoIter<'a, T> {}

impl<'a, T> FusedIterator for IntoIter<'a, T> {}

impl<'a, T> Drop for IntoIter<'a, T> {
	fn drop(&mut self) {
		let remaining = self.remaining();
		self.index = self.end;
		// SAFETY: `remaining` holds the live elements not yet yielded, which nothing else will read.
		unsafe { ptr::drop_in_place(remaining) };
	}
}
