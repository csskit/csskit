use crate::{Cursor, CursorSink, SourceCursor, SourceCursorSink, Token};
use core::fmt::{Result, Write};

/// This is a [CursorSink] that wraps a Writer (`impl fmt::Write`) and on each [CursorSink::append()] call, will write
/// the contents of the cursor [Cursor] given into the given Writer - using the given `&'a str` as the original source.
/// This is useful as way to turn Cursors into Strings or [u8]s (or files or whatever else implements [Write]).
pub struct CursorWriteSink<'a, T: Write> {
	source_text: &'a str,
	writer: T,
	last_token: Option<Token>,
	err: Result,
}

impl<'a, T: Write> CursorWriteSink<'a, T> {
	pub fn new(source_text: &'a str, writer: T) -> Self {
		Self { source_text, writer, last_token: None, err: Ok(()) }
	}

	fn write(&mut self, c: Cursor, source: &'a str) -> Result {
		self.err?;
		if let Some(last) = self.last_token
			&& last.needs_separator_for(c.token())
		{
			self.writer.write_char(' ')?;
		}
		self.last_token = Some(c.token());
		self.writer.write_str(c.str_slice(source))?;
		Ok(())
	}
}

impl<'a, T: Write> CursorSink for CursorWriteSink<'a, T> {
	fn append(&mut self, c: Cursor) {
		self.err = self.write(c, self.source_text);
	}
}

impl<'a, T: Write> SourceCursorSink<'a> for CursorWriteSink<'a, T> {
	fn append(&mut self, c: SourceCursor<'a>) {
		self.err = self.write(c.cursor(), c.source());
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{ComponentValues, EmptyAtomSet, Parser, ToCursors};
	use bumpalo::Bump;

	#[test]
	fn test() {
		let source_text = "foo{bar:baz();}";
		let bump = Bump::default();
		let mut str = String::new();
		let mut stream = CursorWriteSink::new(source_text, &mut str);
		let mut parser = Parser::new(&bump, &EmptyAtomSet::ATOMS, source_text);
		parser.parse_entirely::<ComponentValues>().output.unwrap().to_cursors(&mut stream);
		assert_eq!(str, "foo{bar:baz();}");
	}
}
