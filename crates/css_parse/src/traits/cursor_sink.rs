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

impl<'a> SourceCursorSink<'a> for Vec<'a, SourceCursor<'a>> {
	fn append(&mut self, c: SourceCursor<'a>) {
		// If two adjacent cursors which could not be re-tokenized in the same way if they were written out adjacently occur
		// then they should be separated by some token.
		if let Some(last) = self.last()
			&& last.token().needs_separator_for(c.token())
		{
			self.push(SourceCursor::from(SEPARATOR, " "));
		}
		self.push(c)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{ComponentValues, EmptyAtomSet, Parser, ToCursors};
	use bumpalo::Bump;
	use css_lexer::Lexer;

	#[test]
	fn test_cursor_sink_for_vec() {
		use std::fmt::Write;
		let source_text = "black white";
		let bump = Bump::default();
		let mut stream = Vec::new_in(&bump);
		let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&bump, source_text, lexer);
		parser.parse_entirely::<ComponentValues>().output.unwrap().to_cursors(&mut stream);
		let mut str = String::new();
		for c in stream {
			write!(&mut str, "{}", SourceCursor::from(c, c.str_slice(source_text))).unwrap();
		}
		assert_eq!(str, "black white");
	}
}
