use super::Drain;
use crate::Arena;
use std::alloc::Allocator;
use std::fmt;

/// A splicing iterator for [`Vec`](super::Vec), created by [`Vec::splice`](super::Vec::splice).
pub struct Splice<'v, 'a, I: Iterator + 'v, A: Allocator + 'v = &'a Arena> {
	pub(super) drain: Drain<'v, 'a, I::Item, A>,
	pub(super) replace_with: I,
}

impl<'v, 'a, I: Iterator + fmt::Debug, A: Allocator> fmt::Debug for Splice<'v, 'a, I, A>
where
	I::Item: fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Splice").field("drain", &self.drain).field("replace_with", &self.replace_with).finish()
	}
}

impl<'v, 'a, I: Iterator, A: Allocator> Iterator for Splice<'v, 'a, I, A> {
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		self.drain.next()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.drain.size_hint()
	}
}

impl<'v, 'a, I: Iterator, A: Allocator> DoubleEndedIterator for Splice<'v, 'a, I, A> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.drain.next_back()
	}
}

impl<'v, 'a, I: Iterator, A: Allocator> ExactSizeIterator for Splice<'v, 'a, I, A> {}

impl<'v, 'a, I: Iterator, A: Allocator> Drop for Splice<'v, 'a, I, A> {
	fn drop(&mut self) {
		self.drain.by_ref().for_each(drop);
		self.drain.iter = [].iter();

		if self.drain.tail_len == 0 {
			self.drain.vec.extend(self.replace_with.by_ref());
			return;
		}

		if !self.drain.fill(&mut self.replace_with) {
			return;
		}

		let (lower_bound, _upper_bound) = self.replace_with.size_hint();
		if lower_bound > 0 {
			self.drain.move_tail(lower_bound);
			if !self.drain.fill(&mut self.replace_with) {
				return;
			}
		}

		let mut collected = self.replace_with.by_ref().collect::<std::vec::Vec<I::Item>>().into_iter();
		if collected.len() > 0 {
			self.drain.move_tail(collected.len());
			let filled = self.drain.fill(&mut collected);
			debug_assert!(filled);
			debug_assert_eq!(collected.len(), 0);
		}
	}
}

impl<'v, 'a, T, A: Allocator> Drain<'v, 'a, T, A> {
	/// Fill the gap between the vector's length and `tail_start` from `replace_with`, returning `true` if the gap was
	/// filled.
	fn fill<I: Iterator<Item = T>>(&mut self, replace_with: &mut I) -> bool {
		for index in self.vec.len()..self.tail_start {
			let Some(new_item) = replace_with.next() else {
				return false;
			};
			// SAFETY: `index` is the vacant slot at the vector's length, inside the gap before `tail_start`; the new length covers the item just written.
			unsafe {
				self.vec.as_mut_ptr().add(index).write(new_item);
				self.vec.set_len(index + 1);
			}
		}
		true
	}

	/// Make room for `additional` more elements in the gap by moving the tail up.
	fn move_tail(&mut self, additional: usize) {
		let vec = &mut *self.vec;
		vec.raw.reserve(self.tail_start + self.tail_len, additional, &vec.alloc);
		let new_tail_start = self.tail_start + additional;
		// SAFETY: the tail is live at `tail_start`, and the reserve above guarantees room for it at `new_tail_start`.
		unsafe {
			let src = vec.as_ptr().add(self.tail_start);
			let dst = vec.as_mut_ptr().add(new_tail_start);
			std::ptr::copy(src, dst, self.tail_len);
		}
		self.tail_start = new_tail_start;
	}
}
