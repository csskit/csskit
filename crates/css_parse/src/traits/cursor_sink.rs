use bumpalo::collections::Vec;
use css_lexer::{Cursor, Token};

/// This trait provides the generic `impl` that [ToCursors][crate::ToCursors] can use. This provides just enough API
/// surface for nodes to put the cursors they represent into some buffer which can later be read, the details of which
/// are elided.
pub trait CursorSink {
	fn append(&mut self, c: Cursor);
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
}
