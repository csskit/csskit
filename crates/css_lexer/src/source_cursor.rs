use crate::{
	AssociatedWhitespaceRules, CommentStyle, Cursor, Kind, KindSet, QuoteStyle, SourceOffset, Span, ToSpan, Token,
	syntax::{ParseEscape, is_newline},
};
use bumpalo::{Bump, collections::String};
use std::char::REPLACEMENT_CHARACTER;
use std::fmt::{Display, Formatter, Result};

/// Wraps [Cursor] with a [str] that represents the underlying character data for this cursor.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceCursor<'a> {
	cursor: Cursor,
	source: &'a str,
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
		Self { cursor, source }
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
		Self::from(self.cursor.with_quotes(quote_style), self.source)
	}

	pub fn with_associated_whitespace(&self, rules: AssociatedWhitespaceRules) -> Self {
		Self::from(self.cursor.with_associated_whitespace(rules), self.source)
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

	pub fn parse(&self, allocator: &'a Bump) -> &'a str {
		debug_assert!(self.token() != Kind::Delim);
		let start = self.token().leading_len() as usize;
		let end = self.source.len() - self.token().trailing_len() as usize;
		if !self.token().contains_escape_chars() {
			return &self.source[start..end];
		}
		let mut chars = self.source[start..end].chars().peekable();
		let mut i = 0;
		let mut str: Option<String<'a>> = None;
		while let Some(c) = chars.next() {
			if c == '\0' {
				if str.is_none() {
					str = if i == 0 {
						Some(String::new_in(allocator))
					} else {
						Some(String::from_str_in(&self.source[start..(start + i)], allocator))
					}
				}
				str.as_mut().unwrap().push(REPLACEMENT_CHARACTER);
				i += 1;
			} else if c == '\\' {
				if str.is_none() {
					str = if i == 0 {
						Some(String::new_in(allocator))
					} else {
						Some(String::from_str_in(&self.source[start..(start + i)], allocator))
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
				str.as_mut().unwrap().push(if ch == '\0' { REPLACEMENT_CHARACTER } else { ch });
				i += n as usize;
				chars = self.source[(start + i)..end].chars().peekable();
			} else {
				if let Some(text) = &mut str {
					text.push(c);
				}
				i += c.len_utf8();
			}
		}
		if str.is_some() { str.take().unwrap().into_bump_str() } else { &self.source[start..start + i] }
	}

	pub fn parse_ascii_lower(&self, allocator: &'a Bump) -> &'a str {
		debug_assert!(self.token() != Kind::Delim);
		if self.token().is_lower_case() {
			return self.parse(allocator);
		}
		let start = self.token().leading_len() as usize;
		let end = self.source.len() - self.token().trailing_len() as usize;
		if !self.token().contains_escape_chars() && self.token().is_lower_case() {
			return &self.source[start..end];
		}
		let mut chars = self.source[start..end].chars().peekable();
		let mut i = 0;
		let mut str: String<'a> = String::new_in(allocator);
		while let Some(c) = chars.next() {
			if c == '\0' {
				str.push(REPLACEMENT_CHARACTER);
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
				str.push(if ch == '\0' { REPLACEMENT_CHARACTER } else { ch.to_ascii_lowercase() });
				i += n as usize;
				chars = self.source[(start + i)..end].chars().peekable();
			} else {
				str.push(c.to_ascii_lowercase());
				i += c.len_utf8();
			}
		}
		str.into_bump_str()
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
	use crate::{Cursor, QuoteStyle, SourceCursor, SourceOffset, Token};
	use bumpalo::{Bump, collections::String};
	use std::fmt::Write;

	#[test]
	fn parse_str_lower() {
		let allocator = Bump::new();
		let c = Cursor::new(SourceOffset(0), Token::new_ident(true, false, false, 0, 3));
		assert_eq!(SourceCursor::from(c, "FoO").parse_ascii_lower(&allocator), "foo");
		assert_eq!(SourceCursor::from(c, "FOO").parse_ascii_lower(&allocator), "foo");
		assert_eq!(SourceCursor::from(c, "foo").parse_ascii_lower(&allocator), "foo");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, true, false, 5));
		assert_eq!(SourceCursor::from(c, "'FoO'").parse_ascii_lower(&allocator), "foo");
		assert_eq!(SourceCursor::from(c, "'FOO'").parse_ascii_lower(&allocator), "foo");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, false, false, 4));
		assert_eq!(SourceCursor::from(c, "'FoO").parse_ascii_lower(&allocator), "foo");
		assert_eq!(SourceCursor::from(c, "'FOO").parse_ascii_lower(&allocator), "foo");
		assert_eq!(SourceCursor::from(c, "'foo").parse_ascii_lower(&allocator), "foo");

		let c = Cursor::new(SourceOffset(0), Token::new_url(true, false, false, 4, 1, 6));
		assert_eq!(SourceCursor::from(c, "url(a)").parse_ascii_lower(&allocator), "a");
		assert_eq!(SourceCursor::from(c, "url(b)").parse_ascii_lower(&allocator), "b");

		let c = Cursor::new(SourceOffset(0), Token::new_url(true, false, false, 6, 1, 8));
		assert_eq!(SourceCursor::from(c, "\\75rl(A)").parse_ascii_lower(&allocator), "a");
		assert_eq!(SourceCursor::from(c, "u\\52l(B)").parse_ascii_lower(&allocator), "b");
		assert_eq!(SourceCursor::from(c, "ur\\6c(C)").parse_ascii_lower(&allocator), "c");

		let c = Cursor::new(SourceOffset(0), Token::new_url(true, false, false, 8, 1, 10));
		assert_eq!(SourceCursor::from(c, "\\75\\52l(A)").parse_ascii_lower(&allocator), "a");
		assert_eq!(SourceCursor::from(c, "u\\52\\6c(B)").parse_ascii_lower(&allocator), "b");
		assert_eq!(SourceCursor::from(c, "\\75r\\6c(C)").parse_ascii_lower(&allocator), "c");
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
		let bump = Bump::new();
		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, false, 5));
		let mut str = String::new_in(&bump);
		write!(str, "{}", SourceCursor::from(c, "'foo'")).unwrap();
		assert_eq!(c.token().quote_style(), QuoteStyle::Double);
		assert_eq!(str, "\"foo\"");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, false, false, 4));
		let mut str = String::new_in(&bump);
		write!(str, "{}", SourceCursor::from(c, "'foo")).unwrap();
		assert_eq!(c.token().quote_style(), QuoteStyle::Double);
		assert_eq!(str, "\"foo\"");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, false, false, 4));
		let mut str = String::new_in(&bump);
		write!(str, "{}", SourceCursor::from(c, "\"foo")).unwrap();
		assert_eq!(c.token().quote_style(), QuoteStyle::Single);
		assert_eq!(str, "'foo'");
	}
}
