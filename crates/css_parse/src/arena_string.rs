use crate::{Arena, Vec};
use allocator_api2::alloc::Allocator;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// A growable, arena-allocated UTF-8 string.
///
/// Wraps [`Vec<u8>`][crate::Vec] the same way [`std::string::String`] wraps [`std::vec::Vec`], so it inherits the
/// allocator generic from [Vec] too. Like the other arena collections it never runs destructors: the bytes are
/// released wholesale when the arena is dropped.
///
/// The contents are always valid UTF-8: the only ways to append are [`String::push_str`], [`String::push`] and the
/// [`fmt::Write`] impl, all of which take a `str` or a `char`, and nothing exposes the bytes mutably or truncates
/// them. [`String::as_str`] and [`String::into_str`] rely on that invariant to hand out a `str` without revalidating.
#[repr(C)]
pub struct String<'a, A: Allocator = &'a Arena> {
	bytes: Vec<'a, u8, A>,
}

impl<'a, A: Allocator> String<'a, A> {
	/// Create a new, empty `String` backed by `alloc`. Allocates nothing until the first push.
	#[inline]
	pub fn new_in(alloc: A) -> Self {
		Self { bytes: Vec::new_in(alloc) }
	}

	/// Create an empty `String` with room for at least `cap` bytes.
	#[inline]
	pub fn with_capacity_in(cap: usize, alloc: A) -> Self {
		Self { bytes: Vec::with_capacity_in(cap, alloc) }
	}

	/// Length in bytes, not characters.
	#[inline]
	pub fn len(&self) -> usize {
		self.bytes.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.bytes.is_empty()
	}

	/// Capacity in bytes.
	#[inline]
	pub fn capacity(&self) -> usize {
		self.bytes.capacity()
	}

	/// Append a string slice.
	#[inline]
	pub fn push_str(&mut self, str: &str) {
		self.bytes.extend_from_slice(str.as_bytes());
	}

	/// Append a single character, encoded as UTF-8.
	#[inline]
	pub fn push(&mut self, char: char) {
		let mut buf = [0; 4];
		self.push_str(char.encode_utf8(&mut buf));
	}

	/// Borrow the contents as a `str`.
	#[inline]
	pub fn as_str(&self) -> &str {
		Self::as_utf8(&self.bytes)
	}

	/// Consume the `String`, returning a `str` borrowed from the arena for `'a`.
	///
	/// Use this to hand a parsed-in-arena string to an API that wants `&'a str`; the bytes outlive the `String` because
	/// they belong to the arena, not to this handle.
	#[inline]
	pub fn into_str(self) -> &'a str {
		Self::as_utf8(self.bytes.into_slice())
	}

	#[inline]
	fn as_utf8(bytes: &[u8]) -> &str {
		debug_assert!(std::str::from_utf8(bytes).is_ok(), "arena String must always hold valid UTF-8");
		// SAFETY: the buffer only ever grows through `push_str`, `push` and the `fmt::Write` impl, each of which appends a
		// `str` or a UTF-8 encoded `char`. Nothing hands the bytes out mutably or truncates them.
		unsafe { std::str::from_utf8_unchecked(bytes) }
	}
}

impl<'a, A: Allocator> Deref for String<'a, A> {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

impl<'a, A: Allocator> AsRef<str> for String<'a, A> {
	#[inline]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl<'a, A: Allocator> fmt::Write for String<'a, A> {
	#[inline]
	fn write_str(&mut self, str: &str) -> fmt::Result {
		self.push_str(str);
		Ok(())
	}
}

impl<'a, A: Allocator> fmt::Display for String<'a, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(self.as_str(), f)
	}
}

impl<'a, A: Allocator> fmt::Debug for String<'a, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self.as_str(), f)
	}
}

impl<'a, A: Allocator> PartialEq for String<'a, A> {
	fn eq(&self, other: &Self) -> bool {
		**self == **other
	}
}

impl<'a, A: Allocator> Eq for String<'a, A> {}

impl<'a, A: Allocator> PartialEq<str> for String<'a, A> {
	fn eq(&self, other: &str) -> bool {
		&**self == other
	}
}

impl<'a, A: Allocator> PartialEq<&str> for String<'a, A> {
	fn eq(&self, other: &&str) -> bool {
		&**self == *other
	}
}

impl<'a, A: Allocator> Hash for String<'a, A> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(**self).hash(state);
	}
}

/// A [`std::format!`]-style constructor for the arena [`String`].
///
/// ```
/// use css_parse::{Arena, format_in};
/// let alloc = Arena::default();
/// let str = format_in!(in &alloc, "{}px", 12);
/// assert_eq!(str.as_str(), "12px");
/// ```
#[macro_export]
macro_rules! format_in {
	(in $alloc:expr, $($arg:tt)*) => {{
		let mut str = $crate::String::new_in($alloc);
		::core::fmt::Write::write_fmt(&mut str, ::core::format_args!($($arg)*))
			.expect("formatting into an arena String cannot fail");
		str
	}};
}

#[cfg(test)]
mod test {
	use super::String;
	use crate::Arena;
	use std::fmt::Write;

	#[test]
	fn new_is_empty_and_allocates_nothing() {
		let alloc = Arena::default();
		let str = String::new_in(&alloc);
		assert!(str.is_empty());
		assert_eq!(str.len(), 0);
		assert_eq!(str.as_str(), "");
	}

	#[test]
	fn push_str_appends_in_order() {
		let alloc = Arena::default();
		let mut str = String::new_in(&alloc);
		str.push_str("foo");
		str.push_str("bar");
		assert_eq!(str.as_str(), "foobar");
		assert_eq!(str.len(), 6);
	}

	#[test]
	fn push_encodes_multibyte_chars() {
		let alloc = Arena::default();
		let mut str = String::new_in(&alloc);
		str.push('a');
		str.push('£');
		str.push('😀');
		assert_eq!(str.as_str(), "a£😀");
		// 1 + 2 + 4 bytes, so length counts bytes rather than characters.
		assert_eq!(str.len(), 7);
	}

	#[test]
	fn write_macro_formats_through_fmt_write() {
		let alloc = Arena::default();
		let mut str = String::new_in(&alloc);
		write!(&mut str, "{}px", 12).unwrap();
		assert_eq!(str.as_str(), "12px");
	}

	#[test]
	fn format_in_builds_a_string() {
		let alloc = Arena::default();
		let str = format_in!(in &alloc, "{}{}", 12, "px");
		assert_eq!(str, "12px");
	}

	#[test]
	fn into_str_outlives_the_string() {
		let alloc = Arena::default();
		let borrowed: &str = {
			let mut str = String::new_in(&alloc);
			str.push_str("outlives");
			str.into_str()
		};
		assert_eq!(borrowed, "outlives");
	}

	#[test]
	fn survives_reallocation() {
		let alloc = Arena::default();
		let mut str = String::new_in(&alloc);
		for i in 0..512 {
			write!(&mut str, "{}", i % 10).unwrap();
		}
		// Every byte must survive the regrowth, so compare against the whole expected sequence.
		let expected: std::string::String = (0..512).map(|i| char::from(b'0' + (i % 10) as u8)).collect();
		assert_eq!(str.len(), 512);
		assert_eq!(str.as_str(), expected);
	}

	#[test]
	fn with_capacity_reserves_without_writing() {
		let alloc = Arena::default();
		let mut str = String::with_capacity_in(16, &alloc);
		assert!(str.is_empty());
		assert!(str.capacity() >= 16);
		str.push_str("fits");
		assert_eq!(str.as_str(), "fits");
	}

	#[test]
	fn deref_exposes_str_methods() {
		let alloc = Arena::default();
		let mut str = String::new_in(&alloc);
		str.push_str("  padded  ");
		assert_eq!(str.trim(), "padded");
		assert!(str.contains("padded"));
	}

	#[test]
	fn equality_against_str_and_self() {
		let alloc = Arena::default();
		let mut a = String::new_in(&alloc);
		a.push_str("same");
		let mut b = String::new_in(&alloc);
		b.push_str("same");
		assert_eq!(a, b);
		assert_eq!(a, "same");
		assert_eq!(a, *"same");
	}
}
