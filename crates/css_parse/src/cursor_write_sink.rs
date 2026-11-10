use crate::{Cursor, CursorSink, Kind, SourceCursor, SourceCursorSink, Token};
use core::fmt::{Result, Write};

/// This is a [CursorSink] that wraps a Writer (`impl fmt::Write`) and on each [CursorSink::append()] call, will write
/// the contents of the cursor [Cursor] given into the given Writer - using the given `&'a str` as the original source.
/// This is useful as way to turn Cursors into Strings or [u8]s (or files or whatever else implements [Write]).
pub struct CursorWriteSink<'a, T: Write> {
	source_text: &'a str,
	writer: T,
	last_token: Option<Token>,
	last_end: Option<(*const u8, css_lexer::SourceOffset)>,
	unclosed: Option<&'static str>,
	err: Result,
}

impl<'a, T: Write> CursorWriteSink<'a, T> {
	pub fn new(source_text: &'a str, writer: T) -> Self {
		Self { source_text, writer, last_token: None, last_end: None, unclosed: None, err: Ok(()) }
	}

	fn write(&mut self, c: Cursor, source: &'a str) -> Result {
		self.err?;
		let adjacent_in_source = self.last_end == Some((source.as_ptr(), c.offset()));
		if let Some(last) = self.last_token
			&& !adjacent_in_source
			&& (last == Kind::BadString || last.needs_separator_for(c.token()))
		{
			self.writer.write_char(if last.char() == Some('\\') || last == Kind::BadString { '\n' } else { ' ' })?;
		}
		self.last_token = Some(c.token());
		self.last_end = (!c.token().is_synthetic()).then(|| (source.as_ptr(), c.end_offset()));
		if c.token().is_synthetic() {
			if let Some(close) = self.unclosed.take() {
				self.writer.write_str(close)?;
			}
			if let Some(char) = c.token().char() {
				self.writer.write_char(char)?;
			}
			return Ok(());
		}
		let str = c.str_slice(source);
		if !str.is_empty() {
			self.unclosed = Self::unclosed(c.token(), str);
		}
		self.writer.write_str(str)?;
		Ok(())
	}

	pub(crate) fn unclosed(token: Token, str: &str) -> Option<&'static str> {
		let escape_at_end = str.bytes().rev().take_while(|&byte| byte == b'\\').count() % 2 == 1;
		match token.kind() {
			Kind::String if !token.has_close_quote() => Some(match (token.quote_style(), escape_at_end) {
				(css_lexer::QuoteStyle::Double, false) => "\"",
				(css_lexer::QuoteStyle::Double, true) => "\n\"",
				(_, false) => "'",
				(_, true) => "\n'",
			}),
			Kind::Url | Kind::BadUrl if escape_at_end => Some("\u{FFFD})"),
			Kind::Url if !token.url_has_closing_paren() => Some(")"),
			Kind::BadUrl if !str.ends_with(')') => Some(")"),
			Kind::Comment if str.starts_with("/*") && (str.len() < 4 || !str.ends_with("*/")) => Some("*/"),
			Kind::Comment => None,
			_ if escape_at_end => Some("\u{FFFD}"),
			_ => None,
		}
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
	use crate::Arena;
	use crate::{ComponentValues, EmptyAtomSet, Parser, ToCursors};
	use css_lexer::Lexer;

	#[test]
	fn test() {
		let source_text = "foo{bar:baz();}";
		let alloc = Arena::default();
		let mut str = String::new();
		let mut stream = CursorWriteSink::new(source_text, &mut str);
		let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&alloc, source_text, lexer);
		parser.parse_entirely::<ComponentValues>().output.unwrap().to_cursors(&mut stream);
		assert_eq!(str, "foo{bar:baz();}");
	}

	#[test]
	fn backslash_delim_separator_is_not_an_escape() {
		let source_text = "\\\n";
		let mut str = String::new();
		let mut stream = CursorWriteSink::new(source_text, &mut str);
		let backslash = Token::BACKSLASH.with_associated_whitespace(crate::AssociatedWhitespaceRules::EnforceAfter);
		CursorSink::append(&mut stream, Cursor::new(css_lexer::SourceOffset(0), backslash));
		CursorSink::append(&mut stream, Cursor::new(css_lexer::SourceOffset(2), Token::EOF));
		assert_eq!(str, "\\\n");
	}

	#[test]
	fn bad_string_keeps_its_newline() {
		let source_text = "'a\nb";
		let mut str = String::new();
		let mut stream = CursorWriteSink::new(source_text, &mut str);
		for cursor in Lexer::new(&EmptyAtomSet::ATOMS, source_text).filter(|cursor| cursor.token() != Kind::Whitespace)
		{
			CursorSink::append(&mut stream, cursor);
		}
		assert_eq!(str, "'a\nb");
	}

	#[test]
	fn bad_string_at_eof_keeps_its_newline() {
		let source_text = "'a\n";
		let mut str = String::new();
		let mut stream = CursorWriteSink::new(source_text, &mut str);
		for cursor in Lexer::new(&EmptyAtomSet::ATOMS, source_text).filter(|cursor| cursor.token() != Kind::Whitespace)
		{
			CursorSink::append(&mut stream, cursor);
		}
		CursorSink::append(&mut stream, Cursor::new(css_lexer::SourceOffset(3), Token::EOF));
		assert_eq!(str, "'a\n");
	}

	#[test]
	fn source_adjacent_tokens_stay_adjacent() {
		let source_text = "\\3.02xl";
		let mut str = String::new();
		let mut stream = CursorWriteSink::new(source_text, &mut str);
		for cursor in Lexer::new(&EmptyAtomSet::ATOMS, source_text) {
			CursorSink::append(&mut stream, cursor);
		}
		assert_eq!(str, "\\3.02xl");
	}
}
