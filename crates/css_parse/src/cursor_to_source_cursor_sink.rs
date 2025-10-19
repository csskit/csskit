use crate::{Cursor, CursorSink, SourceCursor, SourceCursorSink};
use std::fmt::Write;

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
		self.sink.append(SourceCursor::from(cursor, cursor.str_slice(self.source)))
	}
}

impl<'a> SourceCursorSink<'a> for String {
	fn append(&mut self, c: SourceCursor<'a>) {
		let _ = write!(self, "{c}");
	}
}

impl<'a, T: SourceCursorSink<'a>> SourceCursorSink<'a> for &mut T {
	fn append(&mut self, c: SourceCursor<'a>) {
		(**self).append(c)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{ComponentValues, EmptyAtomSet, Parser, ToCursors};
	use bumpalo::Bump;
	use css_lexer::Lexer;

	#[test]
	fn test_source_cursor_sink_for_string() {
		let source_text = "black white";
		let bump = Bump::default();
		let mut str = String::new();
		let mut transform = CursorToSourceCursorSink::new(source_text, &mut str);
		let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&bump, source_text, lexer);
		parser.parse_entirely::<ComponentValues>().output.unwrap().to_cursors(&mut transform);
		assert_eq!(str, "black white");
	}
}
