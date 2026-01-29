use crate::{Cursor, CursorSink, Kind, SourceCursor, SourceCursorSink, Token};

/// This is a [CursorSink] that wraps a sink (`impl SourceCursorSink`) and on each [CursorSink::append()] call, will write
/// the contents of the cursor [Cursor] given into the given sink - using the given `&'a str` as the original source.
/// Tokens will be expanded to their most verbose form. It can be used as an "anti-minifier" for ToCursors structs.
pub struct CursorExpandedWriteSink<'a, T: SourceCursorSink<'a>> {
	source_text: &'a str,
	sink: T,
	last_token: Option<Token>,
	extra_semicolons: u8,
	escape_idents: bool,
}

impl<'a, T: SourceCursorSink<'a>> CursorExpandedWriteSink<'a, T> {
	pub fn new(source_text: &'a str, sink: T) -> Self {
		Self { source_text, sink, last_token: None, extra_semicolons: 0, escape_idents: false }
	}

	/// Set the number of extra semicolons to add after each semicolon.
	pub fn with_extra_semicolons(mut self, count: u8) -> Self {
		self.extra_semicolons = count;
		self
	}

	/// Disable ident escaping for a milder expansion effect.
	pub fn with_escape_idents(mut self, escape: bool) -> Self {
		self.escape_idents = escape;
		self
	}

	fn write(&mut self, c: SourceCursor<'a>) {
		if let Some(last) = self.last_token
			&& last.needs_separator_for(c.token())
		{
			self.sink.append(SourceCursor::SPACE);
		}
		self.last_token = Some(c.token());
		let is_ident_like =
			matches!(c.token().kind(), Kind::Ident | Kind::Function | Kind::AtKeyword | Kind::Hash | Kind::String);
		let should_expand = matches!(c.token().kind(), Kind::Whitespace | Kind::Number | Kind::Dimension | Kind::Url)
			|| (is_ident_like && self.escape_idents);
		if should_expand {
			self.sink.append(c.expand());
		} else {
			self.sink.append(c);
		}
		// Add extra semicolons after each semicolon
		if c == Kind::Semicolon {
			for _ in 0..self.extra_semicolons {
				self.sink.append(SourceCursor::SEMICOLON);
			}
		}
	}
}

impl<'a, T: SourceCursorSink<'a>> CursorSink for CursorExpandedWriteSink<'a, T> {
	fn append(&mut self, c: Cursor) {
		self.write(SourceCursor::from(c, c.str_slice(self.source_text)))
	}
}

impl<'a, T: SourceCursorSink<'a>> SourceCursorSink<'a> for CursorExpandedWriteSink<'a, T> {
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

	macro_rules! assert_expand {
		($before: literal, $after: literal) => {
			assert_expand!(ComponentValues, $before, $after);
		};
		($struct: ident, $before: literal, $after: literal) => {
			let source_text = $before;
			let bump = Bump::default();
			let mut sink = String::new();
			let mut stream = CursorExpandedWriteSink::new(source_text, &mut sink).with_escape_idents(true);
			let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
			let mut parser = Parser::new(&bump, source_text, lexer);
			parser.parse_entirely::<$struct>().output.unwrap().to_cursors(&mut stream);
			assert_eq!(sink, $after);
		};
	}

	#[test]
	fn test_basic() {
		// foo -> \000066 \00006f \00006f (f=0x66, o=0x6f)
		// bar -> \000062 \000061 \000072 (b=0x62, a=0x61, r=0x72)
		// baz -> \000062 \000061 \00007a (z=0x7a)
		assert_expand!(
			"foo{bar: baz();}",
			r#"\000066 \00006f \00006f {\000062 \000061 \000072 :    \000062 \000061 \00007a ();}"#
		);
	}

	#[test]
	fn test_expands_numbers() {
		// opacity -> o=0x6f p=0x70 a=0x61 c=0x63 i=0x69 t=0x74 y=0x79
		assert_expand!(
			"opacity: 0.8",
			r#"\00006f \000070 \000061 \000063 \000069 \000074 \000079 :    +8.00000000000000e-0000000001"#
		);
		assert_expand!(
			"opacity: .5",
			r#"\00006f \000070 \000061 \000063 \000069 \000074 \000079 :    +5.00000000000000e-0000000001"#
		);
		// px -> p=0x70 x=0x78
		assert_expand!(
			"width: 1.0px",
			r#"\000077 \000069 \000064 \000074 \000068 :    +1.00000000000000e+0000000000\000070 \000078 "#
		);
	}

	#[test]
	fn test_expands_whitespace() {
		// a=0x61, b=0x62
		assert_expand!("a b", r#"\000061     \000062 "#);
	}

	#[test]
	fn test_expands_url() {
		assert_expand!("url(foo.png)", r#"url(   foo.png   )"#);
	}

	#[test]
	fn test_expands_strings() {
		// h=0x68 e=0x65 l=0x6c o=0x6f
		assert_expand!(
			r#"content: "hello""#,
			r#"\000063 \00006f \00006e \000074 \000065 \00006e \000074 :    '\000068 \000065 \00006c \00006c \00006f '"#
		);
	}
}
