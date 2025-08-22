use bumpalo::collections::Vec;
use css_lexer::{Cursor, ToSpan, Token};
use std::fmt::{Result, Write};

/// This trait provides the generic `impl` that [ToCursors][crate::ToCursors] can use. This provides just enough API
/// surface for nodes to put the cursors they represent into some buffer which can later be read, the details of which
/// are elided.
pub trait CursorSink {
	fn append(&mut self, c: Cursor);
}

#[derive(Debug, Copy, Clone)]
pub struct SourceCursor<'a> {
	cursor: Cursor,
	source: &'a str,
}

impl<'a> ToSpan for SourceCursor<'a> {
	fn to_span(&self) -> css_lexer::Span {
		self.cursor.to_span()
	}
}

impl<'a> SourceCursor<'a> {
	#[inline(always)]
	pub fn from(cursor: Cursor, source: &'a str) -> SourceCursor<'a> {
		Self { cursor, source }
	}

	#[inline(always)]
	pub fn write_str(&self, f: &mut impl Write) -> Result {
		self.cursor.write_str(self.source, f)
	}

	#[inline(always)]
	pub const fn cursor(&self) -> Cursor {
		self.cursor
	}

	#[inline(always)]
	pub const fn source(&self) -> &'a str {
		self.source
	}

	#[inline(always)]
	pub const fn token(&self) -> Token {
		self.cursor.token()
	}
}

pub trait SourceCursorSink<'a> {
	fn append(&mut self, c: SourceCursor<'a>);
}

const SEPARATOR: Cursor = Cursor::dummy(Token::SPACE);

impl<'a> CursorSink for Vec<'a, Cursor> {
	fn append(&mut self, c: Cursor) {
		// If two adjacent cursors which could not be re-tokenized in the same way if they were written out adjacently occur
		// then they should be separated by some token.
		if let Some(last) = self.last() {
			if last.token().needs_separator_for(c.into()) {
				self.push(SEPARATOR);
			}
		}
		self.push(c);
	}
}

impl<'a> SourceCursorSink<'a> for &mut Vec<'a, SourceCursor<'a>> {
	fn append(&mut self, c: SourceCursor<'a>) {
		// If two adjacent cursors which could not be re-tokenized in the same way if they were written out adjacently occur
		// then they should be separated by some token.
		if let Some(last) = self.last() {
			if last.token().needs_separator_for(c.token()) {
				self.push(SourceCursor::from(SEPARATOR, ""));
			}
		}
		self.push(c);
	}
}

pub struct CursorToSourceCursorSink<'a, T: SourceCursorSink<'a>> {
	source: &'a str,
	sink: T,
}

impl<'a, T: SourceCursorSink<'a>> CursorToSourceCursorSink<'a, T> {
	pub fn new(source: &'a str, sink: T) -> Self {
		Self { source, sink }
	}
}

impl<'a, T: SourceCursorSink<'a>> CursorSink for CursorToSourceCursorSink<'a, T> {
	fn append(&mut self, cursor: Cursor) {
		self.sink.append(SourceCursor::from(cursor, self.source))
	}
}

impl<'a> SourceCursorSink<'a> for &mut String {
	fn append(&mut self, c: SourceCursor<'a>) {
		let _ = c.write_str(self);
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{ComponentValues, Parser, ToCursors};
	use bumpalo::Bump;

	#[test]
	fn test_cursor_sink_for_vec() {
		let source_text = "black white";
		let bump = Bump::default();
		let result = Parser::new(&bump, source_text).parse_entirely::<ComponentValues>();
		let mut stream = Vec::new_in(&bump);
		result.to_cursors(&mut stream);
		let mut str = String::new();
		for sc in stream {
			sc.write_str(source_text, &mut str).unwrap();
		}
		assert_eq!(str, "black white");
	}

	#[test]
	fn test_source_cursor_sink_for_string() {
		let source_text = "black white";
		let bump = Bump::default();
		let result = Parser::new(&bump, source_text).parse_entirely::<ComponentValues>();
		let mut str = String::new();
		let mut transform = CursorToSourceCursorSink::new(source_text, &mut str);
		result.to_cursors(&mut transform);
		assert_eq!(str, "black white");
	}
}
