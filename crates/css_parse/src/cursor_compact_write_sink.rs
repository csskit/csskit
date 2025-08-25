use crate::{CursorSink, SourceCursor, SourceCursorSink};
use core::fmt::{Result, Write};
use css_lexer::{Cursor, Kind, KindSet, QuoteStyle, Token};

/// This is a [CursorSink] that wraps a Writer (`impl fmt::Write`) and on each [CursorSink::append()] call, will write
/// the contents of the cursor [Cursor] given into the given Writer - using the given `&'a str` as the original source.
/// Some tokens will not be output, and Whitespace tokens will always write out as a single `' '`. It can be used as a
/// light-weight minifier for ToCursors structs.
pub struct CursorCompactWriteSink<'a, T: Write> {
	source_text: &'a str,
	writer: T,
	last_token: Option<Token>,
	pending: Option<Cursor>,
	err: Result,
}

const PENDING_KINDSET: KindSet = KindSet::new(&[Kind::Semicolon, Kind::Whitespace]);
const REDUNDANT_SEMI_KINDSET: KindSet = KindSet::new(&[Kind::Semicolon, Kind::Colon, Kind::RightCurly]);
const REDUNDANT_WHITESPACE_KINDSET: KindSet =
	KindSet::new(&[Kind::Whitespace, Kind::Colon, Kind::Delim, Kind::LeftCurly, Kind::RightCurly]);

impl<'a, T: Write> CursorCompactWriteSink<'a, T> {
	pub fn new(source_text: &'a str, writer: T) -> Self {
		Self { source_text, writer, last_token: None, pending: None, err: Ok(()) }
	}

	fn write(&mut self, c: Cursor, source: &'a str) -> Result {
		self.err?;
		if let Some(prev) = self.pending {
			self.pending = None;
			let is_redundant_semi = prev.token() == Kind::Semicolon
				&& (c.token() == REDUNDANT_SEMI_KINDSET
					|| self.last_token.is_some_and(|c| c == REDUNDANT_SEMI_KINDSET));
			let is_redundant_whitespace = self.last_token.is_none()
				|| prev.token() == Kind::Whitespace
					&& (c.token() == REDUNDANT_WHITESPACE_KINDSET
						|| self.last_token.is_some_and(|c| c == REDUNDANT_WHITESPACE_KINDSET));
			if !is_redundant_semi && !is_redundant_whitespace {
				self.last_token = Some(prev.into());
				if prev == Kind::Whitespace {
					// Whitespace can be minimised to a single space
					self.writer.write_char(' ')?;
				} else {
					prev.write_str(source, &mut self.writer)?;
				}
			}
		}
		if c.token() == PENDING_KINDSET {
			self.pending = Some(c);
			return Ok(());
		}
		if let Some(last) = self.last_token {
			if last.needs_separator_for(c.token()) {
				self.writer.write_char(' ')?;
			}
		}
		self.last_token = Some(c.token());
		let mut write_c = c;
		if c.token().quote_style() == QuoteStyle::Single {
			dbg!(c);
			write_c = dbg!(Cursor::new(c.offset(), c.token().with_quotes(QuoteStyle::Double)));
		}
		write_c.write_str(source, &mut self.writer)?;
		Ok(())
	}
}

impl<'a, T: Write> CursorSink for CursorCompactWriteSink<'a, T> {
	fn append(&mut self, c: Cursor) {
		self.err = self.write(c, self.source_text);
	}
}

impl<'a, T: Write> SourceCursorSink<'a> for CursorCompactWriteSink<'a, T> {
	fn append(&mut self, c: SourceCursor<'a>) {
		self.err = self.write(c.cursor(), c.source());
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{ComponentValues, ToCursors, parse};
	use bumpalo::Bump;

	macro_rules! assert_format {
		($before: literal, $after: literal) => {
			assert_format!(ComponentValues, $before, $after);
		};
		($struct: ident, $before: literal, $after: literal) => {
			let source_text = $before;
			let bump = Bump::default();
			let mut writer = String::new();
			let mut stream = CursorCompactWriteSink::new(source_text, &mut writer);
			parse!(in bump &source_text as $struct).output.unwrap().to_cursors(&mut stream);
			assert_eq!(writer, $after.trim());
		};
	}

	#[test]
	fn test_basic() {
		assert_format!("foo{bar: baz();}", r#"foo{bar:baz()}"#);
	}

	#[test]
	fn test_removes_redundant_semis() {
		assert_format!("foo{bar: 1;;;;bing: 2;;;}", r#"foo{bar:1;bing:2}"#);
	}

	#[test]
	fn normalizes_quotes() {
		assert_format!("bar:'baz';bing:'quux';x:url('foo')", r#"bar:"baz";bing:"quux";x:url("foo")"#);
	}

	#[test]
	fn test_does_not_ignore_whitespace_in_selectors() {
		assert_format!("div dialog:modal >td p a", "div dialog:modal>td p a");
	}

	#[test]
	fn test_compacts_whitespace() {
		assert_format!(
			r#"
		body   >   div {
			bar:  baz
		}
		"#,
			"body>div{bar:baz}"
		);
	}
}
