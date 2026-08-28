use crate::Arena;
use crate::raw_vec::RawVec;
use allocator_api2::alloc::Allocator;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

/// A `bumpalo::vec!`-style constructor for the arena [`Vec`], generic over the allocator backend.
///
/// - `vec_in![in alloc]` -> empty
/// - `vec_in![in alloc; elem; n]` -> `n` clones of `elem`
/// - `vec_in![in alloc; a, b, c]` -> the listed elements, in order
#[macro_export]
macro_rules! vec_in {
	(in $alloc:expr $(,)?) => { $crate::Vec::new_in($alloc) };
	(in $alloc:expr; $elem:expr; $n:expr) => {{
		let n = $n;
		let mut v = $crate::Vec::with_capacity_in(n, $alloc);
		for _ in 0..n {
			v.push(::core::clone::Clone::clone(&$elem));
		}
		v
	}};
	(in $alloc:expr; $($x:expr),+ $(,)?) => {{
		let mut v = $crate::Vec::new_in($alloc);
		$( v.push($x); )+
		v
	}};
}

/// A growable, arena-allocated contiguous array, generic over any [`Allocator`].
///
/// Unlike `std`'s `Vec`, this never runs element destructors: values live in the arena and are released wholesale when
/// the arena is dropped. `T` should therefore not own resources outside the arena that require `Drop` to run.
#[repr(C)]
pub struct Vec<'a, T, A: Allocator = &'a Arena> {
	raw: RawVec<T>,
	alloc: A,
	marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, T, A: Allocator> Vec<'a, T, A> {
	/// Create a new, empty `Vec` backed by `alloc`. Allocates nothing until the first push.
	#[inline]
	pub fn new_in(alloc: A) -> Self {
		Self { raw: RawVec::new(), alloc, marker: std::marker::PhantomData }
	}

	/// Create a new, empty `Vec` with room for at least `cap` elements.
	#[inline]
	pub fn with_capacity_in(cap: usize, alloc: A) -> Self {
		let mut raw = RawVec::new();
		if cap > 0 {
			raw.grow(cap as u32, &alloc);
		}
		Self { raw, alloc, marker: std::marker::PhantomData }
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.raw.len as usize
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.raw.len == 0
	}

	#[inline]
	pub fn capacity(&self) -> usize {
		self.raw.cap as usize
	}

	/// View the contents as a slice.
	#[inline]
	pub fn as_slice(&self) -> &[T] {
		self
	}

	/// View the contents as a mutable slice.
	#[inline]
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		self
	}

	#[inline]
	fn reserve_one(&mut self) {
		if self.raw.len == self.raw.cap {
			self.raw.grow(self.raw.len + 1, &self.alloc);
		}
	}

	/// Append an element, growing the backing allocation if necessary.
	#[inline]
	pub fn push(&mut self, value: T) {
		self.reserve_one();
		debug_assert!(self.raw.len < self.raw.cap, "reserve_one must guarantee spare capacity");
		unsafe {
			self.raw.ptr.as_ptr().add(self.raw.len as usize).write(value);
		}
		self.raw.len += 1;
	}

	/// Remove and return the last element, or `None` if empty.
	#[inline]
	pub fn pop(&mut self) -> Option<T> {
		if self.raw.len == 0 {
			return None;
		}
		self.raw.len -= 1;
		Some(unsafe { self.raw.ptr.as_ptr().add(self.raw.len as usize).read() })
	}

	/// Insert `value` at `index`, shifting later elements right.
	///
	/// # Panics
	/// Panics if `index > len`.
	pub fn insert(&mut self, index: usize, value: T) {
		assert!(index as u32 <= self.raw.len, "insertion index out of bounds");
		self.reserve_one();
		debug_assert!(self.raw.len < self.raw.cap, "reserve_one must guarantee spare capacity");
		unsafe {
			let base = self.raw.ptr.as_ptr();
			let at = base.add(index);
			std::ptr::copy(at, at.add(1), (self.raw.len as usize) - index);
			at.write(value);
		}
		self.raw.len += 1;
	}

	/// Remove and return the element at `index`, shifting later elements left.
	///
	/// # Panics
	/// Panics if `index >= len`.
	pub fn remove(&mut self, index: usize) -> T {
		assert!((index as u32) < self.raw.len, "removal index out of bounds");
		unsafe {
			let base = self.raw.ptr.as_ptr();
			let at = base.add(index);
			let value = at.read();
			std::ptr::copy(at.add(1), at, (self.raw.len as usize) - index - 1);
			self.raw.len -= 1;
			value
		}
	}

	/// Shorten the vector to `len` elements. Excess elements are forgotten (no destructors run).
	#[inline]
	pub fn truncate(&mut self, len: usize) {
		if (len as u32) < self.raw.len {
			self.raw.len = len as u32;
		}
	}

	/// Empty the vector. Elements are forgotten (no destructors run).
	#[inline]
	pub fn clear(&mut self) {
		self.raw.len = 0;
	}

	/// Retain only elements for which `f` returns `true`, preserving order.
	///
	/// Panic-safe: if `f` panics, elements already processed are left in a consistent state (kept ones compacted to the
	/// front, dropped ones removed) and the not-yet-processed tail is shifted back so no element is duplicated or lost,
	/// mirroring [`std::vec::Vec::retain`].
	pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
		let original_len = self.raw.len;
		let base = self.raw.ptr.as_ptr();
		self.raw.len = 0;

		struct Guard<'v, 'a, T, A: Allocator> {
			v: &'v mut Vec<'a, T, A>,
			base: *mut T,
			processed: u32,
			deleted: u32,
			original_len: u32,
		}
		impl<'v, 'a, T, A: Allocator> Drop for Guard<'v, 'a, T, A> {
			fn drop(&mut self) {
				let tail = self.original_len - self.processed;
				if self.deleted > 0 && tail > 0 {
					unsafe {
						std::ptr::copy(
							self.base.add(self.processed as usize),
							self.base.add((self.processed - self.deleted) as usize),
							tail as usize,
						);
					}
				}
				self.v.raw.len = self.original_len - self.deleted;
			}
		}

		let mut g = Guard { v: self, base, processed: 0, deleted: 0, original_len };
		for read in 0..original_len {
			let keep = unsafe { f(&*g.base.add(read as usize)) };
			g.processed = read + 1;
			if keep {
				if g.deleted > 0 {
					unsafe {
						let src = g.base.add(read as usize);
						g.base.add((read - g.deleted) as usize).write(src.read());
					}
				}
			} else {
				unsafe { g.base.add(read as usize).drop_in_place() };
				g.deleted += 1;
			}
		}
		drop(g);
	}

	/// Remove consecutive elements that compare equal, keeping the first of each run.
	pub fn dedup(&mut self)
	where
		T: PartialEq,
	{
		if self.raw.len < 2 {
			return;
		}
		let base = self.raw.ptr.as_ptr();
		let mut write = 1usize;
		for read in 1..self.raw.len as usize {
			// SAFETY: `read` is below the length, and every index below `write` holds a live element that
			// was either never moved or written by an earlier step of this loop.
			let duplicate = unsafe { *base.add(read) == *base.add(write - 1) };
			if duplicate {
				// SAFETY: as above; the element is live and is not read again.
				unsafe { base.add(read).drop_in_place() };
				continue;
			}
			if write != read {
				// SAFETY: as above; `write` is behind `read`, so the read and the write do not alias.
				unsafe { base.add(write).write(base.add(read).read()) };
			}
			write += 1;
		}
		self.raw.len = write as u32;
	}

	/// Remove the elements in `range`, yielding them by value. Elements after the range are shifted
	/// down to fill the gap when the returned [`Drain`] is dropped.
	///
	/// # Panics
	/// Panics if the range is out of bounds or its start is after its end.
	pub fn drain<R: std::ops::RangeBounds<u32>>(&mut self, range: R) -> Drain<'_, T> {
		let len = self.raw.len;
		let start = match range.start_bound() {
			std::ops::Bound::Included(&n) => n,
			std::ops::Bound::Excluded(&n) => n + 1,
			std::ops::Bound::Unbounded => 0,
		};
		let end = match range.end_bound() {
			std::ops::Bound::Included(&n) => n + 1,
			std::ops::Bound::Excluded(&n) => n,
			std::ops::Bound::Unbounded => len,
		};
		assert!(start <= end, "drain start must not exceed end");
		assert!(end <= len, "drain range out of bounds");
		self.raw.len = start;
		Drain {
			ptr: self.raw.ptr.as_ptr(),
			index: start,
			end,
			tail: len,
			vec_len: NonNull::from(&mut self.raw.len),
			marker: std::marker::PhantomData,
		}
	}

	/// Consume the `Vec`, returning its contents as a slice borrowed from the arena for `'a`.
	///
	/// Use this to hand arena-allocated data to an API wanting `&'a [T]`: the elements outlive this handle because they
	/// belong to the arena, not to the `Vec`.
	#[inline]
	pub fn into_slice(self) -> &'a [T] {
		// SAFETY: `ptr` is aligned and points at `len` initialised `T`s living in the arena for `'a` (or is dangling when
		// `len` is 0, which `from_raw_parts` permits). `Vec` has no `Drop`, so nothing destroys the elements behind the
		// returned reference.
		unsafe { std::slice::from_raw_parts(self.raw.ptr.as_ptr(), self.raw.len as usize) }
	}
}

impl<'a, T: Clone, A: Allocator> Vec<'a, T, A> {
	/// Append all elements of `slice` by cloning.
	pub fn extend_from_slice(&mut self, slice: &[T]) {
		self.reserve(slice.len());
		for value in slice {
			self.push(value.clone());
		}
	}

	#[inline]
	fn reserve(&mut self, additional: usize) {
		let required = self.raw.len + (additional as u32);
		if required > self.raw.cap {
			self.raw.grow(required, &self.alloc);
		}
	}
}

impl<'a, T, A: Allocator> Extend<T> for Vec<'a, T, A> {
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		let iter = iter.into_iter();
		let (lower, _) = iter.size_hint();
		if lower > 0 {
			let required = self.raw.len + (lower as u32);
			if required > self.raw.cap {
				self.raw.grow(required, &self.alloc);
			}
		}
		for value in iter {
			self.push(value);
		}
	}
}

impl<'a, T, A: Allocator> Deref for Vec<'a, T, A> {
	type Target = [T];

	#[inline]
	fn deref(&self) -> &[T] {
		debug_assert!(self.raw.len <= self.raw.cap, "len must never exceed capacity");
		unsafe { std::slice::from_raw_parts(self.raw.ptr.as_ptr(), self.raw.len as usize) }
	}
}

impl<'a, T, A: Allocator> DerefMut for Vec<'a, T, A> {
	#[inline]
	fn deref_mut(&mut self) -> &mut [T] {
		debug_assert!(self.raw.len <= self.raw.cap, "len must never exceed capacity");
		unsafe { std::slice::from_raw_parts_mut(self.raw.ptr.as_ptr(), self.raw.len as usize) }
	}
}

impl<'a, T: Clone, A: Allocator + Clone> Clone for Vec<'a, T, A> {
	fn clone(&self) -> Self {
		let mut out = Vec::with_capacity_in(self.raw.len as usize, self.alloc.clone());
		for value in self.iter() {
			out.push(value.clone());
		}
		out
	}
}

impl<'a, T: fmt::Debug, A: Allocator> fmt::Debug for Vec<'a, T, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(&**self, f)
	}
}

impl<'a, T: PartialEq, A: Allocator> PartialEq for Vec<'a, T, A> {
	fn eq(&self, other: &Self) -> bool {
		**self == **other
	}
}

impl<'a, T: Eq, A: Allocator> Eq for Vec<'a, T, A> {}

impl<'a, T: PartialOrd, A: Allocator> PartialOrd for Vec<'a, T, A> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		(**self).partial_cmp(&**other)
	}
}

impl<'a, T: Ord, A: Allocator> Ord for Vec<'a, T, A> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		(**self).cmp(&**other)
	}
}

impl<'a, T, A: Allocator, I: std::slice::SliceIndex<[T]>> std::ops::Index<I> for Vec<'a, T, A> {
	type Output = I::Output;
	#[inline]
	fn index(&self, index: I) -> &Self::Output {
		std::ops::Index::index(&**self, index)
	}
}

impl<'a, T, A: Allocator, I: std::slice::SliceIndex<[T]>> std::ops::IndexMut<I> for Vec<'a, T, A> {
	#[inline]
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		std::ops::IndexMut::index_mut(&mut **self, index)
	}
}

impl<'a, T: Hash, A: Allocator> Hash for Vec<'a, T, A> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(**self).hash(state);
	}
}

impl<'a, T, A: Allocator> AsRef<[T]> for Vec<'a, T, A> {
	#[inline]
	fn as_ref(&self) -> &[T] {
		self
	}
}

impl<'v, 'a, T, A: Allocator> IntoIterator for &'v Vec<'a, T, A> {
	type Item = &'v T;
	type IntoIter = std::slice::Iter<'v, T>;
	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'v, 'a, T, A: Allocator> IntoIterator for &'v mut Vec<'a, T, A> {
	type Item = &'v mut T;
	type IntoIter = std::slice::IterMut<'v, T>;
	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}

impl<'a, T: 'a, A: Allocator> IntoIterator for Vec<'a, T, A> {
	type Item = T;
	type IntoIter = IntoIter<'a, T>;
	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		let iter =
			IntoIter { ptr: self.raw.ptr.as_ptr(), index: 0, len: self.raw.len, marker: std::marker::PhantomData };
		std::mem::forget(self);
		iter
	}
}

/// By-value iterator produced by [`Vec::into_iter`].
pub struct IntoIter<'a, T: 'a> {
	ptr: *mut T,
	index: u32,
	len: u32,
	marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IntoIter<'a, T> {
	type Item = T;
	#[inline]
	fn next(&mut self) -> Option<T> {
		if self.index == self.len {
			return None;
		}
		let value = unsafe { self.ptr.add(self.index as usize).read() };
		self.index += 1;
		Some(value)
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let remaining = (self.len - self.index) as usize;
		(remaining, Some(remaining))
	}
}

impl<'a, T> Drop for IntoIter<'a, T> {
	fn drop(&mut self) {
		while self.next().is_some() {}
	}
}

/// By-value iterator produced by [`Vec::drain`].
pub struct Drain<'v, T> {
	/// Base pointer of the source vector's buffer.
	ptr: *mut T,
	/// Index of the next element to yield (advances towards `end`).
	index: u32,
	/// One past the last index in the drained range.
	end: u32,
	/// Original length of the source vector (one past the last live element before draining).
	tail: u32,
	/// Pointer to the source vector's `len` field, restored on drop.
	vec_len: NonNull<u32>,
	marker: std::marker::PhantomData<&'v mut T>,
}

impl<'v, T> Iterator for Drain<'v, T> {
	type Item = T;
	#[inline]
	fn next(&mut self) -> Option<T> {
		if self.index == self.end {
			return None;
		}
		let value = unsafe { self.ptr.add(self.index as usize).read() };
		self.index += 1;
		Some(value)
	}
}

impl<'v, T> Drop for Drain<'v, T> {
	fn drop(&mut self) {
		while self.index < self.end {
			unsafe { self.ptr.add(self.index as usize).drop_in_place() };
			self.index += 1;
		}
		let drained = self.end;
		let tail = self.tail;
		debug_assert!(drained <= tail, "drain end must not exceed the original length");
		let count = tail - drained;
		unsafe {
			let start = *self.vec_len.as_ptr();
			debug_assert!(start <= drained, "drain start must not exceed the drained region start");
			if count > 0 {
				std::ptr::copy(self.ptr.add(drained as usize), self.ptr.add(start as usize), count as usize);
			}
			*self.vec_len.as_ptr() = start + count;
		}
	}
}

#[cfg(feature = "serde")]
impl<'a, T: serde::Serialize, A: Allocator> serde::Serialize for Vec<'a, T, A> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.collect_seq(self.iter())
	}
}

#[cfg(test)]
mod test {
	use super::Vec;
	use crate::Arena;
	use std::cell::Cell;
	use std::panic::{AssertUnwindSafe, catch_unwind};
	use std::rc::Rc;

	#[derive(Clone)]
	struct DropCounter {
		id: u32,
		drops: Rc<Cell<u32>>,
	}

	impl DropCounter {
		fn new(id: u32, drops: &Rc<Cell<u32>>) -> Self {
			Self { id, drops: Rc::clone(drops) }
		}
	}

	impl Drop for DropCounter {
		fn drop(&mut self) {
			self.drops.set(self.drops.get() + 1);
		}
	}

	#[test]
	fn new_is_empty_and_allocates_nothing() {
		let alloc = Arena::new();
		let v: Vec<i32> = Vec::new_in(&alloc);
		assert!(v.is_empty());
		assert_eq!(v.len(), 0);
		assert_eq!(v.capacity(), 0);
		assert_eq!(v.as_slice(), &[] as &[i32]);
	}

	#[test]
	fn with_capacity_reserves_but_stays_empty() {
		let alloc = Arena::new();
		let v: Vec<i32> = Vec::with_capacity_in(16, &alloc);
		assert!(v.is_empty());
		assert_eq!(v.len(), 0);
		assert!(v.capacity() >= 16);
	}

	#[test]
	fn with_capacity_zero_allocates_nothing() {
		let alloc = Arena::new();
		let v: Vec<i32> = Vec::with_capacity_in(0, &alloc);
		assert_eq!(v.capacity(), 0);
	}

	#[test]
	fn push_grows_and_preserves_order() {
		let alloc = Arena::new();
		let mut v: Vec<u32> = Vec::new_in(&alloc);
		for i in 0..1000u32 {
			v.push(i);
		}
		assert_eq!(v.len(), 1000);
		assert!(v.capacity() >= 1000);
		for (i, &value) in v.iter().enumerate() {
			assert_eq!(value, i as u32, "element is still in vec");
		}
	}

	#[test]
	fn pop_returns_last_then_none() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([10, 20, 30]);
		assert_eq!(v.pop(), Some(30));
		assert_eq!(v.pop(), Some(20));
		assert_eq!(v.pop(), Some(10));
		assert_eq!(v.pop(), None);
		assert!(v.is_empty());
	}

	#[test]
	fn insert_at_boundaries_and_middle() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2, 3]);
		v.insert(0, 0); // front
		assert_eq!(&*v, &[0, 1, 2, 3]);
		v.insert(v.len(), 4); // back (index == len)
		assert_eq!(&*v, &[0, 1, 2, 3, 4]);
		v.insert(2, 99); // middle
		assert_eq!(&*v, &[0, 1, 99, 2, 3, 4]);
	}

	#[test]
	fn insert_into_empty() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.insert(0, 42);
		assert_eq!(&*v, &[42]);
	}

	#[test]
	fn insert_out_of_bounds_panics() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2]);
		let result = catch_unwind(AssertUnwindSafe(|| v.insert(3, 0)));
		assert!(result.is_err());
	}

	#[test]
	fn remove_at_boundaries_and_middle() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2, 3, 4]);
		assert_eq!(v.remove(0), 0); // front
		assert_eq!(&*v, &[1, 2, 3, 4]);
		assert_eq!(v.remove(v.len() - 1), 4); // back
		assert_eq!(&*v, &[1, 2, 3]);
		assert_eq!(v.remove(1), 2); // middle
		assert_eq!(&*v, &[1, 3]);
	}

	#[test]
	fn remove_out_of_bounds_panics() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2]);
		let result = catch_unwind(AssertUnwindSafe(|| v.remove(2)));
		assert!(result.is_err());
	}

	#[test]
	fn truncate_shortens_without_dropping() {
		let alloc = Arena::new();
		let drops = Rc::new(Cell::new(0));
		let mut v: Vec<DropCounter> = Vec::new_in(&alloc);
		for i in 0..5 {
			v.push(DropCounter::new(i, &drops));
		}
		v.truncate(2);
		assert_eq!(v.len(), 2);
		assert_eq!(drops.get(), 0, "truncate must not run destructors");
	}

	#[test]
	fn truncate_longer_than_len_is_noop() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2, 3]);
		v.truncate(10);
		assert_eq!(&*v, &[1, 2, 3]);
	}

	#[test]
	fn clear_empties_without_dropping() {
		let alloc = Arena::new();
		let drops = Rc::new(Cell::new(0));
		let mut v: Vec<DropCounter> = Vec::new_in(&alloc);
		for i in 0..3 {
			v.push(DropCounter::new(i, &drops));
		}
		v.clear();
		assert!(v.is_empty());
		assert_eq!(drops.get(), 0, "clear must not run destructors");
	}

	#[test]
	fn extend_from_slice_clones_elements() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.push(1);
		v.extend_from_slice(&[2, 3, 4]);
		assert_eq!(&*v, &[1, 2, 3, 4]);
	}

	#[test]
	fn extend_with_accurate_size_hint_reserves_once() {
		let alloc = Arena::new();
		let mut v: Vec<u32> = Vec::new_in(&alloc);
		v.extend(0..64u32);
		assert_eq!(v.len(), 64);
		for i in 0..64u32 {
			assert_eq!(v[i as usize], i);
		}
	}

	#[test]
	fn extend_empty_iterator_is_noop() {
		let alloc = Arena::new();
		let mut v: Vec<u32> = Vec::new_in(&alloc);
		v.extend(std::iter::empty::<u32>());
		assert!(v.is_empty());
		assert_eq!(v.capacity(), 0);
	}

	#[test]
	fn retain_keeps_matching_and_shifts() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2, 3, 4, 5, 6, 7]);
		v.retain(|&x| x % 2 == 0);
		assert_eq!(&*v, &[0, 2, 4, 6]);
	}

	#[test]
	fn dedup_collapses_consecutive_runs_only() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 1, 2, 3, 3, 3, 1, 1]);
		v.dedup();
		assert_eq!(&*v, &[1, 2, 3, 1]);
	}

	#[test]
	fn dedup_leaves_short_and_distinct_vectors_alone() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.dedup();
		assert!(v.is_empty());
		v.extend([1, 2, 3]);
		v.dedup();
		assert_eq!(&*v, &[1, 2, 3]);
		let mut same: Vec<i32> = Vec::new_in(&alloc);
		same.extend([7, 7, 7]);
		same.dedup();
		assert_eq!(&*same, &[7]);
	}

	#[test]
	fn retain_all_and_none() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2, 3]);
		v.retain(|_| true);
		assert_eq!(&*v, &[1, 2, 3]);
		v.retain(|_| false);
		assert!(v.is_empty());
	}

	#[test]
	fn retain_drops_removed_exactly_once() {
		let alloc = Arena::new();
		let drops = Rc::new(Cell::new(0));
		let mut v: Vec<DropCounter> = Vec::new_in(&alloc);
		for i in 0..6 {
			v.push(DropCounter::new(i, &drops));
		}
		v.retain(|c| c.id % 2 == 1);
		assert_eq!(v.len(), 3);
		assert_eq!(drops.get(), 3, "each removed element dropped exactly once");
		let ids: std::vec::Vec<u32> = v.iter().map(|c| c.id).collect();
		assert_eq!(ids, vec![1, 3, 5], "survivors intact and in order after write-back");
	}

	#[test]
	fn retain_panic_drops_no_element_twice() {
		let alloc = Arena::new();
		let drops = Rc::new(Cell::new(0));
		let mut v: Vec<DropCounter> = Vec::new_in(&alloc);
		for i in 0..5 {
			v.push(DropCounter::new(i, &drops));
		}
		let drops_for_closure = Rc::clone(&drops);
		let result = catch_unwind(AssertUnwindSafe(|| {
			v.retain(|c| {
				if c.id == 3 {
					panic!("boom");
				}
				// Drop id 0 and id 1 before the panic.
				c.id >= 2
			});
		}));
		assert!(result.is_err());
		let removed = drops_for_closure.get();
		assert_eq!(removed, 2, "only the elements filtered out before the panic were dropped");
		assert_eq!(v.len() as u32 + removed, 5, "no leaked or double-counted elements");
	}

	#[test]
	fn drain_empty_range() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2, 3]);
		let drained: std::vec::Vec<i32> = v.drain(1..1).collect();
		assert!(drained.is_empty());
		assert_eq!(&*v, &[1, 2, 3]);
	}

	#[test]
	fn drain_suffix() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2, 3, 4]);
		let drained: std::vec::Vec<i32> = v.drain(3..).collect();
		assert_eq!(drained, vec![3, 4]);
		assert_eq!(&*v, &[0, 1, 2]);
	}

	#[test]
	fn drain_inclusive_bound() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2, 3, 4]);
		let drained: std::vec::Vec<i32> = v.drain(1..=3).collect();
		assert_eq!(drained, vec![1, 2, 3]);
		assert_eq!(&*v, &[0, 4]);
	}

	#[test]
	fn drain_start_after_end_panics() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2]);
		use std::ops::Bound;
		let bad_range = (Bound::Included(2u32), Bound::Excluded(1u32));
		let result = catch_unwind(AssertUnwindSafe(|| {
			let _ = v.drain(bad_range);
		}));
		assert!(result.is_err());
	}

	#[test]
	fn drain_out_of_bounds_panics() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2]);
		let result = catch_unwind(AssertUnwindSafe(|| {
			let _ = v.drain(1..99);
		}));
		assert!(result.is_err());
	}

	#[test]
	fn drain_yielded_and_remaining_drop_exactly_once() {
		let alloc = Arena::new();
		let drops = Rc::new(Cell::new(0));
		let mut v: Vec<DropCounter> = Vec::new_in(&alloc);
		for i in 0..6 {
			v.push(DropCounter::new(i, &drops));
		}
		{
			let mut d = v.drain(1..4);
			let first = d.next().unwrap();
			assert_eq!(first.id, 1);
			drop(first); // 1 drop
			// d dropped here: ids 2, 3 dropped by Drain::drop -> 2 more drops.
		}
		assert_eq!(drops.get(), 3, "drained range dropped exactly once total");
		let ids: std::vec::Vec<u32> = v.iter().map(|c| c.id).collect();
		assert_eq!(ids, vec![0, 4, 5], "tail shifted correctly after partial drain");
	}

	#[test]
	fn drain_fully_consumed_then_no_extra_drops() {
		let alloc = Arena::new();
		let drops = Rc::new(Cell::new(0));
		let mut v: Vec<DropCounter> = Vec::new_in(&alloc);
		for i in 0..4 {
			v.push(DropCounter::new(i, &drops));
		}
		let collected: std::vec::Vec<DropCounter> = v.drain(..).collect();
		let ids: std::vec::Vec<u32> = collected.iter().map(|c| c.id).collect();
		assert_eq!(ids, vec![0, 1, 2, 3]);
		// The drained DropCounters were moved into `collected`; none dropped yet.
		assert_eq!(drops.get(), 0);
		assert!(v.is_empty());
		drop(collected);
		assert_eq!(drops.get(), 4, "each drained element dropped exactly once when the collection drops");
	}

	#[test]
	fn drain_size_hint_not_relied_on_but_iteration_correct() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([5, 6, 7, 8]);
		let mut iter = v.drain(0..4);
		assert_eq!(iter.next(), Some(5));
		assert_eq!(iter.next(), Some(6));
		assert_eq!(iter.next(), Some(7));
		assert_eq!(iter.next(), Some(8));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn drain_prefix_shifts_tail() {
		let alloc = Arena::default();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2, 3, 4, 5]);
		let drained: std::vec::Vec<i32> = v.drain(0..2).collect();
		assert_eq!(drained, vec![0, 1]);
		assert_eq!(&*v, &[2, 3, 4, 5]);
	}

	#[test]
	fn drain_middle_shifts_tail() {
		let alloc = Arena::default();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2, 3, 4, 5]);
		let drained: std::vec::Vec<i32> = v.drain(2..4).collect();
		assert_eq!(drained, vec![2, 3]);
		assert_eq!(&*v, &[0, 1, 4, 5]);
	}

	#[test]
	fn drain_full_range() {
		let alloc = Arena::default();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2, 3]);
		let drained: std::vec::Vec<i32> = v.drain(..).collect();
		assert_eq!(drained, vec![1, 2, 3]);
		assert!(v.is_empty());
	}

	#[test]
	fn drain_dropped_without_iterating_still_shifts() {
		let alloc = Arena::default();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2, 3, 4]);
		drop(v.drain(1..3));
		assert_eq!(&*v, &[0, 3, 4]);
	}

	#[test]
	fn retain_preserves_length_when_predicate_panics() {
		let alloc = Arena::new();
		let mut values = Vec::new_in(&alloc);
		values.extend([0, 1, 2]);
		let calls = Cell::new(0);

		let result = catch_unwind(AssertUnwindSafe(|| {
			values.retain(|_| {
				let call = calls.get();
				calls.set(call + 1);
				match call {
					0 => false,
					1 => true,
					_ => panic!("predicate failed"),
				}
			});
		}));

		assert!(result.is_err());
		assert_eq!(values.len(), 2, "moved-from slots must not remain visible");
	}

	#[test]
	fn into_iter_yields_all_in_order() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2, 3, 4]);
		let collected: std::vec::Vec<i32> = v.into_iter().collect();
		assert_eq!(collected, vec![1, 2, 3, 4]);
	}

	#[test]
	fn into_iter_size_hint_is_exact() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2, 3]);
		let mut iter = v.into_iter();
		assert_eq!(iter.size_hint(), (3, Some(3)));
		iter.next();
		assert_eq!(iter.size_hint(), (2, Some(2)));
	}

	#[test]
	fn into_iter_partial_consume_drops_remainder_once() {
		let alloc = Arena::new();
		let drops = Rc::new(Cell::new(0));
		let mut v: Vec<DropCounter> = Vec::new_in(&alloc);
		for i in 0..5 {
			v.push(DropCounter::new(i, &drops));
		}
		{
			let mut iter = v.into_iter();
			let a = iter.next().unwrap();
			let b = iter.next().unwrap();
			assert_eq!((a.id, b.id), (0, 1));
			drop(a); // 1
			drop(b); // 1
			// iter dropped here: ids 2, 3, 4 dropped by IntoIter::drop -> 3 more.
		}
		assert_eq!(drops.get(), 5, "every element dropped exactly once across manual + IntoIter drop");
	}

	#[test]
	fn into_iter_fully_consumed_no_leak() {
		let alloc = Arena::new();
		let drops = Rc::new(Cell::new(0));
		let mut v: Vec<DropCounter> = Vec::new_in(&alloc);
		for i in 0..4 {
			v.push(DropCounter::new(i, &drops));
		}
		for c in v {
			let _ = c.id; // dropped at end of each loop iteration
		}
		assert_eq!(drops.get(), 4);
	}

	#[test]
	fn clone_is_independent_copy() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2, 3]);
		let mut c = v.clone();
		c.push(4);
		assert_eq!(&*v, &[1, 2, 3], "original unchanged after mutating clone");
		assert_eq!(&*c, &[1, 2, 3, 4]);
	}

	#[test]
	fn eq_and_ord_delegate_to_slice() {
		let alloc = Arena::new();
		let mut a: Vec<i32> = Vec::new_in(&alloc);
		a.extend([1, 2, 3]);
		let mut b: Vec<i32> = Vec::new_in(&alloc);
		b.extend([1, 2, 3]);
		assert_eq!(a, b);
		let mut c: Vec<i32> = Vec::new_in(&alloc);
		c.extend([1, 2, 4]);
		assert!(a < c);
		assert_ne!(a, c);
	}

	#[test]
	fn index_and_index_mut() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([10, 20, 30]);
		assert_eq!(v[1], 20);
		v[1] = 99;
		assert_eq!(v[1], 99);
		assert_eq!(&v[0..2], &[10, 99]);
	}

	#[test]
	fn hash_matches_equal_vecs() {
		use std::collections::hash_map::DefaultHasher;
		use std::hash::{Hash, Hasher};
		let alloc = Arena::new();
		let mut a: Vec<i32> = Vec::new_in(&alloc);
		a.extend([1, 2, 3]);
		let mut b: Vec<i32> = Vec::new_in(&alloc);
		b.extend([1, 2, 3]);
		let mut ha = DefaultHasher::new();
		let mut hb = DefaultHasher::new();
		a.hash(&mut ha);
		b.hash(&mut hb);
		assert_eq!(ha.finish(), hb.finish());
	}

	#[test]
	fn realloc_preserves_boxed_contents() {
		let alloc = Arena::new();
		let mut v: Vec<std::boxed::Box<u32>> = Vec::new_in(&alloc);
		for i in 0..512u32 {
			v.push(std::boxed::Box::new(i));
		}
		for (i, b) in v.iter().enumerate() {
			assert_eq!(**b, i as u32);
		}
		let sum: u32 = v.into_iter().map(|b| *b).sum();
		assert_eq!(sum, (0..512u32).sum());
	}

	#[test]
	fn zero_sized_type_push_pop_len() {
		let alloc = Arena::new();
		let mut v: Vec<()> = Vec::new_in(&alloc);
		for _ in 0..100 {
			v.push(());
		}
		assert_eq!(v.len(), 100);
		for _ in 0..100 {
			assert_eq!(v.pop(), Some(()));
		}
		assert_eq!(v.pop(), None);
	}
}
