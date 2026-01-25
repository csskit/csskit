use crate::{
	AssociatedWhitespaceRules, CommentStyle, CowStr, Cursor, Kind, KindSet, QuoteStyle, SourceOffset, Span, ToSpan,
	Token,
	small_str_buf::SmallStrBuf,
	syntax::{ParseEscape, is_newline},
};
use allocator_api2::{alloc::Allocator, boxed::Box, vec::Vec};
use std::char::REPLACEMENT_CHARACTER;
use std::fmt::{Display, Formatter, Result, Write};

/// Wraps [Cursor] with a [str] that represents the underlying character data for this cursor.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceCursor<'a> {
	cursor: Cursor,
	source: &'a str,
	should_compact: bool,
}

impl<'a> ToSpan for SourceCursor<'a> {
	fn to_span(&self) -> Span {
		self.cursor.to_span()
	}
}

impl<'a> Display for SourceCursor<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self.token().kind() {
			Kind::Eof => Ok(()),
			// It is important to manually write out quotes for 2 reasons:
			//  1. The quote style can be mutated from the source string (such as the case of normalising/switching quotes.
			//  2. Some strings may not have the closing quote, which should be corrected.
			Kind::String => match self.token().quote_style() {
				QuoteStyle::Single => {
					let inner =
						&self.source[1..(self.token().len() as usize) - self.token().has_close_quote() as usize];
					write!(f, "'{inner}'")
				}
				QuoteStyle::Double => {
					let inner =
						&self.source[1..(self.token().len() as usize) - self.token().has_close_quote() as usize];
					write!(f, "\"{inner}\"")
				}
				// Strings must always be quoted!
				QuoteStyle::None => unreachable!(),
			},
			Kind::Delim
			| Kind::Colon
			| Kind::Semicolon
			| Kind::Comma
			| Kind::LeftSquare
			| Kind::LeftParen
			| Kind::RightSquare
			| Kind::RightParen
			| Kind::LeftCurly
			| Kind::RightCurly => self.token().char().unwrap().fmt(f),
			_ if self.should_compact => self.fmt_compacted(f),
			_ => f.write_str(self.source),
		}
	}
}

impl<'a> SourceCursor<'a> {
	pub const SPACE: SourceCursor<'static> = SourceCursor::from(Cursor::new(SourceOffset(0), Token::SPACE), " ");
	pub const TAB: SourceCursor<'static> = SourceCursor::from(Cursor::new(SourceOffset(0), Token::TAB), "\t");
	pub const NEWLINE: SourceCursor<'static> = SourceCursor::from(Cursor::new(SourceOffset(0), Token::NEWLINE), "\n");

	#[inline(always)]
	pub const fn from(cursor: Cursor, source: &'a str) -> Self {
		debug_assert!(
			(cursor.len() as usize) == source.len(),
			"A SourceCursor should be constructed with a source that matches the length of the cursor!"
		);
		Self { cursor, source, should_compact: false }
	}

	#[inline(always)]
	pub const fn cursor(&self) -> Cursor {
		self.cursor
	}

	#[inline(always)]
	pub const fn token(&self) -> Token {
		self.cursor.token()
	}

	#[inline(always)]
	pub const fn source(&self) -> &'a str {
		self.source
	}

	pub fn with_quotes(&self, quote_style: QuoteStyle) -> Self {
		Self { cursor: self.cursor.with_quotes(quote_style), source: self.source, should_compact: self.should_compact }
	}

	pub fn with_associated_whitespace(&self, rules: AssociatedWhitespaceRules) -> Self {
		Self {
			cursor: self.cursor.with_associated_whitespace(rules),
			source: self.source,
			should_compact: self.should_compact,
		}
	}

	/// Returns a new `SourceCursor` with the `should_compact` flag set.
	///
	/// With the `should_compact` flag set, the cursor will format with optimised displays of:
	/// - Numbers: Remove leading zeros (`0.8` -> `.8`), trailing zeros (`1.0` -> `1`), redundant `+` sign
	/// - Idents/Functions/AtKeywords: Write UTF-8 instead of escape codes
	/// - Dimensions: Same as numbers for the number part, and same as Idents for the unit part
	/// - Whitespace: Normalize to a single space
	///
	pub fn compact(&self) -> SourceCursor<'a> {
		Self { cursor: self.cursor, source: self.source, should_compact: true }
	}

	/// Checks if calling `compact().fmt(..)` _might_ produce different output than `fmt(..)`.
	///
	/// This can be used to check, rather than a full allocation & display, e.g. `format!("{}", sc.compact())`.
	///
	/// - Whitespace: returns `true` if len > 1
	/// - Ident/Function/AtKeyword/Hash: returns `true` if contains escape chars
	/// - Number: returns `true` if the number representation could be shortened
	/// - Dimension: combines number and unit checks
	#[inline]
	pub fn may_compact(&self) -> bool {
		let token = self.token();
		match token.kind() {
			Kind::Whitespace => token.len() > 1,
			Kind::Ident | Kind::Function | Kind::AtKeyword | Kind::Hash => token.contains_escape_chars(),
			Kind::Number => self.can_compact_number(),
			Kind::Dimension => {
				self.can_compact_number()
					|| self.source[(self.token().numeric_len() as usize)..].bytes().any(|b| b == b'\\' || b == 0)
			}
			_ => false,
		}
	}

	/// Check if the numeric value could be compacted.
	#[inline]
	fn can_compact_number(&self) -> bool {
		let token = self.token();
		let value = token.value();
		let num_len = token.numeric_len() as usize;
		if value > -1.0 && value < 1.0 && value != 0.0 {
			let bytes = self.source.as_bytes();
			if bytes.first() == Some(&b'.') {
				return false;
			}
			if value < 0.0 && bytes.get(1) == Some(&b'.') {
				return false;
			}
			return true;
		}
		if token.has_sign() && value > 0.0 {
			return true;
		}
		if token.is_float() && value.fract() == 0.0 {
			return true;
		}
		if token.is_int() {
			let abs_value = value.abs();
			let digits = if abs_value == 0.0 { 1 } else { (abs_value.log10().floor() as usize) + 1 };
			return num_len > (digits + (value < 0.0) as usize);
		}
		false
	}

	fn fmt_compacted(&self, f: &mut Formatter<'_>) -> Result {
		let token = self.token();
		match token.kind() {
			Kind::Whitespace => f.write_str(" "),
			Kind::Ident | Kind::Function | Kind::AtKeyword | Kind::Hash
				if self.should_compact && token.contains_escape_chars() =>
			{
				self.fmt_compacted_ident(f)
			}
			Kind::Number => self.fmt_compacted_number(f),
			Kind::Dimension => {
				self.fmt_compacted_number(f)?;
				self.fmt_compacted_ident(f)
			}
			_ => f.write_str(self.source),
		}
	}

	fn fmt_compacted_number(&self, f: &mut Formatter<'_>) -> Result {
		let value = self.token().value();
		if value <= -1.0 || value >= 1.0 || value == 0.0 {
			if value > 0.0 && self.token().kind() == Kind::Number && self.token().sign_is_required() {
				f.write_str("+")?;
			}
			return value.fmt(f);
		}

		let mut small_str = SmallStrBuf::<255>::new();
		write!(&mut small_str, "{}", value.abs())?;
		if let Some(str) = small_str.as_str() {
			if value < 0.0 {
				f.write_str("-")?;
			} else if value > 0.0 && self.token().kind() == Kind::Number && self.token().sign_is_required() {
				f.write_str("+")?;
			}
			if let Some(rest) = str.strip_prefix("0.") {
				f.write_str(".")?;
				f.write_str(rest)
			} else {
				f.write_str(str)
			}
		} else {
			value.fmt(f)
		}
	}

	fn fmt_compacted_ident(&self, f: &mut Formatter<'_>) -> Result {
		let token = self.token();
		let start = token.leading_len() as usize;
		let end = self.source.len() - token.trailing_len() as usize;
		let source = &self.source[start..end];

		match token.kind() {
			Kind::AtKeyword => f.write_str("@")?,
			Kind::Hash => f.write_str("#")?,
			_ => {}
		}

		let mut chars = source.chars().peekable();
		let mut i = 0;
		while let Some(c) = chars.next() {
			if c == '\0' {
				write!(f, "{}", REPLACEMENT_CHARACTER)?;
				i += 1;
			} else if c == '\\' {
				i += 1;
				let (ch, n) = source[i..].chars().parse_escape_sequence();
				let char_to_write = if ch == '\0' { REPLACEMENT_CHARACTER } else { ch };
				write!(f, "{}", char_to_write)?;
				i += n as usize;
				chars = source[i..].chars().peekable();
			} else {
				write!(f, "{}", c)?;
				i += c.len_utf8();
			}
		}

		if token.kind() == Kind::Function {
			f.write_str("(")?;
		}

		Ok(())
	}

	pub fn eq_ignore_ascii_case(&self, other: &str) -> bool {
		debug_assert!(self.token() != Kind::Delim && self.token() != Kind::Url);
		debug_assert!(other.to_ascii_lowercase() == other);
		let start = self.token().leading_len() as usize;
		let end = self.source.len() - self.token().trailing_len() as usize;
		if !self.token().contains_escape_chars() {
			if end - start != other.len() {
				return false;
			}
			if self.token().is_lower_case() {
				debug_assert!(self.source[start..end].to_ascii_lowercase() == self.source[start..end]);
				return &self.source[start..end] == other;
			}
			return self.source[start..end].eq_ignore_ascii_case(other);
		}
		let mut chars = self.source[start..end].chars().peekable();
		let mut other_chars = other.chars();
		let mut i = 0;
		while let Some(c) = chars.next() {
			let o = other_chars.next();
			if o.is_none() {
				return false;
			}
			let o = o.unwrap();
			if c == '\0' {
				if REPLACEMENT_CHARACTER != o {
					return false;
				}
				i += 1;
			} else if c == '\\' {
				// String has special rules
				// https://drafts.csswg.org/css-syntax-3/#consume-string-token
				if self.token().kind_bits() == Kind::String as u8 {
					// When the token is a string, escaped EOF points are not consumed
					// U+005C REVERSE SOLIDUS (\)
					//   If the next input code point is EOF, do nothing.
					//   Otherwise, if the next input code point is a newline, consume it.
					let c = chars.peek();
					if let Some(c) = c {
						if is_newline(*c) {
							chars.next();
							if chars.peek() == Some(&'\n') {
								i += 1;
							}
							i += 2;
							chars = self.source[(start + i)..end].chars().peekable();
							continue;
						}
					} else {
						break;
					}
				}
				i += 1;
				let (ch, n) = self.source[(start + i)..].chars().parse_escape_sequence();
				i += n as usize;
				chars = self.source[(start + i)..end].chars().peekable();
				if (ch == '\0' && REPLACEMENT_CHARACTER != o) || ch != o {
					return false;
				}
			} else if c != o {
				return false;
			} else {
				i += c.len_utf8();
			}
		}
		other_chars.next().is_none()
	}

	/// Parse the cursor's content using any allocator that implements the Allocator trait.
	pub fn parse<A: Allocator + Clone + 'a>(&self, allocator: A) -> CowStr<'a, A> {
		debug_assert!(self.token() != Kind::Delim);
		let start = self.token().leading_len() as usize;
		let end = self.source.len() - self.token().trailing_len() as usize;
		if !self.token().contains_escape_chars() {
			return CowStr::<A>::Borrowed(&self.source[start..end]);
		}
		let mut chars = self.source[start..end].chars().peekable();
		let mut i = 0;
		let mut vec: Option<Vec<u8, A>> = None;
		while let Some(c) = chars.next() {
			if c == '\0' {
				if vec.is_none() {
					vec = if i == 0 {
						Some(Vec::new_in(allocator.clone()))
					} else {
						Some({
							let mut v = Vec::new_in(allocator.clone());
							v.extend(self.source[start..(start + i)].bytes());
							v
						})
					}
				}
				let mut buf = [0; 4];
				let bytes = REPLACEMENT_CHARACTER.encode_utf8(&mut buf).as_bytes();
				vec.as_mut().unwrap().extend_from_slice(bytes);
				i += 1;
			} else if c == '\\' {
				if vec.is_none() {
					vec = if i == 0 {
						Some(Vec::new_in(allocator.clone()))
					} else {
						Some({
							let mut v = Vec::new_in(allocator.clone());
							v.extend(self.source[start..(start + i)].bytes());
							v
						})
					}
				}
				// String has special rules
				// https://drafts.csswg.org/css-syntax-3/#consume-string-cursor
				if self.token().kind_bits() == Kind::String as u8 {
					// When the token is a string, escaped EOF points are not consumed
					// U+005C REVERSE SOLIDUS (\)
					//   If the next input code point is EOF, do nothing.
					//   Otherwise, if the next input code point is a newline, consume it.
					let c = chars.peek();
					if let Some(c) = c {
						if is_newline(*c) {
							chars.next();
							if chars.peek() == Some(&'\n') {
								i += 1;
							}
							i += 2;
							chars = self.source[(start + i)..end].chars().peekable();
							continue;
						}
					} else {
						break;
					}
				}
				i += 1;
				let (ch, n) = self.source[(start + i)..].chars().parse_escape_sequence();
				let char_to_push = if ch == '\0' { REPLACEMENT_CHARACTER } else { ch };
				let mut buf = [0; 4];
				let bytes = char_to_push.encode_utf8(&mut buf).as_bytes();
				vec.as_mut().unwrap().extend_from_slice(bytes);
				i += n as usize;
				chars = self.source[(start + i)..end].chars().peekable();
			} else {
				if let Some(bytes) = &mut vec {
					let mut buf = [0; 4];
					let char_bytes = c.encode_utf8(&mut buf).as_bytes();
					bytes.extend_from_slice(char_bytes);
				}
				i += c.len_utf8();
			}
		}
		match vec {
			Some(vec) => {
				let boxed_slice = vec.into_boxed_slice();
				// SAFETY: The source is valid UTF-8, so the slice is valid UTF-8
				unsafe { CowStr::Owned(Box::from_raw_in(Box::into_raw(boxed_slice) as *mut str, allocator)) }
			}
			None => CowStr::Borrowed(&self.source[start..start + i]),
		}
	}

	/// Parse the cursor's content to ASCII lowercase using any allocator that implements the Allocator trait.
	pub fn parse_ascii_lower<A: Allocator + Clone + 'a>(&self, allocator: A) -> CowStr<'a, A> {
		debug_assert!(self.token() != Kind::Delim);
		let start = self.token().leading_len() as usize;
		let end = self.source.len() - self.token().trailing_len() as usize;
		if !self.token().contains_escape_chars() && self.token().is_lower_case() {
			return CowStr::Borrowed(&self.source[start..end]);
		}
		let mut chars = self.source[start..end].chars().peekable();
		let mut i = 0;
		let mut vec: Vec<u8, A> = Vec::new_in(allocator.clone());
		while let Some(c) = chars.next() {
			if c == '\0' {
				let mut buf = [0; 4];
				let bytes = REPLACEMENT_CHARACTER.encode_utf8(&mut buf).as_bytes();
				vec.extend_from_slice(bytes);
				i += 1;
			} else if c == '\\' {
				// String has special rules
				// https://drafts.csswg.org/css-syntax-3/#consume-string-cursor
				if self.token().kind_bits() == Kind::String as u8 {
					// When the token is a string, escaped EOF points are not consumed
					// U+005C REVERSE SOLIDUS (\)
					//   If the next input code point is EOF, do nothing.
					//   Otherwise, if the next input code point is a newline, consume it.
					let c = chars.peek();
					if let Some(c) = c {
						if is_newline(*c) {
							chars.next();
							if chars.peek() == Some(&'\n') {
								i += 1;
							}
							i += 2;
							chars = self.source[(start + i)..end].chars().peekable();
							continue;
						}
					} else {
						break;
					}
				}
				i += 1;
				let (ch, n) = self.source[(start + i)..].chars().parse_escape_sequence();
				let char_to_push = if ch == '\0' { REPLACEMENT_CHARACTER } else { ch.to_ascii_lowercase() };
				let mut buf = [0; 4];
				let bytes = char_to_push.encode_utf8(&mut buf).as_bytes();
				vec.extend_from_slice(bytes);
				i += n as usize;
				chars = self.source[(start + i)..end].chars().peekable();
			} else {
				let mut buf = [0; 4];
				let bytes = c.to_ascii_lowercase().encode_utf8(&mut buf).as_bytes();
				vec.extend_from_slice(bytes);
				i += c.len_utf8();
			}
		}
		let boxed_slice = vec.into_boxed_slice();
		// SAFETY: The source is valid UTF-8, so the slice is valid UTF-8
		unsafe { CowStr::Owned(Box::from_raw_in(Box::into_raw(boxed_slice) as *mut str, allocator)) }
	}
}

impl PartialEq<Kind> for SourceCursor<'_> {
	fn eq(&self, other: &Kind) -> bool {
		self.token() == *other
	}
}

impl PartialEq<CommentStyle> for SourceCursor<'_> {
	fn eq(&self, other: &CommentStyle) -> bool {
		self.token() == *other
	}
}

impl From<SourceCursor<'_>> for KindSet {
	fn from(cursor: SourceCursor<'_>) -> Self {
		cursor.token().into()
	}
}

impl PartialEq<KindSet> for SourceCursor<'_> {
	fn eq(&self, other: &KindSet) -> bool {
		self.token() == *other
	}
}

#[cfg(test)]
mod test {
	use crate::{Cursor, QuoteStyle, SourceCursor, SourceOffset, Token, Whitespace};
	use allocator_api2::alloc::Global;
	use std::fmt::Write;

	#[test]
	fn parse_str_lower() {
		let c = Cursor::new(SourceOffset(0), Token::new_ident(true, false, false, 0, 3));
		assert_eq!(SourceCursor::from(c, "FoO").parse_ascii_lower(Global), "foo");
		assert_eq!(SourceCursor::from(c, "FOO").parse_ascii_lower(Global), "foo");
		assert_eq!(SourceCursor::from(c, "foo").parse_ascii_lower(Global), "foo");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, true, false, 5));
		assert_eq!(SourceCursor::from(c, "'FoO'").parse_ascii_lower(Global), "foo");
		assert_eq!(SourceCursor::from(c, "'FOO'").parse_ascii_lower(Global), "foo");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, false, false, 4));
		assert_eq!(SourceCursor::from(c, "'FoO").parse_ascii_lower(Global), "foo");
		assert_eq!(SourceCursor::from(c, "'FOO").parse_ascii_lower(Global), "foo");
		assert_eq!(SourceCursor::from(c, "'foo").parse_ascii_lower(Global), "foo");

		let c = Cursor::new(SourceOffset(0), Token::new_url(true, false, false, 4, 1, 6));
		assert_eq!(SourceCursor::from(c, "url(a)").parse_ascii_lower(Global), "a");
		assert_eq!(SourceCursor::from(c, "url(b)").parse_ascii_lower(Global), "b");

		let c = Cursor::new(SourceOffset(0), Token::new_url(true, false, false, 6, 1, 8));
		assert_eq!(SourceCursor::from(c, "\\75rl(A)").parse_ascii_lower(Global), "a");
		assert_eq!(SourceCursor::from(c, "u\\52l(B)").parse_ascii_lower(Global), "b");
		assert_eq!(SourceCursor::from(c, "ur\\6c(C)").parse_ascii_lower(Global), "c");

		let c = Cursor::new(SourceOffset(0), Token::new_url(true, false, false, 8, 1, 10));
		assert_eq!(SourceCursor::from(c, "\\75\\52l(A)").parse_ascii_lower(Global), "a");
		assert_eq!(SourceCursor::from(c, "u\\52\\6c(B)").parse_ascii_lower(Global), "b");
		assert_eq!(SourceCursor::from(c, "\\75r\\6c(C)").parse_ascii_lower(Global), "c");
	}

	#[test]
	fn eq_ignore_ascii_case() {
		let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, false, 0, 3));
		assert!(SourceCursor::from(c, "foo").eq_ignore_ascii_case("foo"));
		assert!(!SourceCursor::from(c, "foo").eq_ignore_ascii_case("bar"));
		assert!(!SourceCursor::from(c, "fo ").eq_ignore_ascii_case("foo"));
		assert!(!SourceCursor::from(c, "foo").eq_ignore_ascii_case("fooo"));
		assert!(!SourceCursor::from(c, "foo").eq_ignore_ascii_case("ғоо"));

		let c = Cursor::new(SourceOffset(0), Token::new_ident(true, false, false, 0, 3));
		assert!(SourceCursor::from(c, "FoO").eq_ignore_ascii_case("foo"));
		assert!(SourceCursor::from(c, "FOO").eq_ignore_ascii_case("foo"));
		assert!(!SourceCursor::from(c, "foo").eq_ignore_ascii_case("bar"));
		assert!(!SourceCursor::from(c, "fo ").eq_ignore_ascii_case("foo"));
		assert!(!SourceCursor::from(c, "foo").eq_ignore_ascii_case("fooo"));
		assert!(!SourceCursor::from(c, "foo").eq_ignore_ascii_case("ғоо"));

		let c = Cursor::new(SourceOffset(3), Token::new_ident(false, false, false, 0, 3));
		assert!(SourceCursor::from(c, "bar").eq_ignore_ascii_case("bar"));

		let c = Cursor::new(SourceOffset(3), Token::new_ident(false, false, true, 0, 3));
		assert!(SourceCursor::from(c, "bar").eq_ignore_ascii_case("bar"));

		let c = Cursor::new(SourceOffset(3), Token::new_ident(false, false, true, 0, 5));
		assert!(SourceCursor::from(c, "b\\61r").eq_ignore_ascii_case("bar"));

		let c = Cursor::new(SourceOffset(3), Token::new_ident(false, false, true, 0, 7));
		assert!(SourceCursor::from(c, "b\\61\\72").eq_ignore_ascii_case("bar"));
	}

	#[test]
	fn write_str() {
		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, false, 5));
		let mut str = String::new();
		write!(str, "{}", SourceCursor::from(c, "'foo'")).unwrap();
		assert_eq!(c.token().quote_style(), QuoteStyle::Double);
		assert_eq!(str, "\"foo\"");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, false, false, 4));
		let mut str = String::new();
		write!(str, "{}", SourceCursor::from(c, "'foo")).unwrap();
		assert_eq!(c.token().quote_style(), QuoteStyle::Double);
		assert_eq!(str, "\"foo\"");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, false, false, 4));
		let mut str = String::new();
		write!(str, "{}", SourceCursor::from(c, "\"foo")).unwrap();
		assert_eq!(c.token().quote_style(), QuoteStyle::Single);
		assert_eq!(str, "'foo'");
	}

	#[test]
	#[cfg(feature = "bumpalo")]
	fn test_bumpalo_compatibility() {
		use bumpalo::Bump;

		// Test that Bumpalo's Bump can be used as an allocator
		let bump = Bump::new();
		let c = Cursor::new(SourceOffset(0), Token::new_ident(true, false, false, 0, 3));

		// Test that the old interface still works
		assert_eq!(SourceCursor::from(c, "FoO").parse(&bump), "FoO");
		assert_eq!(SourceCursor::from(c, "FoO").parse_ascii_lower(&bump), "foo");

		// Test that the new interface works with Bumpalo too
		assert_eq!(&*SourceCursor::from(c, "FoO").parse(&bump), "FoO");
		assert_eq!(&*SourceCursor::from(c, "FoO").parse_ascii_lower(&bump), "foo");

		// Test with escape sequences
		let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, true, 0, 7));
		assert_eq!(SourceCursor::from(c, "b\\61\\72").parse(&bump), "bar");
		assert_eq!(&*SourceCursor::from(c, "b\\61\\72").parse(&bump), "bar");
	}

	#[test]
	fn test_compact_ident_with_escapes() {
		let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, true, 0, 5));
		let sc = SourceCursor::from(c, r"\66oo");
		assert_eq!(format!("{}", sc), r"\66oo");
		assert_eq!(format!("{}", sc.compact()), "foo");
	}

	#[test]
	fn test_compact_function_with_escapes() {
		let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, true, 0, 6));
		let sc = SourceCursor::from(c, r"\72gb(");
		assert_eq!(format!("{}", sc), r"\72gb(");
		assert_eq!(format!("{}", sc.compact()), "rgb(");
	}

	#[test]
	fn test_compact_number() {
		let c = Cursor::new(SourceOffset(0), Token::new_number(true, false, 3, 0.8));
		let sc = SourceCursor::from(c, r"0.8");
		assert_eq!(format!("{}", sc), r"0.8");
		assert_eq!(format!("{}", sc.compact()), ".8");

		let c = Cursor::new(SourceOffset(0), Token::new_number(false, false, 3, 1.0));
		let sc = SourceCursor::from(c, r"001");
		assert_eq!(format!("{}", sc), r"001");
		assert_eq!(format!("{}", sc.compact()), "1");

		let c = Cursor::new(SourceOffset(0), Token::new_number(true, true, 8, 1.0));
		let sc = SourceCursor::from(c, r"+1.00000");
		assert_eq!(format!("{}", sc), r"+1.00000");
		assert_eq!(format!("{}", sc.compact()), "1");

		let c = Cursor::new(SourceOffset(0), Token::new_number(true, true, 8, 1.0).with_sign_required());
		let sc = SourceCursor::from(c, r"+1.00000");
		assert_eq!(format!("{}", sc), r"+1.00000");
		assert_eq!(format!("{}", sc.compact()), "+1");

		let c = Cursor::new(SourceOffset(0), Token::new_number(true, false, 4, 0.01));
		let sc = SourceCursor::from(c, r"0.01");
		assert_eq!(format!("{}", sc), r"0.01");
		assert_eq!(format!("{}", sc.compact()), ".01");

		let c = Cursor::new(SourceOffset(0), Token::new_number(true, false, 5, -0.01));
		let sc = SourceCursor::from(c, r"-0.01");
		assert_eq!(format!("{}", sc), r"-0.01");
		assert_eq!(format!("{}", sc.compact()), "-.01");

		let c = Cursor::new(SourceOffset(0), Token::new_number(true, false, 4, 0.06));
		let sc = SourceCursor::from(c, r"0.06");
		assert_eq!(format!("{}", sc), r"0.06");
		assert_eq!(format!("{}", sc.compact()), ".06");
	}

	#[test]
	fn test_compact_dimension() {
		let c = Cursor::new(SourceOffset(0), Token::new_dimension(true, false, 4, 4, 0.8, 0));
		let sc = SourceCursor::from(c, r"+0.8\70x");
		assert_eq!(format!("{}", sc), r"+0.8\70x");
		assert_eq!(format!("{}", sc.compact()), ".8px");
	}

	#[test]
	fn test_compact_whitespace() {
		let c = Cursor::new(SourceOffset(0), Token::new_whitespace(Whitespace::Space, 3));
		let sc = SourceCursor::from(c, "   ");
		assert_eq!(format!("{}", sc), r"   ");
		assert_eq!(format!("{}", sc.compact()), " ");

		let c = Cursor::new(SourceOffset(0), Token::new_whitespace(Whitespace::Space, 7));
		let sc = SourceCursor::from(c, r"   \n\r");
		assert_eq!(format!("{}", sc), r"   \n\r");
		assert_eq!(format!("{}", sc.compact()), " ");
	}

	#[test]
	fn test_can_be_compacted_whitespace() {
		let c = Cursor::new(SourceOffset(0), Token::new_whitespace(Whitespace::Space, 1));
		assert!(!SourceCursor::from(c, " ").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_whitespace(Whitespace::Space, 3));
		assert!(SourceCursor::from(c, "   ").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_whitespace(Whitespace::Newline, 2));
		assert!(SourceCursor::from(c, "\n\n").may_compact());
	}

	#[test]
	fn test_can_be_compacted_ident() {
		let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, false, 0, 3));
		assert!(!SourceCursor::from(c, "foo").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, true, 0, 5));
		assert!(SourceCursor::from(c, r"\66oo").may_compact());
	}

	#[test]
	fn test_can_be_compacted_number() {
		let c = Cursor::new(SourceOffset(0), Token::new_number(true, false, 3, 0.8));
		assert!(SourceCursor::from(c, "0.8").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_number(true, false, 4, -0.5));
		assert!(SourceCursor::from(c, "-0.5").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_number(false, false, 1, 1.0));
		assert!(!SourceCursor::from(c, "1").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_number(false, false, 3, 1.0));
		assert!(SourceCursor::from(c, "001").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_number(false, false, 2, 1.0));
		assert!(SourceCursor::from(c, "+1").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_number(true, false, 3, 1.0));
		assert!(SourceCursor::from(c, "1.0").may_compact());
	}

	#[test]
	fn test_can_be_compacted_dimension() {
		let c = Cursor::new(SourceOffset(0), Token::new_dimension(true, true, 4, 4, 0.8, 0));
		assert!(SourceCursor::from(c, r"+0.8\70x").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_dimension(false, false, 1, 2, 1.0, 0));
		assert!(!SourceCursor::from(c, "1px").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_dimension(false, false, 2, 2, 1.0, 0));
		assert!(SourceCursor::from(c, "01px").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_dimension(false, false, 2, 2, 1.0, 0));
		assert!(SourceCursor::from(c, "+1px").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_dimension(true, false, 3, 2, 0.5, 0));
		assert!(SourceCursor::from(c, "0.5px").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_dimension(true, false, 2, 2, 0.5, 0));
		assert!(!SourceCursor::from(c, ".5px").may_compact());

		let c = Cursor::new(SourceOffset(0), Token::new_dimension(false, false, 1, 4, 1.0, 0));
		assert!(SourceCursor::from(c, r"1\70x").may_compact());
	}
}
