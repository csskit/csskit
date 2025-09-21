use crate::{Cursor, SourceCursor, Token};
use bumpalo::collections::Vec;

/// This trait provides the generic `impl` that [ToCursors][crate::ToCursors] can use. This provides just enough API
/// surface for nodes to put the cursors they represent into some buffer which can later be read, the details of which
/// are elided.
pub trait CursorSink {
	fn append(&mut self, c: Cursor);
}

pub trait SourceCursorSink<'a> {
	fn append(&mut self, c: SourceCursor<'a>);
}

const SEPARATOR: Cursor = Cursor::dummy(Token::SPACE);

impl<'a> CursorSink for Vec<'a, Cursor> {
	fn append(&mut self, c: Cursor) {
		// If two adjacent cursors which could not be re-tokenized in the same way if they were written out adjacently occur
		// then they should be separated by some token.
		if let Some(last) = self.last()
			&& last.token().needs_separator_for(c.into())
		{
			self.push(SEPARATOR);
		}
		self.push(c);
	}
}

impl<'a> SourceCursorSink<'a> for &mut Vec<'a, SourceCursor<'a>> {
	fn append(&mut self, c: SourceCursor<'a>) {
		// If two adjacent cursors which could not be re-tokenized in the same way if they were written out adjacently occur
		// then they should be separated by some token.
		if let Some(last) = self.last()
			&& last.token().needs_separator_for(c.token())
		{
			self.push(SourceCursor::from(SEPARATOR, " "));
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
	fn append(&mut self, c: Cursor) {
		self.sink.append(SourceCursor::from(c, c.str_slice(self.source)))
	}
}

impl<'a> SourceCursorSink<'a> for &mut String {
	fn append(&mut self, c: SourceCursor<'a>) {
		use std::fmt::Write;
		let _ = write!(self, "{c}");
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{ComponentValues, EmptyAtomSet, Parser, ToCursors};
	use bumpalo::Bump;

	#[test]
	fn test_cursor_sink_for_vec() {
		use std::fmt::Write;
		let source_text = "black white";
		let bump = Bump::default();
		let mut stream = Vec::new_in(&bump);
		let mut parser = Parser::new(&bump, &EmptyAtomSet::ATOMS, source_text);
		parser.parse_entirely::<ComponentValues>().output.unwrap().to_cursors(&mut stream);
		let mut str = String::new();
		for c in stream {
			write!(&mut str, "{}", SourceCursor::from(c, c.str_slice(source_text))).unwrap();
		}
		assert_eq!(str, "black white");
	}

	#[test]
	fn test_source_cursor_sink_for_string() {
		let source_text = "black white";
		let bump = Bump::default();
		let mut str = String::new();
		let mut transform = CursorToSourceCursorSink::new(source_text, &mut str);
		let mut parser = Parser::new(&bump, &EmptyAtomSet::ATOMS, source_text);
		parser.parse_entirely::<ComponentValues>().output.unwrap().to_cursors(&mut transform);
		assert_eq!(str, "black white");
	}
}
