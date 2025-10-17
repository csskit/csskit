use crate::{
	AssociatedWhitespaceRules, Cursor, CursorSink, Kind, KindSet, QuoteStyle, SourceCursor, SourceCursorSink, Token,
	Whitespace,
};

/// This is a [CursorSink] that wraps a sink (`impl SourceCursorSink`) and on each [CursorSink::append()] call, will write
/// the contents of the cursor [Cursor] given into the given Writer - using the given `&'a str` as the original source.
/// This also attempts to write additional newlines and indentation into the Writer to create a more aesthetically
/// pleasing output. It can be used as a light-weight formatter for ToCursors structs.
pub struct CursorPrettyWriteSink<'a, T: SourceCursorSink<'a>> {
	source_text: &'a str,
	sink: T,
	last_token: Option<Token>,
	indent_level: u8,
	expand_tab: Option<u8>,
	quotes: QuoteStyle,
}

const SPACE_AFTER_KINDSET: KindSet = KindSet::new(&[Kind::Comma]);
const SPACE_BEFORE_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly]);
const NEWLINE_AFTER_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly, Kind::RightCurly, Kind::Semicolon]);
const INCREASE_INDENT_LEVEL_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly]);
const DECREASE_INDENT_LEVEL_KINDSET: KindSet = KindSet::new(&[Kind::RightCurly]);

impl<'a, T: SourceCursorSink<'a>> CursorPrettyWriteSink<'a, T> {
	pub fn new(source_text: &'a str, sink: T, expand_tab: Option<u8>, quotes: QuoteStyle) -> Self {
		Self { source_text, sink, last_token: None, indent_level: 0, expand_tab, quotes }
	}

	fn space_before(first: Token, second: Token) -> bool {
		// CSS demands it
		first.needs_separator_for(second)
		// It's a kind which might like some space around it.
		|| (second != Kind::Whitespace && (first == SPACE_AFTER_KINDSET || first == '>' || first == '<' || first == '+' || first == '-'))
	}

	fn space_after(first: Token, second: Token) -> bool {
		// It's a kind which might like some space around it.
		first != Kind::Whitespace
			&& first != AssociatedWhitespaceRules::BanAfter
			&& (second == SPACE_BEFORE_KINDSET || second == '>' || second == '<')
	}

	fn newline_after(first: Token, second: Token) -> bool {
		!(
			// Don't create a newline for kinds that don't need one!
			first != NEWLINE_AFTER_KINDSET ||
			// Don't create a newline between `{}` with no inner content.
			first == '{' && second == '}'
		)
	}

	fn write(&mut self, c: SourceCursor<'a>) {
		let token = c.token();
		if token == INCREASE_INDENT_LEVEL_KINDSET {
			self.indent_level += 1;
		} else if token == DECREASE_INDENT_LEVEL_KINDSET && self.indent_level > 0 {
			self.indent_level -= 1;
		}
		if let Some(last) = self.last_token {
			if Self::newline_after(last, token) {
				self.sink.append(SourceCursor::NEWLINE);
			}
			if Self::newline_after(last, token)
				|| last == Kind::Whitespace && last.whitespace_style() == Whitespace::Newline
			{
				let (c, count) = if let Some(len) = self.expand_tab {
					(SourceCursor::SPACE, self.indent_level * len)
				} else {
					(SourceCursor::TAB, self.indent_level)
				};
				for _ in 0..count {
					self.sink.append(c);
				}
			} else if Self::space_before(last, token) || Self::space_after(last, token) {
				self.sink.append(SourceCursor::SPACE);
			}
		}
		self.last_token = Some(token);
		// Normalize quotes
		if c.token() == Kind::String {
			self.sink.append(c.with_quotes(self.quotes))
		} else {
			self.sink.append(c);
		}
	}
}

impl<'a, T: SourceCursorSink<'a>> CursorSink for CursorPrettyWriteSink<'a, T> {
	fn append(&mut self, c: Cursor) {
		self.write(SourceCursor::from(c, c.str_slice(self.source_text)))
	}
}

impl<'a, T: SourceCursorSink<'a>> SourceCursorSink<'a> for CursorPrettyWriteSink<'a, T> {
	fn append(&mut self, c: SourceCursor<'a>) {
		self.write(c)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::ToCursors;
	use crate::{ComponentValues, EmptyAtomSet, Parser};
	use bumpalo::Bump;
	use css_lexer::Lexer;

	macro_rules! assert_format {
		($struct: ident, $before: literal, $after: literal) => {
			let source_text = $before;
			let bump = Bump::default();
			let mut sink = String::new();
			let mut stream = CursorPrettyWriteSink::new(source_text, &mut sink, None, QuoteStyle::Double);
			let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
			let mut parser = Parser::new(&bump, source_text, lexer);
			parser.parse_entirely::<$struct>().output.unwrap().to_cursors(&mut stream);
			assert_eq!(sink, $after.trim());
		};
		($before: literal, $after: literal) => {
			let source_text = $before;
			let bump = Bump::default();
			let mut sink = String::new();
			let mut stream = CursorPrettyWriteSink::new(source_text, &mut sink, None, QuoteStyle::Double);
			let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
			let mut parser = Parser::new(&bump, source_text, lexer);
			parser.parse_entirely::<ComponentValues>().output.unwrap().to_cursors(&mut stream);
			assert_eq!(sink, $after.trim());
		};
	}

	#[test]
	fn test_basic() {
		assert_format!(
			"foo{bar: baz();}",
			r#"
foo {
	bar: baz();
}
"#
		);
	}

	#[test]
	fn test_does_not_repeat_whitespace() {
		assert_format!(
			"foo {bar: baz();}",
			r#"
foo {
	bar: baz();
}
"#
		);
	}

	#[test]
	fn test_can_handle_nested_curlies() {
		assert_format!(
			"foo {bar{baz{bing{}}}}",
			r#"
foo {
	bar {
		baz {
			bing {}
		}
	}
}
"#
		);
	}

	#[test]
	fn test_does_not_ignore_whitespace_in_selectors() {
		assert_format!("div dialog:modal>td p a", "div dialog:modal > td p a");
	}

	#[test]
	fn test_does_normalizes_quotes() {
		assert_format!(
			"foo[attr='bar']{baz:'bing';}",
			r#"
foo[attr="bar"] {
	baz:"bing";
}
"#
		);
	}
}
