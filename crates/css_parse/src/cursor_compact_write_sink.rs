use crate::{Cursor, CursorSink, Kind, KindSet, QuoteStyle, SourceCursor, SourceCursorSink, Token};

/// This is a [CursorSink] that wraps a sink (`impl SourceCursorSink`) and on each [CursorSink::append()] call, will write
/// the contents of the cursor [Cursor] given into the given sink - using the given `&'a str` as the original source.
/// Some tokens will not be output, and Whitespace tokens will always write out as a single `' '`. It can be used as a
/// light-weight minifier for ToCursors structs.
pub struct CursorCompactWriteSink<'a, T: SourceCursorSink<'a>> {
	source_text: &'a str,
	sink: T,
	last_token: Option<Token>,
	pending: Option<SourceCursor<'a>>,
}

const PENDING_KINDSET: KindSet = KindSet::new(&[Kind::Semicolon, Kind::Whitespace]);
const REDUNDANT_SEMI_KINDSET: KindSet = KindSet::new(&[Kind::Semicolon, Kind::Colon, Kind::RightCurly]);
// Tokens where whitespace immediately before them can be removed
const NO_WHITESPACE_BEFORE_KINDSET: KindSet =
	KindSet::new(&[Kind::Whitespace, Kind::Colon, Kind::Delim, Kind::LeftCurly, Kind::RightCurly]);
// Tokens where whitespace immediately after them can be removed
const NO_WHITESPACE_AFTER_KINDSET: KindSet =
	KindSet::new(&[Kind::Comma, Kind::RightParen, Kind::RightCurly, Kind::LeftCurly, Kind::Colon]);

impl<'a, T: SourceCursorSink<'a>> CursorCompactWriteSink<'a, T> {
	pub fn new(source_text: &'a str, sink: T) -> Self {
		Self { source_text, sink, last_token: None, pending: None }
	}

	fn write(&mut self, c: SourceCursor<'a>) {
		let mut skip_separator_check = false;
		if let Some(prev) = self.pending {
			self.pending = None;
			let is_redundant_semi = prev == Kind::Semicolon
				&& (c == REDUNDANT_SEMI_KINDSET || self.last_token.is_some_and(|c| c == REDUNDANT_SEMI_KINDSET));
			let no_whitespace_after_last =
				prev == Kind::Whitespace && self.last_token.is_some_and(|c| c == NO_WHITESPACE_AFTER_KINDSET);
			let is_redundant_whitespace = self.last_token.is_none()
				|| prev == Kind::Whitespace && (c == NO_WHITESPACE_BEFORE_KINDSET || no_whitespace_after_last);
			if !is_redundant_semi && !is_redundant_whitespace {
				self.last_token = Some(prev.token());
				if prev == Kind::Whitespace {
					// Whitespace can be minimised to a single space
					self.sink.append(SourceCursor::SPACE);
				} else {
					self.sink.append(prev);
				}
			} else if no_whitespace_after_last {
				// If we're skipping whitespace because the last token doesn't need whitespace after it,
				// don't add it back via needs_separator_for
				skip_separator_check = true;
			}
		}
		if c == PENDING_KINDSET {
			self.pending = Some(c);
			return;
		}
		if !skip_separator_check
			&& let Some(last) = self.last_token
			&& last.needs_separator_for(c.token())
		{
			self.sink.append(SourceCursor::SPACE);
		}
		self.last_token = Some(c.token());
		// Normalize quotes
		if c == Kind::String {
			self.sink.append(c.with_quotes(QuoteStyle::Double))
		} else {
			self.sink.append(c);
		}
	}
}

impl<'a, T: SourceCursorSink<'a>> CursorSink for CursorCompactWriteSink<'a, T> {
	fn append(&mut self, c: Cursor) {
		self.write(SourceCursor::from(c, c.str_slice(self.source_text)))
	}
}

impl<'a, T: SourceCursorSink<'a>> SourceCursorSink<'a> for CursorCompactWriteSink<'a, T> {
	fn append(&mut self, c: SourceCursor<'a>) {
		self.write(c)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{ComponentValues, EmptyAtomSet, Parser, ToCursors};
	use bumpalo::Bump;
	use css_lexer::Lexer;

	macro_rules! assert_format {
		($before: literal, $after: literal) => {
			assert_format!(ComponentValues, $before, $after);
		};
		($struct: ident, $before: literal, $after: literal) => {
			let source_text = $before;
			let bump = Bump::default();
			let mut sink = String::new();
			let mut stream = CursorCompactWriteSink::new(source_text, &mut sink);
			let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
			let mut parser = Parser::new(&bump, source_text, lexer);
			parser.parse_entirely::<$struct>().output.unwrap().to_cursors(&mut stream);
			assert_eq!(sink, $after.trim());
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
	fn test_does_not_ignore_whitespace_component_values() {
		assert_format!("div dialog:modal > td p a", "div dialog:modal > td p a");
	}

	#[test]
	fn test_compacts_whitespace() {
		assert_format!(
			r#"
		body   >   div {
			bar:  baz
		}
		"#,
			"body > div{bar:baz}"
		);
	}

	#[test]
	fn test_does_not_compact_whitespace_resulting_in_new_ident() {
		assert_format!("12px - 1px", "12px - 1px");
	}

	#[test]
	fn test_removes_whitespace_after_comma() {
		assert_format!("foo(a, b, c)", "foo(a,b,c)");
		assert_format!("rgb(255, 128, 0)", "rgb(255,128,0)");
	}

	#[test]
	fn test_removes_whitespace_after_right_paren() {
		assert_format!("foo() bar", "foo()bar");
		assert_format!("rgb(0, 0, 0) solid", "rgb(0,0,0)solid");
	}

	#[test]
	fn test_removes_whitespace_after_right_curly() {
		assert_format!("@media screen{} .foo{}", "@media screen{}.foo{}");
	}
}
