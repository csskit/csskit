use crate::{
	AssociatedWhitespaceRules, CommentStyle, CowStr, Cursor, Kind, KindSet, QuoteStyle, SourceOffset, Span, ToSpan,
	Token,
	small_str_buf::SmallStrBuf,
	syntax::{
		ParseEscape,
		identifier::{is_ident, is_ident_start},
		is_newline,
	},
};
use allocator_api2::{alloc::Allocator, boxed::Box, vec::Vec};
use source_tools::SourceCursor as GenericSourceCursor;
use std::fmt::{Display, Formatter, Result, Write};
use std::{char::REPLACEMENT_CHARACTER, str::Chars};

struct ParsedChars<'a> {
	chars: Chars<'a>,
	is_string: bool,
}

impl<'a> ParsedChars<'a> {
	fn new(source: &'a str, is_string: bool) -> Self {
		Self { chars: source.chars(), is_string }
	}

	fn as_str(&self) -> &'a str {
		self.chars.as_str()
	}
}

impl Iterator for ParsedChars<'_> {
	// The decoded code point, and whether it came from an escape sequence.
	type Item = (char, bool);

	fn next(&mut self) -> Option<(char, bool)> {
		loop {
			let before = self.chars.clone();
			let c = self.chars.next()?;
			if c == '\0' {
				return Some((REPLACEMENT_CHARACTER, false));
			}
			if c != '\\' {
				return Some((c, false));
			}
			if self.is_string {
				// When the token is a string, an escaped EOF is not consumed and an escaped
				// newline is consumed without producing a code point.
				// https://drafts.csswg.org/css-syntax-3/#consume-string-token
				match self.chars.clone().next() {
					None => {
						self.chars = before;
						return None;
					}
					Some(c2) if is_newline(c2) => {
						self.chars.next();
						if c2 == '\r' && self.chars.as_str().starts_with('\n') {
							self.chars.next();
						}
						continue;
					}
					_ => {}
				}
			}
			let (c, _) = self.chars.parse_escape_sequence();
			return Some((if c == '\0' { REPLACEMENT_CHARACTER } else { c }, true));
		}
	}
}

/// Wraps [Cursor] with a [str] that represents the underlying character data for this cursor.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceCursor<'a> {
	inner: GenericSourceCursor<'a, Token>,
	should_compact: bool,
	#[cfg(feature = "egg")]
	should_expand: bool,
}

impl<'a> ToSpan for SourceCursor<'a> {
	fn to_span(&self) -> Span {
		self.inner.cursor().to_span()
	}
}

impl<'a> Display for SourceCursor<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self.token().kind() {
			Kind::Eof => Ok(()),
			#[cfg(feature = "egg")]
			Kind::String if self.should_expand => self.fmt_expanded_string(f),
			Kind::String if self.should_compact && self.token().contains_escape_chars() => self.fmt_compacted_string(f),
			// It is important to manually write out quotes for 2 reasons:
			//  1. The quote style can be mutated from the source string (such as the case of normalising/switching quotes.
			//  2. Some strings may not have the closing quote, which should be corrected.
			Kind::String => match self.token().quote_style() {
				QuoteStyle::Single => {
					let inner = &self.inner.source()
						[1..(self.token().len() as usize) - self.token().has_close_quote() as usize];
					write!(f, "'{inner}'")
				}
				QuoteStyle::Double => {
					let inner = &self.inner.source()
						[1..(self.token().len() as usize) - self.token().has_close_quote() as usize];
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
			#[cfg(feature = "egg")]
			_ if self.should_expand => self.fmt_expanded(f),
			_ => f.write_str(self.inner.source()),
		}
	}
}

impl<'a> SourceCursor<'a> {
	pub const SPACE: SourceCursor<'static> = SourceCursor::from(Cursor::new(SourceOffset(0), Token::SPACE), " ");
	pub const TAB: SourceCursor<'static> = SourceCursor::from(Cursor::new(SourceOffset(0), Token::TAB), "\t");
	pub const NEWLINE: SourceCursor<'static> = SourceCursor::from(Cursor::new(SourceOffset(0), Token::NEWLINE), "\n");
	pub const SEMICOLON: SourceCursor<'static> =
		SourceCursor::from(Cursor::new(SourceOffset(0), Token::SEMICOLON), ";");
	pub const EMPTY_COMMENT: SourceCursor<'static> =
		SourceCursor::from(Cursor::new(SourceOffset(0), Token::new_comment(CommentStyle::Block, 4)), "/**/");

	#[inline(always)]
	pub const fn from(cursor: Cursor, source: &'a str) -> Self {
		debug_assert!(
			(cursor.token().len() as usize) == source.len(),
			"A SourceCursor should be constructed with a source that matches the length of the cursor!"
		);
		Self {
			inner: GenericSourceCursor::new(cursor, source),
			should_compact: false,
			#[cfg(feature = "egg")]
			should_expand: false,
		}
	}

	#[inline(always)]
	pub const fn cursor(&self) -> Cursor {
		self.inner.cursor()
	}

	#[inline(always)]
	pub const fn token(&self) -> Token {
		self.inner.cursor().token()
	}

	#[inline(always)]
	pub const fn source(&self) -> &'a str {
		self.inner.source()
	}

	pub fn with_quotes(&self, quote_style: QuoteStyle) -> Self {
		let cursor = self.inner.cursor().map_token(|token| token.with_quotes(quote_style));
		Self {
			inner: GenericSourceCursor::new(cursor, self.inner.source()),
			should_compact: self.should_compact,
			#[cfg(feature = "egg")]
			should_expand: self.should_expand,
		}
	}

	pub fn with_associated_whitespace(&self, rules: AssociatedWhitespaceRules) -> Self {
		let cursor = self.inner.cursor().map_token(|token| token.with_associated_whitespace(rules));
		Self {
			inner: GenericSourceCursor::new(cursor, self.inner.source()),
			should_compact: self.should_compact,
			#[cfg(feature = "egg")]
			should_expand: self.should_expand,
		}
	}

	pub fn with_significant_whitespace(&self, significant: bool) -> Self {
		let cursor = self.inner.cursor().map_token(|token| token.with_significant_whitespace(significant));
		Self {
			inner: GenericSourceCursor::new(cursor, self.inner.source()),
			should_compact: self.should_compact,
			#[cfg(feature = "egg")]
			should_expand: self.should_expand,
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
		Self {
			inner: self.inner,
			should_compact: true,
			#[cfg(feature = "egg")]
			should_expand: false,
		}
	}

	/// Returns a new `SourceCursor` with the `should_expand` flag set.
	///
	/// With the `should_expand` flag set, the cursor will format with verbose displays of:
	/// - Numbers: Padded with leading zeros and trailing decimal places (`1` -> `000001.00000000`)
	/// - Idents/Functions/AtKeywords: Each character as `\XXXXXX ` escape codes
	/// - Dimensions: Same as numbers for the number part, and same as Idents for the unit part
	/// - Whitespace: Expanded to multiple characters
	///
	#[cfg(feature = "egg")]
	pub fn expand(&self) -> SourceCursor<'a> {
		Self { inner: self.inner, should_compact: false, should_expand: true }
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
					|| self.inner.source()[(self.token().numeric_len() as usize)..]
						.bytes()
						.any(|b| b == b'\\' || b == 0)
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
			let bytes = self.inner.source().as_bytes();
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
			Kind::Url => self.fmt_compacted_url(f),
			_ => f.write_str(self.inner.source()),
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
		let source = self.content_slice();

		match token.kind() {
			Kind::AtKeyword => f.write_str("@")?,
			Kind::Hash => f.write_str("#")?,
			_ => {}
		}

		let mut chars = self.parsed_chars();
		let mut char_i = 0;
		while let Some((ch, escaped)) = chars.next() {
			if !escaped {
				f.write_char(ch)?;
			} else {
				// Check if the decoded character is valid at this position unescaped.
				let valid_unescaped = if ch == '-' && char_i == 0 {
					true
				} else if char_i == 0 || (char_i == 1 && source.starts_with('-')) {
					is_ident_start(ch)
				} else {
					is_ident(ch)
				};
				if valid_unescaped {
					f.write_char(ch)?;
				} else if !ch.is_ascii_hexdigit() && !ch.is_ascii_whitespace() && !is_newline(ch) {
					write!(f, "\\{}", ch)?;
				} else {
					write!(f, "\\{:x}", ch as u32)?;
					// A trailing space is needed if the next character is a hex digit
					// or whitespace, to prevent it from being consumed as part of the escape.
					let next_char = chars.as_str().chars().next();
					if next_char.is_some_and(|nc| nc.is_ascii_hexdigit() || nc == ' ' || nc == '\t') {
						f.write_char(' ')?;
					}
				}
			}
			char_i += 1;
		}

		if token.kind() == Kind::Function {
			f.write_str("(")?;
		}

		Ok(())
	}

	fn fmt_compacted_url(&self, f: &mut Formatter<'_>) -> Result {
		let token = self.token();
		let leading_len = token.leading_len() as usize;
		let trailing_len = token.trailing_len() as usize;
		f.write_str("url(")?;
		let url_content = &self.inner.source()[leading_len..(self.inner.source().len() - trailing_len)];
		f.write_str(url_content.trim())?;
		if token.url_has_closing_paren() {
			f.write_str(")")?;
		}
		Ok(())
	}

	fn fmt_compacted_string(&self, f: &mut Formatter<'_>) -> Result {
		let quote = match self.token().quote_style() {
			QuoteStyle::Single => '\'',
			QuoteStyle::Double => '"',
			QuoteStyle::None => unreachable!(),
		};
		f.write_char(quote)?;
		let mut chars = self.parsed_chars();
		while let Some((ch, escaped)) = chars.next() {
			if escaped && (is_newline(ch) || ch == quote || ch == '\\') {
				write!(f, "\\{:x}", ch as u32)?;
				// Trailing space needed if next char is a hex digit.
				let next_char = chars.as_str().chars().next();
				if next_char.is_some_and(|nc| nc.is_ascii_hexdigit() || nc == ' ' || nc == '\t') {
					f.write_char(' ')?;
				}
			} else {
				f.write_char(ch)?;
			}
		}
		f.write_char(quote)?;
		Ok(())
	}

	#[cfg(feature = "egg")]
	fn fmt_expanded(&self, f: &mut Formatter<'_>) -> Result {
		let token = self.token();
		match token.kind() {
			Kind::Whitespace => f.write_str("    "),
			Kind::Ident | Kind::Function | Kind::AtKeyword | Kind::Hash => self.fmt_expanded_ident(f),
			Kind::Number => self.fmt_expanded_number(f),
			Kind::Dimension => {
				self.fmt_expanded_number(f)?;
				self.fmt_expanded_ident(f)
			}
			Kind::Url => self.fmt_expanded_url(f),
			_ => f.write_str(self.inner.source()),
		}
	}

	#[cfg(feature = "egg")]
	fn fmt_expanded_number(&self, f: &mut Formatter<'_>) -> Result {
		let value = self.token().value();
		let is_negative = value < 0.0;
		let abs_value = value.abs();
		if is_negative {
			f.write_str("-")?;
		} else {
			f.write_str("+")?;
		}

		if self.token().is_int() {
			return write!(f, "{:010.0}", abs_value);
		}
		if value == 0.0 {
			return f.write_str("0.00000000000000e+0000000000");
		}
		let exp = abs_value.log10().floor() as i32;
		let mantissa = abs_value / 10_f32.powi(exp);
		write!(f, "{:.14}e{:+011}", mantissa, exp)
	}

	#[cfg(feature = "egg")]
	fn fmt_expanded_ident(&self, f: &mut Formatter<'_>) -> Result {
		let token = self.token();

		match token.kind() {
			Kind::AtKeyword => f.write_str("@")?,
			Kind::Hash => f.write_str("#")?,
			_ => {}
		}

		for (ch, _) in self.parsed_chars() {
			write!(f, "\\{:06x} ", ch as u32)?;
		}

		if token.kind() == Kind::Function {
			f.write_str("(")?;
		}

		Ok(())
	}

	#[cfg(feature = "egg")]
	fn fmt_expanded_url(&self, f: &mut Formatter<'_>) -> Result {
		let token = self.token();
		let leading_len = token.leading_len() as usize;
		let trailing_len = token.trailing_len() as usize;
		let url_prefix = &self.inner.source()[..leading_len];
		let url_content = &self.inner.source()[leading_len..(self.inner.source().len() - trailing_len)];
		f.write_str(url_prefix)?;
		f.write_str("   ")?;
		f.write_str(url_content.trim())?;
		f.write_str("   ")?;
		if token.url_has_closing_paren() {
			f.write_str(")")?;
		}
		Ok(())
	}

	#[cfg(feature = "egg")]
	fn fmt_expanded_string(&self, f: &mut Formatter<'_>) -> Result {
		let token = self.token();
		let inner = &self.inner.source()[1..(token.len() as usize) - token.has_close_quote() as usize];
		// Use the opposite quote style to maximize escaping opportunity
		let (open_quote, close_quote, escape_char) = match token.quote_style() {
			QuoteStyle::Single => ('"', '"', '"'),
			QuoteStyle::Double => ('\'', '\'', '\''),
			QuoteStyle::None => unreachable!(),
		};
		f.write_char(open_quote)?;
		for c in inner.chars() {
			if c == escape_char {
				// Escape the quote character
				write!(f, "\\{:06x} ", c as u32)?;
			} else if c.is_ascii() && !c.is_ascii_control() {
				// Escape all printable ASCII as hex
				write!(f, "\\{:06x} ", c as u32)?;
			} else {
				// Non-ASCII or control chars: write as-is or escape
				write!(f, "\\{:06x} ", c as u32)?;
			}
		}
		f.write_char(close_quote)?;
		Ok(())
	}

	pub fn eq_ignore_ascii_case(&self, other: &str) -> bool {
		debug_assert!(self.token() != Kind::Delim && self.token() != Kind::Url);
		debug_assert!(other.to_ascii_lowercase() == other);
		let source = self.content_slice();
		if !self.token().contains_escape_chars() {
			if source.len() != other.len() {
				return false;
			}
			if self.token().is_lower_case() {
				debug_assert!(source.to_ascii_lowercase() == source);
				return source == other;
			}
			return source.eq_ignore_ascii_case(other);
		}
		let mut chars = self.parsed_chars();
		let mut other_chars = other.chars();
		loop {
			match (chars.next(), other_chars.next()) {
				(None, None) => return true,
				(Some((c, _)), Some(o)) => {
					if !c.eq_ignore_ascii_case(&o) {
						return false;
					}
				}
				_ => return false,
			}
		}
	}

	/// Parse the cursor's content using any allocator that implements the Allocator trait.
	pub fn parse<A: Allocator + Clone + 'a>(&self, allocator: A) -> CowStr<'a, A> {
		debug_assert!(self.token() != Kind::Delim);
		let source = self.content_slice();
		if !self.token().contains_escape_chars() {
			return CowStr::Borrowed(source);
		}
		let mut chars = self.parsed_chars();
		let mut vec: Option<Vec<u8, A>> = None;
		let mut yielded_len = 0;
		loop {
			let rest = chars.as_str();
			let Some((c, escaped)) = chars.next() else { break };
			if let Some(vec) = &mut vec {
				let mut buf = [0; 4];
				vec.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
			} else if escaped || rest.len() - chars.as_str().len() != c.len_utf8() {
				let mut v = Vec::new_in(allocator.clone());
				v.extend_from_slice(&source.as_bytes()[..source.len() - rest.len()]);
				let mut buf = [0; 4];
				v.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
				vec = Some(v);
			}
			yielded_len = source.len() - chars.as_str().len();
		}
		match vec {
			Some(vec) => {
				let boxed_slice = vec.into_boxed_slice();
				// SAFETY: The source is valid UTF-8, so the slice is valid UTF-8
				unsafe { CowStr::Owned(Box::from_raw_in(Box::into_raw(boxed_slice) as *mut str, allocator)) }
			}
			None => CowStr::Borrowed(&source[..yielded_len]),
		}
	}

	/// Parse the cursor's content to ASCII lowercase using any allocator that implements the Allocator trait.
	pub fn parse_ascii_lower<A: Allocator + Clone + 'a>(&self, allocator: A) -> CowStr<'a, A> {
		debug_assert!(self.token() != Kind::Delim);
		let source = self.content_slice();
		if !self.token().contains_escape_chars() && self.token().is_lower_case() {
			return CowStr::Borrowed(source);
		}
		let mut vec: Vec<u8, A> = Vec::new_in(allocator.clone());
		for (c, _) in self.parsed_chars() {
			let mut buf = [0; 4];
			vec.extend_from_slice(c.to_ascii_lowercase().encode_utf8(&mut buf).as_bytes());
		}
		let boxed_slice = vec.into_boxed_slice();
		// SAFETY: The source is valid UTF-8, so the slice is valid UTF-8
		unsafe { CowStr::Owned(Box::from_raw_in(Box::into_raw(boxed_slice) as *mut str, allocator)) }
	}

	fn content_slice(&self) -> &'a str {
		let start = self.token().leading_len() as usize;
		let end = self.source().len() - self.token().trailing_len() as usize;
		&self.source()[start..end]
	}

	fn parsed_chars(&self) -> ParsedChars<'a> {
		ParsedChars::new(self.content_slice(), self.token().kind() == Kind::String)
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
	fn parse_string_line_continuation() {
		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 7));
		assert_eq!(SourceCursor::from(c, "\"foo\\\n\"").parse(Global), "foo");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 8));
		assert_eq!(SourceCursor::from(c, "\"foo\\\r\n\"").parse(Global), "foo");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 7));
		assert_eq!(SourceCursor::from(c, "\"fo\\\no\"").parse(Global), "foo");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, false, true, 5));
		assert_eq!(SourceCursor::from(c, "\"foo\\").parse(Global), "foo");
	}

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

		let c = Cursor::new(SourceOffset(3), Token::new_ident(false, false, true, 0, 5));
		assert!(SourceCursor::from(c, "b\\41r").eq_ignore_ascii_case("bar"));
		assert!(!SourceCursor::from(c, "b\\42r").eq_ignore_ascii_case("bar"));

		let c = Cursor::new(SourceOffset(3), Token::new_ident(false, false, true, 0, 7));
		assert!(SourceCursor::from(c, "b\\41\\52").eq_ignore_ascii_case("bar"));
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

	#[test]
	fn test_compact_url() {
		let c = Cursor::new(SourceOffset(0), Token::new_url(true, true, false, 7, 1, 15));
		let sc = SourceCursor::from(c, "url(   foo.png)");
		assert_eq!(format!("{}", sc), "url(   foo.png)");
		assert_eq!(format!("{}", sc.compact()), "url(foo.png)");

		let c = Cursor::new(SourceOffset(0), Token::new_url(true, false, false, 4, 4, 15));
		let sc = SourceCursor::from(c, "url(foo.png   )");
		assert_eq!(format!("{}", sc), "url(foo.png   )");
		assert_eq!(format!("{}", sc.compact()), "url(foo.png)");

		let c = Cursor::new(SourceOffset(0), Token::new_url(true, true, false, 6, 3, 16));
		let sc = SourceCursor::from(c, "url(  foo.png  )");
		assert_eq!(format!("{}", sc), "url(  foo.png  )");
		assert_eq!(format!("{}", sc.compact()), "url(foo.png)");

		let c = Cursor::new(SourceOffset(0), Token::new_url(false, false, false, 4, 0, 11));
		let sc = SourceCursor::from(c, "url(foo.png");
		assert_eq!(format!("{}", sc), "url(foo.png");
		assert_eq!(format!("{}", sc.compact()), "url(foo.png");
	}

	#[test]
	fn test_compact_string_with_escapes() {
		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 7));
		let sc = SourceCursor::from(c, r#""\66oo""#);
		assert_eq!(format!("{}", sc), r#""\66oo""#);
		assert_eq!(format!("{}", sc.compact()), r#""foo""#);

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, true, true, 8));
		let sc = SourceCursor::from(c, r"'\62 ar'");
		assert_eq!(format!("{}", sc), r"'\62 ar'");
		assert_eq!(format!("{}", sc.compact()), "'bar'");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 11));
		let sc = SourceCursor::from(c, r#""\68\65llo""#);
		assert_eq!(format!("{}", sc), r#""\68\65llo""#);
		assert_eq!(format!("{}", sc.compact()), r#""hello""#);

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 5));
		let sc = SourceCursor::from(c, "\"\0oo\"");
		assert_eq!(format!("{}", sc.compact()), "\"\u{FFFD}oo\"");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 6));
		let sc = SourceCursor::from(c, "\"\x5c0oo\"");
		assert_eq!(format!("{}", sc.compact()), "\"\u{FFFD}oo\"");
	}

	#[test]
	fn test_compact_string_line_continuation() {
		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, true, true, 6));
		let sc = SourceCursor::from(c, "'a\\\nb'");
		assert_eq!(format!("{}", sc), "'a\\\nb'");
		assert_eq!(format!("{}", sc.compact()), "'ab'");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, true, true, 7));
		let sc = SourceCursor::from(c, "'a\\\r\nb'");
		assert_eq!(format!("{}", sc.compact()), "'ab'");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, true, true, 7));
		let sc = SourceCursor::from(c, "'a\\a b'");
		assert_eq!(format!("{}", sc.compact()), "'a\\a b'");
	}

	#[test]
	fn test_compact_ident_reencodes_invalid_unescaped() {
		let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, true, 0, 5));
		let sc = SourceCursor::from(c, r"\66oo");
		assert_eq!(format!("{}", sc.compact()), "foo");

		let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, true, 0, 6));
		let sc = SourceCursor::from(c, r"a\20 b");
		let compacted = format!("{}", sc.compact());
		assert_eq!(compacted, "a\\20 b");

		let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, true, 0, 3));
		let sc = SourceCursor::from(c, "a\\!");
		let compacted = format!("{}", sc.compact());
		assert_eq!(compacted, "a\\!");

		let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, true, 0, 5));
		let sc = SourceCursor::from(c, r"b\61r");
		assert_eq!(format!("{}", sc.compact()), "bar");

		let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, true, 0, 6));
		let sc = SourceCursor::from(c, r"\31 23");
		assert_eq!(format!("{}", sc.compact()), r"\31 23");

		let c = Cursor::new(SourceOffset(0), Token::new_ident(false, false, true, 0, 9));
		let sc = SourceCursor::from(c, r"\66\6f\6f");
		assert_eq!(format!("{}", sc.compact()), "foo");
	}

	#[test]
	fn test_compact_string_reencodes_special_chars() {
		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 7));
		let sc = SourceCursor::from(c, "\"\\22 x\"");
		let compacted = format!("{}", sc.compact());
		assert_eq!(compacted, "\"\\22x\"");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 7));
		let sc = SourceCursor::from(c, "\"\\22 a\"");
		let compacted = format!("{}", sc.compact());
		assert_eq!(compacted, "\"\\22 a\"");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Single, true, true, 7));
		let sc = SourceCursor::from(c, "'\\27 x'");
		let compacted = format!("{}", sc.compact());
		assert_eq!(compacted, "'\\27x'");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 7));
		let sc = SourceCursor::from(c, "\"\\5c x\"");
		let compacted = format!("{}", sc.compact());
		assert_eq!(compacted, "\"\\5cx\"");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 7));
		let sc = SourceCursor::from(c, "\"\\5c a\"");
		let compacted = format!("{}", sc.compact());
		assert_eq!(compacted, "\"\\5c a\"");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 6));
		let sc = SourceCursor::from(c, "\"\\a x\"");
		let compacted = format!("{}", sc.compact());
		assert_eq!(compacted, "\"\\ax\"");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 6));
		let sc = SourceCursor::from(c, "\"\\a b\"");
		let compacted = format!("{}", sc.compact());
		assert_eq!(compacted, "\"\\a b\"");

		let c = Cursor::new(SourceOffset(0), Token::new_string(QuoteStyle::Double, true, true, 7));
		let sc = SourceCursor::from(c, "\"\\66oo\"");
		assert_eq!(format!("{}", sc.compact()), "\"foo\"");
	}
}
