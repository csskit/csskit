use crate::{
	AssociatedWhitespaceRules, CommentStyle, Cursor, Kind, KindSet, QuoteStyle, SourceOffset, Span, ToSpan, Token,
};
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
