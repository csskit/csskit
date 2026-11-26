use crate::Arena;
use crate::raw_vec::RawVec;
use std::alloc::Allocator;
use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Bound, Deref, DerefMut, Range, RangeBounds};
use std::ptr::{self, NonNull};
use std::{iter, slice};

mod drain;
mod extract_if;
mod into_iter;
mod partial_eq;
mod splice;

pub use drain::Drain;
pub use extract_if::ExtractIf;
pub use into_iter::IntoIter;
pub use splice::Splice;

/// A `bumpalo::vec!`-style constructor for the arena [`Vec`], generic over the allocator backend.
///
/// - `vec_in![in alloc]` -> empty
/// - `vec_in![in alloc; elem; n]` -> `n` clones of `elem`
/// - `vec_in![in alloc; a, b, c]` -> the listed elements, in order
#[macro_export]
macro_rules! vec_in {
	(in $alloc:expr $(,)?) => { $crate::Vec::new_in($alloc) };
	(in $alloc:expr; $elem:expr; $n:expr) => {{
		let mut v = $crate::Vec::new_in($alloc);
		v.resize($n, $elem);
		v
	}};
	(in $alloc:expr; $($x:expr),+ $(,)?) => {{
		let mut v = $crate::Vec::new_in($alloc);
		$( v.push($x); )+
		v
	}};
}

/// The error returned by [`Vec::try_reserve`] and [`Vec::try_reserve_exact`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TryReserveError {
	/// The requested capacity exceeds `u32::MAX` elements or `isize::MAX` bytes.
	CapacityOverflow,
	/// The allocator could not provide the requested memory.
	AllocError,
}

impl TryReserveError {
	#[cold]
	#[track_caller]
	pub(crate) fn raise(self) -> ! {
		match self {
			Self::CapacityOverflow => panic!("Vec capacity overflow"),
			Self::AllocError => panic!("arena exhausted"),
		}
	}
}

impl fmt::Display for TryReserveError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Self::CapacityOverflow => "memory allocation failed because the computed capacity exceeded the maximum",
			Self::AllocError => "memory allocation failed because the allocator returned an error",
		})
	}
}

impl std::error::Error for TryReserveError {}

#[track_caller]
fn slice_range<R: RangeBounds<usize>>(range: R, len: usize) -> Range<usize> {
	let start = match range.start_bound() {
		Bound::Included(&start) => start,
		Bound::Excluded(&start) => {
			start.checked_add(1).unwrap_or_else(|| panic!("attempted to index slice from after maximum usize"))
		}
		Bound::Unbounded => 0,
	};
	let end = match range.end_bound() {
		Bound::Included(&end) => {
			end.checked_add(1).unwrap_or_else(|| panic!("attempted to index slice up to maximum usize"))
		}
		Bound::Excluded(&end) => end,
		Bound::Unbounded => len,
	};
	if start > end {
		panic!("slice index starts at {start} but ends at {end}");
	}
	if end > len {
		panic!("range end index {end} out of range for slice of length {len}");
	}
	start..end
}

/// A growable, arena-allocated contiguous array, generic over any [`Allocator`].
///
/// Unlike `std`'s `Vec`, dropping it, [`Vec::truncate`], [`Vec::clear`] and shrinking with [`Vec::resize`] forget
/// elements rather than running their destructors: values live in the arena and are released wholesale when the arena
/// is dropped. `T` should therefore not own resources outside the arena that require `Drop` to run. Operations that
/// remove elements one by one (`pop`, `remove`, `drain`, `retain`, `dedup` and the like) do drop them.
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

	/// Create a new, empty `Vec` with room for exactly `capacity` elements.
	#[inline]
	pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
		let mut vec = Self::new_in(alloc);
		vec.reserve_exact(capacity);
		vec
	}

	/// Like [`Vec::with_capacity_in`], but returns an error instead of panicking.
	pub fn try_with_capacity_in(capacity: usize, alloc: A) -> Result<Self, TryReserveError> {
		let mut vec = Self::new_in(alloc);
		vec.try_reserve_exact(capacity)?;
		Ok(vec)
	}

	/// Create a `Vec` holding the items of `iter`, backed by `alloc`.
	pub fn from_iter_in<I: IntoIterator<Item = T>>(iter: I, alloc: A) -> Self {
		let mut vec = Self::new_in(alloc);
		vec.extend(iter);
		vec
	}

	/// Create a `Vec` from a pointer, a length, a capacity and the allocator that owns the buffer.
	///
	/// # Safety
	/// `ptr` must be non-null, and either dangling with `capacity` 0 or allocated by `alloc` with the layout of
	/// `capacity` elements of `T`. The first `length` elements must be initialised, `length` must not exceed `capacity`,
	/// and `capacity` must not exceed `u32::MAX`.
	#[inline]
	pub unsafe fn from_raw_parts_in(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self {
		// SAFETY: the caller guarantees `ptr` is non-null and upholds the rest of the contract.
		unsafe { Self::from_parts_in(NonNull::new_unchecked(ptr), length, capacity, alloc) }
	}

	/// Create a `Vec` from a non-null pointer, a length, a capacity and the allocator that owns the buffer.
	///
	/// # Safety
	/// Same as [`Vec::from_raw_parts_in`].
	#[inline]
	pub unsafe fn from_parts_in(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self {
		debug_assert!(length <= capacity && capacity <= u32::MAX as usize, "raw parts must describe a valid Vec");
		Self { raw: RawVec { ptr, len: length as u32, cap: capacity as u32 }, alloc, marker: std::marker::PhantomData }
	}

	/// Decompose the `Vec` into its pointer, length, capacity and allocator, the inverse of [`Vec::from_raw_parts_in`].
	#[inline]
	pub fn into_raw_parts_with_allocator(self) -> (*mut T, usize, usize, A) {
		let (ptr, len, capacity, alloc) = self.into_parts_with_allocator();
		(ptr.as_ptr(), len, capacity, alloc)
	}

	/// Decompose the `Vec` into its non-null pointer, length, capacity and allocator, the inverse of
	/// [`Vec::from_parts_in`].
	#[inline]
	pub fn into_parts_with_allocator(self) -> (NonNull<T>, usize, usize, A) {
		let Self { raw, alloc, .. } = self;
		(raw.ptr, raw.len as usize, raw.cap as usize, alloc)
	}

	/// The allocator backing this `Vec`.
	#[inline]
	pub fn allocator(&self) -> &A {
		&self.alloc
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

	/// A raw pointer to the buffer, valid for reads of `len` elements until the `Vec` reallocates.
	#[inline]
	pub fn as_ptr(&self) -> *const T {
		self.raw.ptr.as_ptr()
	}

	/// A raw pointer to the buffer, valid for `capacity` elements until the `Vec` reallocates.
	#[inline]
	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.raw.ptr.as_ptr()
	}

	/// A non-null pointer to the buffer, valid for `capacity` elements until the `Vec` reallocates.
	#[inline]
	pub fn as_non_null(&mut self) -> NonNull<T> {
		self.raw.ptr
	}

	/// Force the length to `new_len`.
	///
	/// # Safety
	/// `new_len` must not exceed the capacity, and the elements up to `new_len` must be initialised.
	#[inline]
	pub unsafe fn set_len(&mut self, new_len: usize) {
		debug_assert!(new_len <= self.capacity(), "set_len must not exceed capacity");
		self.raw.len = new_len as u32;
	}

	/// The unused capacity, as a slice of uninitialised elements.
	#[inline]
	pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
		// SAFETY: the buffer has room for `capacity` elements, those past `len` are unused, and `MaybeUninit` needs no initialisation.
		unsafe {
			slice::from_raw_parts_mut(
				self.as_mut_ptr().add(self.len()).cast::<MaybeUninit<T>>(),
				self.capacity() - self.len(),
			)
		}
	}

	/// Reserve room for at least `additional` more elements, possibly more to amortise future growth.
	///
	/// # Panics
	/// Panics if the new capacity exceeds `u32::MAX` elements or the allocator fails.
	#[inline]
	pub fn reserve(&mut self, additional: usize) {
		self.raw.reserve(self.len(), additional, &self.alloc);
	}

	/// Reserve room for exactly `additional` more elements.
	///
	/// # Panics
	/// Panics if the new capacity exceeds `u32::MAX` elements or the allocator fails.
	pub fn reserve_exact(&mut self, additional: usize) {
		self.raw.reserve_exact(self.len(), additional, &self.alloc);
	}

	/// Like [`Vec::reserve`], but returns an error instead of panicking.
	pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
		self.raw.try_reserve(self.len(), additional, &self.alloc)
	}

	/// Like [`Vec::reserve_exact`], but returns an error instead of panicking.
	pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
		self.raw.try_reserve_exact(self.len(), additional, &self.alloc)
	}

	/// Shrink the capacity to the length. An arena reclaims the freed space if this is its most recent allocation.
	pub fn shrink_to_fit(&mut self) {
		self.shrink_to(0);
	}

	/// Shrink the capacity to `min_capacity`, or to the length if that is greater.
	pub fn shrink_to(&mut self, min_capacity: usize) {
		if self.capacity() > min_capacity {
			self.raw.shrink_to_fit(self.raw.len.max(min_capacity as u32), &self.alloc);
		}
	}

	/// Append an element, growing the backing allocation if necessary.
	#[inline]
	pub fn push(&mut self, value: T) {
		let _ = self.push_mut(value);
	}

	/// Append an element and return a mutable reference to it.
	#[inline]
	pub fn push_mut(&mut self, value: T) -> &mut T {
		let len = self.len();
		if len == self.capacity() {
			self.raw.grow_one(&self.alloc);
		}
		// SAFETY: `len < capacity`, so the slot is in bounds and unused; the new length then covers the value just written.
		unsafe {
			let end = self.as_mut_ptr().add(len);
			ptr::write(end, value);
			self.set_len(len + 1);
			&mut *end
		}
	}

	/// Append an element if there is spare capacity, returning it back otherwise. Never allocates.
	#[inline]
	pub fn push_within_capacity(&mut self, value: T) -> Result<&mut T, T> {
		if self.len() == self.capacity() {
			return Err(value);
		}
		Ok(self.push_mut(value))
	}

	/// Remove and return the last element, or `None` if empty.
	#[inline]
	pub fn pop(&mut self) -> Option<T> {
		if self.raw.len == 0 {
			None
		} else {
			self.raw.len -= 1;
			// SAFETY: the old last element is live and the new length no longer covers it.
			Some(unsafe { ptr::read(self.as_ptr().add(self.len())) })
		}
	}

	/// Remove and return the last element if `predicate` returns `true` for it.
	pub fn pop_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T> {
		let last = self.last_mut()?;
		if predicate(last) { self.pop() } else { None }
	}

	/// Insert `element` at `index`, shifting later elements right.
	///
	/// # Panics
	/// Panics if `index > len`.
	#[track_caller]
	pub fn insert(&mut self, index: usize, element: T) {
		let _ = self.insert_mut(index, element);
	}

	/// Insert `element` at `index`, shifting later elements right, and return a mutable reference to it.
	///
	/// # Panics
	/// Panics if `index > len`.
	#[track_caller]
	pub fn insert_mut(&mut self, index: usize, element: T) -> &mut T {
		#[cold]
		#[track_caller]
		fn assert_failed(index: usize, len: usize) -> ! {
			panic!("insertion index (is {index}) should be <= len (is {len})");
		}

		let len = self.len();
		if index > len {
			assert_failed(index, len);
		}
		self.push(element);
		self[index..].rotate_right(1);
		&mut self[index]
	}

	/// Remove and return the element at `index`, shifting later elements left.
	///
	/// # Panics
	/// Panics if `index >= len`.
	#[track_caller]
	pub fn remove(&mut self, index: usize) -> T {
		#[cold]
		#[track_caller]
		fn assert_failed(index: usize, len: usize) -> ! {
			panic!("removal index (is {index}) should be < len (is {len})");
		}

		match self.try_remove(index) {
			Some(element) => element,
			None => assert_failed(index, self.len()),
		}
	}

	/// Remove and return the element at `index`, shifting later elements left, or `None` if `index` is out of bounds.
	pub fn try_remove(&mut self, index: usize) -> Option<T> {
		if index >= self.len() {
			return None;
		}
		self[index..].rotate_left(1);
		self.pop()
	}

	/// Remove and return the element at `index`, moving the last element into its place. Does not preserve order.
	///
	/// # Panics
	/// Panics if `index >= len`.
	#[track_caller]
	pub fn swap_remove(&mut self, index: usize) -> T {
		#[cold]
		#[track_caller]
		fn assert_failed(index: usize, len: usize) -> ! {
			panic!("swap_remove index (is {index}) should be < len (is {len})");
		}

		let len = self.len();
		if index >= len {
			assert_failed(index, len);
		}
		self.swap(index, len - 1);
		self.pop().expect("the bounds check guarantees an element")
	}

	/// Shorten the vector to `len` elements. Excess elements are forgotten (no destructors run).
	#[inline]
	pub fn truncate(&mut self, len: usize) {
		if len > self.len() {
			return;
		}
		self.raw.len = len as u32;
	}

	/// Empty the vector. Elements are forgotten (no destructors run).
	#[inline]
	pub fn clear(&mut self) {
		self.raw.len = 0;
	}

	/// Move all elements of `other` onto the end of `self`, leaving `other` empty.
	pub fn append(&mut self, other: &mut Self) {
		self.extend(other.drain(..));
	}

	/// Move the elements from `at` onwards into a new `Vec` sharing this one's allocator.
	///
	/// # Panics
	/// Panics if `at > len`.
	#[track_caller]
	pub fn split_off(&mut self, at: usize) -> Self
	where
		A: Clone,
	{
		#[cold]
		#[track_caller]
		fn assert_failed(at: usize, len: usize) -> ! {
			panic!("`at` split index (is {at}) should be <= len (is {len})");
		}

		if at > self.len() {
			assert_failed(at, self.len());
		}
		let mut other = Self::with_capacity_in(self.len() - at, self.alloc.clone());
		other.extend(self.drain(at..));
		other
	}

	/// Resize to `new_len`, filling new slots with values returned by `f`. Shrinking forgets the excess elements, like
	/// [`Vec::truncate`].
	pub fn resize_with<F: FnMut() -> T>(&mut self, new_len: usize, f: F) {
		let len = self.len();
		if new_len > len {
			self.extend(iter::repeat_with(f).take(new_len - len));
		} else {
			self.truncate(new_len);
		}
	}

	/// Retain only elements for which `f` returns `true`, preserving order.
	///
	/// Panic-safe: if `f` panics, elements already processed are left in a consistent state (kept ones compacted to the
	/// front, dropped ones removed) and the not-yet-processed tail is shifted back so no element is duplicated or lost,
	/// mirroring [`std::vec::Vec::retain`].
	pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
		self.retain_mut(|elem| f(elem));
	}

	/// Retain only elements for which `f` returns `true`, preserving order, passing each element mutably to `f`.
	///
	/// Panic-safe in the same way as [`Vec::retain`], mirroring [`std::vec::Vec::retain_mut`].
	pub fn retain_mut<F: FnMut(&mut T) -> bool>(&mut self, mut f: F) {
		self.extract_if(.., |element| !f(element)).for_each(drop);
	}

	/// Remove consecutive elements that compare equal, keeping the first of each run.
	#[inline]
	pub fn dedup(&mut self)
	where
		T: PartialEq,
	{
		self.dedup_by(|a, b| a == b);
	}

	/// Remove consecutive elements that map to the same key, keeping the first of each run.
	#[inline]
	pub fn dedup_by_key<F: FnMut(&mut T) -> K, K: PartialEq>(&mut self, mut key: F) {
		self.dedup_by(|a, b| key(a) == key(b));
	}

	/// Remove consecutive elements for which `same_bucket(element, previous_kept)` returns `true`, keeping the first of
	/// each run. If `same_bucket` panics, no element is lost or dropped twice, but duplicates found so far may be
	/// reordered.
	pub fn dedup_by<F: FnMut(&mut T, &mut T) -> bool>(&mut self, mut same_bucket: F) {
		let len = self.len();
		if len <= 1 {
			return;
		}
		let mut write = 1;
		for read in 1..len {
			let (kept, unchecked) = self.split_at_mut(read);
			if !same_bucket(&mut unchecked[0], &mut kept[write - 1]) {
				if read != write {
					self.swap(read, write);
				}
				write += 1;
			}
		}
		self.drain(write..);
	}

	/// Remove the elements in `range`, yielding them by value. Elements after the range are shifted down to fill the gap
	/// when the returned [`Drain`] is dropped.
	///
	/// # Panics
	/// Panics if the range is out of bounds or its start is after its end.
	#[track_caller]
	pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, 'a, T, A> {
		let len = self.len();
		let Range { start, end } = slice_range(range, len);
		// SAFETY: `start..end` is in bounds and initialised; the shortened length hands it to the returned `Drain`.
		let range_slice = unsafe {
			self.set_len(start);
			slice::from_raw_parts(self.as_ptr().add(start), end - start)
		};
		Drain { tail_start: end, tail_len: len - end, iter: range_slice.iter(), vec: self }
	}

	/// Remove and yield the elements in `range` for which `filter` returns `true`, preserving the order of the rest.
	/// Elements the iterator has not reached when it is dropped are kept.
	///
	/// # Panics
	/// Panics if the range is out of bounds or its start is after its end.
	#[track_caller]
	pub fn extract_if<F: FnMut(&mut T) -> bool, R: RangeBounds<usize>>(
		&mut self,
		range: R,
		filter: F,
	) -> ExtractIf<'_, 'a, T, F, A> {
		ExtractIf::new(self, filter, range)
	}

	/// Replace the elements in `range` with the items of `replace_with`, yielding the removed elements. The
	/// replacement is written when the returned [`Splice`] is dropped.
	///
	/// # Panics
	/// Panics if the range is out of bounds or its start is after its end.
	#[track_caller]
	pub fn splice<R: RangeBounds<usize>, I: IntoIterator<Item = T>>(
		&mut self,
		range: R,
		replace_with: I,
	) -> Splice<'_, 'a, I::IntoIter, A> {
		Splice { drain: self.drain(range), replace_with: replace_with.into_iter() }
	}

	/// Consume the `Vec`, returning its contents as a slice borrowed from the arena for `'a`.
	///
	/// Use this to hand arena-allocated data to an API wanting `&'a [T]`: the elements outlive this handle because they
	/// belong to the arena, not to the `Vec`.
	#[inline]
	pub fn into_slice(self) -> &'a [T]
	where
		A: 'a,
	{
		self.leak()
	}

	/// Consume the `Vec`, returning its contents as a mutable slice borrowed from the arena for `'a`.
	///
	/// The slice cannot outlive the allocator borrow:
	///
	/// ```compile_fail
	/// use csskit_arena::{Arena, Vec};
	/// fn escape(arena: &Arena) -> &'static mut [u8] {
	///     let mut vec = Vec::new_in(arena);
	///     vec.push(1);
	///     vec.leak()
	/// }
	/// ```
	#[inline]
	pub fn leak(self) -> &'a mut [T]
	where
		A: 'a,
	{
		let mut me = ManuallyDrop::new(self);
		// SAFETY: the buffer holds `len` initialised elements that nothing else owns, and the allocator is never dropped, so they live for `'a`.
		unsafe { slice::from_raw_parts_mut(me.as_mut_ptr(), me.len()) }
	}

	/// Shrink the capacity to the length and convert into a boxed slice.
	pub fn into_boxed_slice(mut self) -> crate::Box<'a, [T], A> {
		self.shrink_to_fit();
		let (ptr, len, _, alloc) = self.into_parts_with_allocator();
		// SAFETY: `ptr` owns exactly `len` initialised `T`s allocated by `alloc`.
		unsafe { crate::Box::from_non_null_in(NonNull::slice_from_raw_parts(ptr, len), alloc) }
	}
}

impl<'a, T, A: Allocator, const N: usize> Vec<'a, [T; N], A> {
	/// Flatten a `Vec` of arrays into a `Vec` of their elements, reusing the buffer.
	///
	/// # Panics
	/// Panics if the flattened capacity exceeds `u32::MAX` elements.
	pub fn into_flattened(self) -> Vec<'a, T, A> {
		let scale = |count: usize| {
			count
				.checked_mul(N)
				.filter(|&scaled| scaled <= u32::MAX as usize)
				.unwrap_or_else(|| TryReserveError::CapacityOverflow.raise())
		};
		let (new_len, new_cap) = (scale(self.len()), scale(self.capacity()));
		let (ptr, _, _, alloc) = self.into_parts_with_allocator();
		// SAFETY: `[T; N]` is `N` consecutive `T`s, so the buffer holds `new_len` `T`s and was allocated as `new_cap` `T`s.
		unsafe { Vec::from_parts_in(ptr.cast::<T>(), new_len, new_cap, alloc) }
	}
}

impl<'a, T: Clone, A: Allocator> Vec<'a, T, A> {
	/// Resize to `new_len`, filling new slots with clones of `value`. Shrinking forgets the excess elements, like
	/// [`Vec::truncate`].
	pub fn resize(&mut self, new_len: usize, value: T) {
		let len = self.len();
		if new_len > len {
			self.extend(iter::repeat_n(value, new_len - len));
		} else {
			self.truncate(new_len);
		}
	}

	/// Append clones of all elements of `other`.
	pub fn extend_from_slice(&mut self, other: &[T]) {
		self.reserve(other.len());
		for (slot, value) in self.spare_capacity_mut().iter_mut().zip(other) {
			slot.write(value.clone());
		}
		// SAFETY: the reserve guarantees `other.len()` spare slots, and the loop initialised every one of them.
		unsafe { self.set_len(self.len() + other.len()) };
	}

	/// Append clones of the elements in `src`.
	///
	/// # Panics
	/// Panics if the range is out of bounds or its start is after its end.
	#[track_caller]
	pub fn extend_from_within<R: RangeBounds<usize>>(&mut self, src: R) {
		let range = slice_range(src, self.len());
		self.reserve(range.len());
		for index in range {
			let value = self[index].clone();
			self.push(value);
		}
	}
}

impl<'a, A: Allocator> Vec<'a, u8, A> {
	/// Append a copy of `bytes`. Slices of up to 16 bytes are copied as two overlapping fixed-size chunks instead of
	/// through `memcpy`.
	#[inline]
	pub fn extend_from_bytes(&mut self, bytes: &[u8]) {
		let n = bytes.len();
		self.reserve(n);
		let spare = &mut self.spare_capacity_mut()[..n];
		match n {
			0 => {}
			1..=3 => {
				spare[0].write(bytes[0]);
				spare[n / 2].write(bytes[n / 2]);
				spare[n - 1].write(bytes[n - 1]);
			}
			4..=7 => {
				spare[..4].write_copy_of_slice(&bytes[..4]);
				spare[n - 4..].write_copy_of_slice(&bytes[n - 4..]);
			}
			8..=16 => {
				spare[..8].write_copy_of_slice(&bytes[..8]);
				spare[n - 8..].write_copy_of_slice(&bytes[n - 8..]);
			}
			_ => {
				spare.write_copy_of_slice(bytes);
			}
		}
		// SAFETY: the reserve guarantees `n` spare slots, and every arm initialised all of them.
		unsafe { self.set_len(self.len() + n) };
	}
}

impl<'a, T, A: Allocator> Extend<T> for Vec<'a, T, A> {
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		let mut iterator = iter.into_iter();
		while let Some(element) = iterator.next() {
			if self.len() == self.capacity() {
				let (lower, _) = iterator.size_hint();
				self.reserve(lower.saturating_add(1));
			}
			self.push(element);
		}
	}
}

impl<'a, 'b, T: Copy + 'b, A: Allocator> Extend<&'b T> for Vec<'a, T, A> {
	fn extend<I: IntoIterator<Item = &'b T>>(&mut self, iter: I) {
		self.extend(iter.into_iter().copied());
	}
}

impl<'a, T, A: Allocator + Default> FromIterator<T> for Vec<'a, T, A> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		Self::from_iter_in(iter, A::default())
	}
}

impl<'a, T, A: Allocator + Default> Default for Vec<'a, T, A> {
	#[inline]
	fn default() -> Self {
		Self::new_in(A::default())
	}
}

impl<'a, T, A: Allocator> Deref for Vec<'a, T, A> {
	type Target = [T];

	#[inline]
	fn deref(&self) -> &[T] {
		// SAFETY: the buffer holds `len` initialised elements, or is dangling with `len` 0.
		unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
	}
}

impl<'a, T, A: Allocator> DerefMut for Vec<'a, T, A> {
	#[inline]
	fn deref_mut(&mut self) -> &mut [T] {
		// SAFETY: as in `deref`, and `&mut self` makes the access exclusive.
		unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
	}
}

impl<'a, T: Clone, A: Allocator + Clone> Clone for Vec<'a, T, A> {
	fn clone(&self) -> Self {
		let mut out = Vec::with_capacity_in(self.len(), self.alloc.clone());
		out.extend_from_slice(self);
		out
	}
}

impl<'a, T: fmt::Debug, A: Allocator> fmt::Debug for Vec<'a, T, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(&**self, f)
	}
}

impl<'a, T: Eq, A: Allocator> Eq for Vec<'a, T, A> {}

impl<'a, 'b, T: PartialOrd, A1: Allocator, A2: Allocator> PartialOrd<Vec<'b, T, A2>> for Vec<'a, T, A1> {
	#[inline]
	fn partial_cmp(&self, other: &Vec<'b, T, A2>) -> Option<std::cmp::Ordering> {
		PartialOrd::partial_cmp(&**self, &**other)
	}
}

impl<'a, T: Ord, A: Allocator> Ord for Vec<'a, T, A> {
	#[inline]
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		Ord::cmp(&**self, &**other)
	}
}

impl<'a, T, A: Allocator, I: slice::SliceIndex<[T]>> std::ops::Index<I> for Vec<'a, T, A> {
	type Output = I::Output;

	#[inline]
	fn index(&self, index: I) -> &Self::Output {
		std::ops::Index::index(&**self, index)
	}
}

impl<'a, T, A: Allocator, I: slice::SliceIndex<[T]>> std::ops::IndexMut<I> for Vec<'a, T, A> {
	#[inline]
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		std::ops::IndexMut::index_mut(&mut **self, index)
	}
}

impl<'a, T: Hash, A: Allocator> Hash for Vec<'a, T, A> {
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) {
		Hash::hash(&**self, state);
	}
}

impl<'a, T, A: Allocator> AsRef<[T]> for Vec<'a, T, A> {
	fn as_ref(&self) -> &[T] {
		self
	}
}

impl<'a, T, A: Allocator> AsMut<[T]> for Vec<'a, T, A> {
	fn as_mut(&mut self) -> &mut [T] {
		self
	}
}

impl<'a, T, A: Allocator> AsRef<Vec<'a, T, A>> for Vec<'a, T, A> {
	fn as_ref(&self) -> &Self {
		self
	}
}

impl<'a, T, A: Allocator> AsMut<Vec<'a, T, A>> for Vec<'a, T, A> {
	fn as_mut(&mut self) -> &mut Self {
		self
	}
}

impl<'a, T, A: Allocator> Borrow<[T]> for Vec<'a, T, A> {
	fn borrow(&self) -> &[T] {
		&self[..]
	}
}

impl<'a, T, A: Allocator> BorrowMut<[T]> for Vec<'a, T, A> {
	fn borrow_mut(&mut self) -> &mut [T] {
		&mut self[..]
	}
}

impl<'a, A: Allocator> std::io::Write for Vec<'a, u8, A> {
	#[inline]
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.extend_from_slice(buf);
		Ok(buf.len())
	}

	#[inline]
	fn flush(&mut self) -> std::io::Result<()> {
		Ok(())
	}
}

impl<'a, T, A: Allocator, const N: usize> TryFrom<Vec<'a, T, A>> for [T; N] {
	type Error = Vec<'a, T, A>;

	fn try_from(vec: Vec<'a, T, A>) -> Result<[T; N], Vec<'a, T, A>> {
		if vec.len() != N {
			return Err(vec);
		}
		let mut elements = vec.into_iter();
		Ok(std::array::from_fn(|_| elements.next().expect("the length check guarantees N elements")))
	}
}

impl<'a, T, A: Allocator> From<Vec<'a, T, A>> for crate::Box<'a, [T], A> {
	fn from(vec: Vec<'a, T, A>) -> Self {
		vec.into_boxed_slice()
	}
}

impl<'v, 'a, T, A: Allocator> IntoIterator for &'v Vec<'a, T, A> {
	type Item = &'v T;
	type IntoIter = slice::Iter<'v, T>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'v, 'a, T, A: Allocator> IntoIterator for &'v mut Vec<'a, T, A> {
	type Item = &'v mut T;
	type IntoIter = slice::IterMut<'v, T>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}

impl<'a, T: 'a, A: Allocator + 'a> IntoIterator for Vec<'a, T, A> {
	type Item = T;
	type IntoIter = IntoIter<'a, T>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		IntoIter::new(self.leak())
	}
}

#[cfg(feature = "serde")]
impl<'a, T: serde::Serialize, A: Allocator> serde::Serialize for Vec<'a, T, A> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.collect_seq(self.iter())
	}
}

#[cfg(feature = "serde")]
impl<'de, 'a, T: serde::Deserialize<'de>, A: Allocator + Default> serde::Deserialize<'de> for Vec<'a, T, A> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct SeqVisitor<'a, T, A: Allocator>(std::marker::PhantomData<Vec<'a, T, A>>);

		impl<'de, 'a, T: serde::Deserialize<'de>, A: Allocator + Default> serde::de::Visitor<'de> for SeqVisitor<'a, T, A> {
			type Value = Vec<'a, T, A>;

			fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.write_str("a sequence")
			}

			fn visit_seq<S: serde::de::SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
				const MAX_PREALLOCATED_BYTES: usize = 1024 * 1024;
				let hint = seq.size_hint().unwrap_or(0).min(MAX_PREALLOCATED_BYTES / size_of::<T>().max(1));
				let mut out = Vec::with_capacity_in(hint, A::default());
				while let Some(value) = seq.next_element()? {
					out.push(value);
				}
				Ok(out)
			}
		}

		deserializer.deserialize_seq(SeqVisitor(std::marker::PhantomData))
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
	fn extend_from_bytes_matches_extend_from_slice() {
		let alloc = Arena::new();
		let source: std::vec::Vec<u8> = (1..=40).collect();
		for prefix in 0..3 {
			for n in 0..=source.len() {
				let mut fast: Vec<u8> = Vec::new_in(&alloc);
				let mut slow: Vec<u8> = Vec::new_in(&alloc);
				fast.extend_from_slice(&source[..prefix]);
				slow.extend_from_slice(&source[..prefix]);
				fast.extend_from_bytes(&source[..n]);
				slow.extend_from_slice(&source[..n]);
				assert_eq!(&*fast, &*slow, "prefix {prefix}, len {n}");
			}
		}
	}

	#[test]
	#[should_panic(expected = "Vec capacity overflow")]
	fn extend_from_slice_past_u32_capacity_panics() {
		let alloc = Arena::new();
		let mut v: Vec<()> = Vec::new_in(&alloc);
		v.push(());
		v.extend_from_slice(&[(); u32::MAX as usize]);
	}

	#[test]
	#[should_panic(expected = "Vec capacity overflow")]
	fn extend_past_u32_capacity_panics() {
		let alloc = Arena::new();
		let mut v: Vec<()> = Vec::new_in(&alloc);
		v.push(());
		v.extend(std::iter::repeat_n((), u32::MAX as usize));
	}

	#[test]
	#[should_panic(expected = "Vec capacity overflow")]
	fn push_past_u32_capacity_panics() {
		let alloc = Arena::new();
		let mut v: Vec<()> = Vec::new_in(&alloc);
		v.raw.len = u32::MAX;
		v.raw.cap = u32::MAX;
		v.push(());
	}

	#[cfg(target_pointer_width = "64")]
	#[test]
	#[should_panic(expected = "Vec capacity overflow")]
	fn with_capacity_past_u32_panics() {
		let alloc = Arena::new();
		let _ = Vec::<()>::with_capacity_in(u32::MAX as usize + 1, &alloc);
	}

	#[test]
	fn grow_near_u32_capacity_saturates() {
		let alloc = Arena::new();
		let mut v: Vec<()> = Vec::new_in(&alloc);
		v.push(());
		v.raw.len = u32::MAX / 2 + 1;
		v.raw.cap = u32::MAX / 2 + 1;
		v.push(());
		assert_eq!(v.capacity(), u32::MAX as usize);
		assert_eq!(v.len(), u32::MAX as usize / 2 + 2);
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
		let bad_range = (Bound::Included(2usize), Bound::Excluded(1usize));
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

	#[test]
	fn swap_remove_moves_last_into_hole() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2, 3]);
		assert_eq!(v.swap_remove(1), 1);
		assert_eq!(v, [0, 3, 2]);
		assert_eq!(v.swap_remove(2), 2);
		assert_eq!(v, [0, 3]);
	}

	#[test]
	#[cfg(target_pointer_width = "64")]
	fn index_past_u32_is_rejected_not_truncated() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2]);
		let past_u32 = u32::MAX as usize + 1;
		v.truncate(past_u32);
		assert_eq!(v, [1, 2]);
		assert!(catch_unwind(AssertUnwindSafe(|| v.insert(past_u32, 3))).is_err());
		assert!(catch_unwind(AssertUnwindSafe(|| v.remove(past_u32))).is_err());
		assert_eq!(v, [1, 2]);
	}

	#[test]
	fn splice_with_shorter_replacement_closes_gap() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2, 3, 4]);
		let removed: std::vec::Vec<i32> = v.splice(1..4, [9]).collect();
		assert_eq!(removed, [1, 2, 3]);
		assert_eq!(v, [0, 9, 4]);
	}

	#[test]
	fn splice_with_longer_exact_size_replacement_moves_tail() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2]);
		drop(v.splice(1..2, [7, 8, 9, 10]));
		assert_eq!(v, [0, 7, 8, 9, 10, 2]);
	}

	#[test]
	fn splice_with_unknown_length_replacement_moves_tail() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2]);
		drop(v.splice(1..1, (5..9).filter(|n| n % 2 == 0)));
		assert_eq!(v, [0, 6, 8, 1, 2]);
		drop(v.splice(..0, (0..3).filter(|_| true)));
		assert_eq!(v, [0, 1, 2, 0, 6, 8, 1, 2]);
	}

	#[test]
	fn splice_at_end_appends() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2]);
		let removed: std::vec::Vec<i32> = v.splice(2.., [5, 6]).rev().collect();
		assert_eq!(removed, [2]);
		assert_eq!(v, [0, 1, 5, 6]);
	}

	#[test]
	fn splice_growing_past_later_allocation_keeps_tail() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::with_capacity_in(3, &alloc);
		v.extend([0, 1, 2]);
		let _later: Vec<i32> = Vec::with_capacity_in(1, &alloc);
		drop(v.splice(1..2, [7, 8, 9, 10]));
		assert_eq!(v, [0, 7, 8, 9, 10, 2]);
	}

	#[test]
	fn splice_panicking_replacement_keeps_written_items_and_tail() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2, 3]);
		let result = catch_unwind(AssertUnwindSafe(|| {
			drop(v.splice(1..3, (5..9).inspect(|&n| assert!(n != 7))));
		}));
		assert!(result.is_err());
		assert_eq!(v, [0, 5, 6, 3]);
	}

	#[test]
	fn splice_drops_unyielded_removed_elements_once() {
		let alloc = Arena::new();
		let drops = Rc::new(Cell::new(0));
		let mut v: Vec<DropCounter> = Vec::new_in(&alloc);
		v.extend((0..5).map(|id| DropCounter::new(id, &drops)));
		let replacement = DropCounter::new(9, &drops);
		{
			let mut splice = v.splice(1..4, [replacement]);
			assert_eq!(splice.next().map(|counter| counter.id), Some(1));
		}
		assert_eq!(drops.get(), 3);
		let ids: std::vec::Vec<u32> = v.iter().map(|counter| counter.id).collect();
		assert_eq!(ids, [0, 9, 4]);
	}

	#[test]
	fn extract_if_only_tests_range_and_keeps_order() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend(0..10);
		let extracted: std::vec::Vec<i32> = v.extract_if(2..8, |n| *n % 2 == 0).collect();
		assert_eq!(extracted, [2, 4, 6]);
		assert_eq!(v, [0, 1, 3, 5, 7, 8, 9]);
	}

	#[test]
	fn extract_if_dropped_early_keeps_unreached_elements() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend(0..6);
		let mut iter = v.extract_if(.., |n| *n % 2 == 1);
		assert_eq!(iter.next(), Some(1));
		drop(iter);
		assert_eq!(v, [0, 2, 3, 4, 5]);
	}

	#[test]
	fn extract_if_predicate_panic_drops_no_element_twice() {
		let alloc = Arena::new();
		let drops = Rc::new(Cell::new(0));
		let mut v: Vec<DropCounter> = Vec::new_in(&alloc);
		v.extend((0..5).map(|id| DropCounter::new(id, &drops)));
		let result = catch_unwind(AssertUnwindSafe(|| {
			v.extract_if(.., |counter| {
				assert!(counter.id != 3);
				counter.id == 1
			})
			.for_each(drop);
		}));
		assert!(result.is_err());
		assert_eq!(drops.get(), 1);
		let ids: std::vec::Vec<u32> = v.iter().map(|counter| counter.id).collect();
		assert_eq!(ids, [0, 2, 3, 4]);
	}

	#[test]
	fn dedup_by_key_collapses_runs_by_key() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([10, 11, 20, 25, 31, 12]);
		v.dedup_by_key(|n| *n / 10);
		assert_eq!(v, [10, 20, 31, 12]);
	}

	#[test]
	fn dedup_by_panic_drops_no_element_twice() {
		let alloc = Arena::new();
		let drops = Rc::new(Cell::new(0));
		let mut v: Vec<DropCounter> = Vec::new_in(&alloc);
		v.extend([0, 0, 1, 2, 2].into_iter().map(|id| DropCounter::new(id, &drops)));
		let result = catch_unwind(AssertUnwindSafe(|| {
			v.dedup_by(|current, previous| {
				assert!(current.id != 2);
				current.id == previous.id
			});
		}));
		assert!(result.is_err());
		assert_eq!(drops.get(), 0);
		let mut ids: std::vec::Vec<u32> = v.iter().map(|counter| counter.id).collect();
		assert_eq!(ids[..2], [0, 1]);
		ids.sort_unstable();
		assert_eq!(ids, [0, 0, 1, 2, 2]);
	}

	#[test]
	fn append_and_split_off_move_elements() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		let mut other: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1]);
		other.extend([2, 3, 4]);
		v.append(&mut other);
		assert!(other.is_empty());
		assert_eq!(v, [0, 1, 2, 3, 4]);
		let tail = v.split_off(3);
		assert_eq!(v, [0, 1, 2]);
		assert_eq!(tail, [3, 4]);
		assert!(v.split_off(3).is_empty());
	}

	#[test]
	fn resize_grows_with_clones_and_shrinks() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.resize(3, 7);
		assert_eq!(v, [7, 7, 7]);
		v.resize(1, 0);
		assert_eq!(v, [7]);
		let mut next = 0;
		v.resize_with(4, || {
			next += 1;
			next
		});
		assert_eq!(v, [7, 1, 2, 3]);
	}

	#[test]
	fn extend_from_within_clones_range_onto_end() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([0, 1, 2]);
		v.extend_from_within(1..);
		assert_eq!(v, [0, 1, 2, 1, 2]);
	}

	#[test]
	fn pop_if_only_pops_matching_last() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		assert_eq!(v.pop_if(|_| true), None);
		v.extend([1, 2]);
		assert_eq!(v.pop_if(|n| *n == 1), None);
		assert_eq!(v.pop_if(|n| *n == 2), Some(2));
		assert_eq!(v, [1]);
	}

	#[test]
	fn push_mut_and_insert_mut_return_the_new_element() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		*v.push_mut(1) += 10;
		*v.insert_mut(0, 2) += 20;
		assert_eq!(v, [22, 11]);
	}

	#[test]
	fn shrink_to_fit_returns_trailing_capacity_to_arena() {
		let alloc = Arena::new();
		let mut v: Vec<u64> = Vec::with_capacity_in(64, &alloc);
		v.extend([1, 2, 3]);
		let before = alloc.used_bytes();
		v.shrink_to(8);
		assert_eq!(v.capacity(), 8);
		v.shrink_to_fit();
		assert_eq!(v.capacity(), 3);
		assert_eq!(alloc.used_bytes(), before - 61 * size_of::<u64>());
		assert_eq!(v, [1, 2, 3]);
		v.push(4);
		assert_eq!(v, [1, 2, 3, 4]);
	}

	#[test]
	fn shrink_to_fit_keeps_capacity_of_earlier_allocations() {
		let alloc = Arena::new();
		let mut v: Vec<u64> = Vec::with_capacity_in(16, &alloc);
		v.push(1);
		let _later: Vec<u64> = Vec::with_capacity_in(4, &alloc);
		let before = alloc.used_bytes();
		v.shrink_to_fit();
		assert_eq!(v.capacity(), 1);
		assert_eq!(alloc.used_bytes(), before);
		assert_eq!(v, [1]);
	}

	#[test]
	fn try_reserve_reports_overflow_and_leaves_vec_usable() {
		let alloc = Arena::new();
		let mut v: Vec<u8> = Vec::new_in(&alloc);
		v.push(1);
		assert_eq!(v.try_reserve(u32::MAX as usize), Err(super::TryReserveError::CapacityOverflow));
		assert_eq!(v.try_reserve_exact(5), Ok(()));
		assert_eq!(v.capacity(), 6);
		assert_eq!(v, [1]);
	}

	#[test]
	fn try_reserve_reports_alloc_error_from_exhausted_arena() {
		let backing = Arena::new();
		// SAFETY: `backing` owns the region, outlives `borrowed`, and hands nothing out from it.
		let borrowed = unsafe { Arena::from_raw_parts(backing.base_ptr(), 64) };
		let mut v: Vec<u8> = Vec::new_in(&borrowed);
		assert_eq!(v.try_reserve_exact(128), Err(super::TryReserveError::AllocError));
		assert_eq!(v.try_reserve_exact(32), Ok(()));
		assert_eq!(v.capacity(), 32);
	}

	#[test]
	fn into_boxed_slice_holds_elements() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::with_capacity_in(8, &alloc);
		v.extend([1, 2, 3]);
		let boxed = v.into_boxed_slice();
		assert_eq!(&*boxed, &[1, 2, 3]);
	}

	#[test]
	fn into_flattened_concatenates_arrays() {
		let alloc = Arena::new();
		let mut v: Vec<[i32; 2]> = Vec::new_in(&alloc);
		v.extend([[1, 2], [3, 4]]);
		let mut flat = v.into_flattened();
		assert_eq!(flat, [1, 2, 3, 4]);
		flat.push(5);
		assert_eq!(flat, [1, 2, 3, 4, 5]);
	}

	#[test]
	fn leak_hands_out_mutable_slice() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2]);
		let slice = v.leak();
		slice[0] = 5;
		assert_eq!(slice, &[5, 2]);
	}

	#[test]
	fn try_into_array_requires_exact_length() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2, 3]);
		let v = <[i32; 2]>::try_from(v).unwrap_err();
		assert_eq!(<[i32; 3]>::try_from(v), Ok([1, 2, 3]));
	}

	#[test]
	fn drain_and_into_iter_iterate_from_both_ends() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend(0..6);
		let mut drain = v.drain(1..5);
		assert_eq!(drain.len(), 4);
		assert_eq!(drain.next_back(), Some(4));
		assert_eq!(drain.next(), Some(1));
		assert_eq!(drain.as_slice(), &[2, 3]);
		drop(drain);
		assert_eq!(v, [0, 5]);
		let mut iter = v.into_iter();
		assert_eq!(iter.next_back(), Some(5));
		assert_eq!(iter.as_slice(), &[0]);
		assert_eq!(iter.next(), Some(0));
		assert_eq!(iter.next_back(), None);
	}

	#[test]
	fn drain_keep_rest_restores_unyielded_elements() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend(0..6);
		let mut drain = v.drain(1..5);
		assert_eq!(drain.next(), Some(1));
		assert_eq!(drain.next_back(), Some(4));
		drain.keep_rest();
		assert_eq!(v, [0, 2, 3, 5]);
	}

	#[test]
	fn compares_with_slices_arrays_and_std_vecs() {
		let alloc = Arena::new();
		let mut v: Vec<i32> = Vec::new_in(&alloc);
		v.extend([1, 2]);
		assert!(v == [1, 2]);
		assert!(v == [1, 2][..]);
		assert!(v == vec![1, 2]);
		assert!(vec![1, 2] == v);
		assert!([1, 2][..] == v);
		let deque = std::collections::VecDeque::from([1, 2]);
		assert!(deque == v);
		let borrowed: std::borrow::Cow<'_, [i32]> = std::borrow::Cow::Borrowed(&[1, 2]);
		assert!(borrowed == v);
		let mut array = [1, 2];
		let mutable_slice: &mut [i32] = &mut array;
		assert!(mutable_slice == v);
		assert!(v != [1, 3]);
	}

	#[test]
	fn collects_with_default_allocator() {
		let v: Vec<i32, std::alloc::Global> = (0..3).collect();
		assert_eq!(v, [0, 1, 2]);
		assert!(Vec::<i32, std::alloc::Global>::default().is_empty());
	}

	#[test]
	fn io_write_appends_bytes() {
		use std::io::Write;
		let alloc = Arena::new();
		let mut v: Vec<u8> = Vec::new_in(&alloc);
		write!(v, "{}-{}", 1, 2).unwrap();
		assert_eq!(v, *b"1-2");
	}

	#[cfg(feature = "serde")]
	#[test]
	fn deserializes_sequence_with_default_allocator() {
		use serde::Deserialize;
		use serde::de::value::{Error, SeqDeserializer};
		let deserializer = SeqDeserializer::<_, Error>::new([1, 2, 3].into_iter());
		let v = Vec::<i32, std::alloc::Global>::deserialize(deserializer).unwrap();
		assert_eq!(v, [1, 2, 3]);
	}

	#[test]
	fn drain_of_zero_sized_values_drops_unyielded_and_keeps_tail() {
		thread_local! {
			static ZST_DROPS: Cell<u32> = const { Cell::new(0) };
		}
		struct Zst;
		impl Drop for Zst {
			fn drop(&mut self) {
				ZST_DROPS.with(|drops| drops.set(drops.get() + 1));
			}
		}
		let alloc = Arena::new();
		let mut v: Vec<Zst> = Vec::new_in(&alloc);
		v.extend((0..5).map(|_| Zst));
		let mut drain = v.drain(1..4);
		drop(drain.next());
		drop(drain);
		assert_eq!(ZST_DROPS.with(Cell::get), 3);
		assert_eq!(v.len(), 2);
	}
}
