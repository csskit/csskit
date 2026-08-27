#![doc = include_str!("../README.md")]

use fearless_simd::{Level, dispatch};
pub use fearless_simd::{Simd, SimdBase, SimdInt, SimdMask, u8x16};

/// A `u8` SIMD vector of any width the scanner runs at.
pub trait ByteVector<S: Simd>: SimdInt<S, Element = u8> {}

impl<S: Simd, V: SimdInt<S, Element = u8>> ByteVector<S> for V {}

/// A byte predicate.
///
/// You probably want to declare a class with [`byte_class!`], rather than by hand; write the impl by hand only for a
/// predicate the macro cannot express. A hand written impl must classify a byte identically in [`ByteClass::matches`]
/// and [`ByteClass::mask`]. The scanner uses the SIMD form for whole vectors and the scalar form for the remainder, so
/// both can potentially be called.
pub trait ByteClass: Copy {
	/// Return `true` if the byte is in the class.
	fn matches(self, byte: u8) -> bool;

	/// Return a lane mask with one lane set for each byte of `value` in the class.
	///
	/// This is generic over the vector width because the scanner runs the class at the native width of the host and then
	/// at 128 bits for the remainder.
	fn mask<S: Simd, V: ByteVector<S>>(self, value: V) -> V::Mask;
}

/// Declare a [`ByteClass`] from a set of bytes.
///
/// The set is a `|` separated list of terms to match against. Each term is one of:
///
/// | Term                | Matches                           |
/// | ------------------- | --------------------------------- |
/// | `b'x'`, `0x7F`, `0` | that one byte                     |
/// | `0x80..=0xFF`       | that inclusive range              |
/// | `alpha`             | ASCII letters of either case      |
/// | `!(…)`              | every byte outside the nested set |
/// | a field name        | the byte held in that field       |
///
///
/// ```
/// use bytescan::{byte_class, scan_until};
///
/// byte_class! {
///     /// Bytes that end an identifier.
///     pub struct IdentEnd = !(alpha | b'0'..=b'9' | b'-' | b'_');
/// }
///
/// assert_eq!(scan_until(b"name-1:value", IdentEnd), 6);
/// ```
///
/// Field names can be used to hold data only where the language needs it, such as the quote delimiter of a string. For
/// example, add one field in the struct to hold a byte, and use that name as a term:
///
/// ```
/// use bytescan::{byte_class, scan_until};
///
/// byte_class! {
///     /// Bytes that end a string body.
///     pub struct StringEnd(delimiter) = delimiter | b'\\' | b'\n' | 0x80..=0xFF;
/// }
///
/// assert_eq!(scan_until(b"say 'hi'", StringEnd(b'\'')), 4);
/// ```
#[macro_export]
macro_rules! byte_class {
	($(#[$meta:meta])* $vis:vis struct $name:ident = $($set:tt)+) => {
		$(#[$meta])*
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		$vis struct $name;

		impl $crate::ByteClass for $name {
			#[inline(always)]
			fn matches(self, byte: u8) -> bool {
				$crate::__byte_class_scalar!(byte, $($set)+)
			}

			#[inline(always)]
			fn mask<S: $crate::Simd, V: $crate::ByteVector<S>>(self, value: V) -> V::Mask {
				$crate::__byte_class_vector!(S, V, value, $($set)+)
			}
		}
	};
	($(#[$meta:meta])* $vis:vis struct $name:ident($field:ident) = $($set:tt)+) => {
		$(#[$meta])*
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		$vis struct $name(pub u8);

		impl $crate::ByteClass for $name {
			#[inline(always)]
			fn matches(self, byte: u8) -> bool {
				let $field = self.0;
				$crate::__byte_class_scalar!(byte, $($set)+)
			}

			#[inline(always)]
			fn mask<S: $crate::Simd, V: $crate::ByteVector<S>>(self, value: V) -> V::Mask {
				let $field = self.0;
				$crate::__byte_class_vector!(S, V, value, $($set)+)
			}
		}
	};
}

/// Implementation detail of [`byte_class!`]; this is not public API.
#[doc(hidden)]
#[macro_export]
macro_rules! __byte_class_scalar {
	($b:ident, !($($set:tt)+) | $($rest:tt)+) => {
		!$crate::__byte_class_scalar!($b, $($set)+) || $crate::__byte_class_scalar!($b, $($rest)+)
	};
	($b:ident, !($($set:tt)+) $(;)?) => { !$crate::__byte_class_scalar!($b, $($set)+) };
	($b:ident, alpha | $($rest:tt)+) => {
		$b.is_ascii_alphabetic() || $crate::__byte_class_scalar!($b, $($rest)+)
	};
	($b:ident, alpha $(;)?) => { ($b.is_ascii_alphabetic()) };
	($b:ident, $lo:literal ..= $hi:literal | $($rest:tt)+) => {
		$b.wrapping_sub($lo) <= $hi - $lo || $crate::__byte_class_scalar!($b, $($rest)+)
	};
	($b:ident, $lo:literal ..= $hi:literal $(;)?) => { ($b.wrapping_sub($lo) <= $hi - $lo) };
	($b:ident, $byte:literal | $($rest:tt)+) => {
		$b == $byte || $crate::__byte_class_scalar!($b, $($rest)+)
	};
	($b:ident, $byte:literal $(;)?) => { ($b == $byte) };
	($b:ident, $field:ident | $($rest:tt)+) => {
		$b == $field || $crate::__byte_class_scalar!($b, $($rest)+)
	};
	($b:ident, $field:ident $(;)?) => { ($b == $field) };
}

/// Implementation detail of [`byte_class!`]; this is not public API.
#[doc(hidden)]
#[macro_export]
macro_rules! __byte_class_vector {
	($s:ident, $v:ident, $value:ident, !($($set:tt)+) | $($rest:tt)+) => {
		!$crate::__byte_class_vector!($s, $v, $value, $($set)+)
			| $crate::__byte_class_vector!($s, $v, $value, $($rest)+)
	};
	($s:ident, $v:ident, $value:ident, !($($set:tt)+) $(;)?) => {
		!$crate::__byte_class_vector!($s, $v, $value, $($set)+)
	};
	($s:ident, $v:ident, $value:ident, alpha | $($rest:tt)+) => {
		$crate::range_mask::<$s, $v, { b'a' }, { b'z' }>($value | 0x20)
			| $crate::__byte_class_vector!($s, $v, $value, $($rest)+)
	};
	($s:ident, $v:ident, $value:ident, alpha $(;)?) => {
		$crate::range_mask::<$s, $v, { b'a' }, { b'z' }>($value | 0x20)
	};
	($s:ident, $v:ident, $value:ident, $lo:literal ..= $hi:literal | $($rest:tt)+) => {
		$crate::range_mask::<$s, $v, { $lo }, { $hi }>($value)
			| $crate::__byte_class_vector!($s, $v, $value, $($rest)+)
	};
	($s:ident, $v:ident, $value:ident, $lo:literal ..= $hi:literal $(;)?) => {
		$crate::range_mask::<$s, $v, { $lo }, { $hi }>($value)
	};
	($s:ident, $v:ident, $value:ident, $byte:literal | $($rest:tt)+) => {
		$crate::byte_mask($value, $byte) | $crate::__byte_class_vector!($s, $v, $value, $($rest)+)
	};
	($s:ident, $v:ident, $value:ident, $byte:literal $(;)?) => { $crate::byte_mask($value, $byte) };
	($s:ident, $v:ident, $value:ident, $field:ident | $($rest:tt)+) => {
		$crate::byte_mask($value, $field) | $crate::__byte_class_vector!($s, $v, $value, $($rest)+)
	};
	($s:ident, $v:ident, $value:ident, $field:ident $(;)?) => { $crate::byte_mask($value, $field) };
}

/// Return a lane mask with one lane set for each byte of `value` equal to `byte`.
#[inline(always)]
pub fn byte_mask<S: Simd, V: ByteVector<S>>(value: V, byte: u8) -> V::Mask {
	value.simd_eq(byte)
}

/// Return a lane mask with one lane set for each byte of `value` in the inclusive range `LO..=HI`.
#[inline(always)]
pub fn range_mask<S: Simd, V: ByteVector<S>, const LO: u8, const HI: u8>(value: V) -> V::Mask {
	if LO == u8::MIN {
		value.simd_le(HI)
	} else if HI == u8::MAX {
		value.simd_ge(LO)
	} else {
		value.simd_ge(LO) & value.simd_le(HI)
	}
}

/// Scan for the first byte matched by `stop`, or return `bytes.len()` if absent.
#[inline]
pub fn scan_until<C: ByteClass>(bytes: &[u8], stop: C) -> usize {
	scan_until_and_mark(bytes, stop, NoBytes).0
}

/// Scan for the first byte matched by `stop` and report whether `mark` matched any preceding byte.
#[inline]
pub fn scan_until_and_mark<C: ByteClass, M: ByteClass>(bytes: &[u8], stop: C, mark: M) -> (usize, bool) {
	dispatch!(Level::new(), simd => scan(simd, bytes, stop, mark))
}

/// Scan for the first occurrence of `needle`, or return `bytes.len()` if absent.
#[inline]
pub fn scan_byte(bytes: &[u8], needle: u8) -> usize {
	scan_until(bytes, Byte(needle))
}

#[derive(Clone, Copy)]
struct NoBytes;

impl ByteClass for NoBytes {
	#[inline(always)]
	fn matches(self, _byte: u8) -> bool {
		false
	}

	#[inline(always)]
	fn mask<S: Simd, V: ByteVector<S>>(self, value: V) -> V::Mask {
		V::Mask::splat(value.witness(), false)
	}
}

byte_class! {
	struct Byte(needle) = needle;
}

#[inline(always)]
fn scan_scalar<C: ByteClass, M: ByteClass>(bytes: &[u8], stop: C, mark: M) -> (usize, bool) {
	let mut marked = false;
	for (offset, &byte) in bytes.iter().enumerate() {
		if stop.matches(byte) {
			return (offset, marked);
		}
		marked |= mark.matches(byte);
	}
	(bytes.len(), marked)
}

/// Scan whole `V`-wide vectors, reporting `(offset, marked, stopped)`.
///
/// `offset` is the stopping byte when `stopped`, and the number of scanned bytes otherwise.
#[inline(always)]
fn scan_vectors<S: Simd, V: ByteVector<S>, C: ByteClass, M: ByteClass>(
	simd: S,
	bytes: &[u8],
	stop: C,
	mark: M,
) -> (usize, bool, bool) {
	let mut marked = false;
	let mut offset = 0;
	#[allow(clippy::chunks_exact_to_as_chunks)] // V::N is a generic const, as_chunks needs generic_const_exprs
	for chunk in bytes.chunks_exact(V::N) {
		let value = V::from_slice(simd, chunk);
		let stop_mask = stop.mask(value).to_bitmask();
		let mark_mask = mark.mask(value).to_bitmask();
		if stop_mask != 0 {
			let lane = stop_mask.trailing_zeros() as usize;
			marked |= (mark_mask & ((1u64 << lane) - 1)) != 0;
			return (offset + lane, marked, true);
		}
		marked |= mark_mask != 0;
		offset += V::N;
	}
	(offset, marked, false)
}

#[inline(always)]
fn scan<S: Simd, C: ByteClass, M: ByteClass>(simd: S, bytes: &[u8], stop: C, mark: M) -> (usize, bool) {
	let (mut offset, mut marked, stopped) = scan_vectors::<S, S::u8s, C, M>(simd, bytes, stop, mark);
	if stopped {
		return (offset, marked);
	}
	if <S::u8s as SimdBase<S>>::N > <u8x16<S> as SimdBase<S>>::N {
		let (block, block_marked, stopped) = scan_vectors::<S, u8x16<S>, C, M>(simd, &bytes[offset..], stop, mark);
		offset += block;
		marked |= block_marked;
		if stopped {
			return (offset, marked);
		}
	}
	let (tail, tail_marked) = scan_scalar(&bytes[offset..], stop, mark);
	(offset + tail, marked | tail_marked)
}

#[cfg(test)]
mod test {
	use super::*;

	byte_class! {
		struct Angle = b'<' | b'>';
	}

	byte_class! {
		struct IdentEnd = !(alpha | b'0'..=b'9' | b'-' | b'_');
	}

	byte_class! {
		struct AsciiUpper = b'A'..=b'Z';
	}

	byte_class! {
		struct StringEnd(delimiter) = delimiter | b'\\' | b'\n' | 0x80..=0xFF;
	}

	#[test]
	fn matches_agrees_with_mask_over_every_byte() {
		for byte in 0..=u8::MAX {
			let input = [byte; 64];
			let stopped = scan_until(&input, IdentEnd) == 0;
			assert_eq!(stopped, IdentEnd.matches(byte), "ident {byte:#04x}");
			let stopped = scan_until(&input, StringEnd(b'"')) == 0;
			assert_eq!(stopped, StringEnd(b'"').matches(byte), "string {byte:#04x}");
		}
	}

	#[test]
	fn finds_a_byte() {
		assert_eq!(scan_byte(b"", b'<'), 0);
		assert_eq!(scan_byte(b"abc", b'<'), 3);
		assert_eq!(scan_byte(b"ab<c", b'<'), 2);
	}

	#[test]
	fn finds_a_byte_past_the_first_lane() {
		let long = format!("{}<x", "y".repeat(40));
		assert_eq!(scan_byte(long.as_bytes(), b'<'), 40);
		let in_tail = format!("{}<x", "y".repeat(35));
		assert_eq!(scan_byte(in_tail.as_bytes(), b'<'), 35);
		let absent = "y".repeat(40);
		assert_eq!(scan_byte(absent.as_bytes(), b'<'), 40);
	}

	#[test]
	fn scans_until_any_byte_of_a_set() {
		assert_eq!(scan_until(b"abcd", Angle), 4);
		assert_eq!(scan_until(b"ab<cd", Angle), 2);
		assert_eq!(scan_until(b"ab>cd", Angle), 2);
		assert_eq!(scan_until(b"ab>cd<ef", Angle), 2);
	}

	#[test]
	fn scans_every_vector_boundary() {
		for offset in 0..=130 {
			let input = format!("{}<x", "y".repeat(offset));
			assert_eq!(scan_until(input.as_bytes(), Angle), offset, "stop at {offset}");
			let absent = "y".repeat(offset);
			assert_eq!(scan_until(absent.as_bytes(), Angle), offset, "absent at {offset}");
		}
	}

	#[test]
	fn scans_until_a_class() {
		assert_eq!(scan_until(b"Name-1:value", IdentEnd), 6);
		assert_eq!(scan_until(b"ascii\xc3\xa9", StringEnd(b'"')), 5);
		assert_eq!(scan_until(b"0123456789abcdefghij\xc3\xa9", StringEnd(b'"')), 20);
		assert_eq!(scan_until(b"it's", StringEnd(b'\'')), 2);
		assert_eq!(scan_until(b"it's", StringEnd(b'"')), 4);
	}

	#[test]
	fn marks_only_bytes_before_the_stop() {
		assert_eq!(scan_until_and_mark(b"lowerCase:value", Byte(b':'), AsciiUpper), (9, true));
		assert_eq!(scan_until_and_mark(b"lower:valueCase", Byte(b':'), AsciiUpper), (5, false));
		for offset in 0..=130 {
			let input = format!("{}A:value", "x".repeat(offset));
			assert_eq!(
				scan_until_and_mark(input.as_bytes(), Byte(b':'), AsciiUpper),
				(offset + 1, true),
				"mark before stop at {offset}"
			);
			let after = format!("{}:valueA", "x".repeat(offset));
			assert_eq!(
				scan_until_and_mark(after.as_bytes(), Byte(b':'), AsciiUpper),
				(offset, false),
				"mark after stop at {offset}"
			);
		}
	}
}
