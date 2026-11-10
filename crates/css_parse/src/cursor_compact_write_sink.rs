use crate::{
	AssociatedWhitespaceRules, Cursor, CursorSink, CursorWriteSink, Kind, KindSet, SourceCursor, SourceCursorSink,
	Token,
};

/// This is a [CursorSink] that wraps a sink (`impl SourceCursorSink`) and on each [CursorSink::append()] call, will write
/// the contents of the cursor [Cursor] given into the given sink - using the given `&'a str` as the original source.
/// Some tokens will not be output, and Whitespace tokens will always write out as a single `' '`. It can be used as a
/// light-weight minifier for ToCursors structs.
pub struct CursorCompactWriteSink<'a, T: SourceCursorSink<'a>> {
	source_text: &'a str,
	sink: T,
	last_token: Option<Token>,
	after_less_than_bang: bool,
	after_exponent_unit: bool,
	after_percentage: bool,
	after_escaped_double_dash: bool,
	pending: Option<SourceCursor<'a>>,
	pending_comment: Option<SourceCursor<'a>>,
	unclosed_url: Option<&'static str>,
}

// Tokens that get buffered as `pending` rather than emitted immediately.
const PENDING_KINDSET: KindSet = KindSet::new(&[Kind::Semicolon, Kind::Whitespace]);
// Whitespace immediately before these tokens is never meaningful in any CSS grammar, so it can always be removed -
// even when the parser marked it significant. `:` is absent because `.a :hover` is not `.a:hover`, and `)`/`]` only
// appear here as the *closing* token of a block, where `foo( a )` is always `foo(a)`.
const NO_WHITESPACE_BEFORE_KINDSET: KindSet = KindSet::new(&[
	Kind::Whitespace,
	Kind::Comma,
	Kind::Semicolon,
	Kind::LeftCurly,
	Kind::RightCurly,
	Kind::RightParen,
	Kind::RightSquare,
	Kind::Eof,
]);
// Whitespace immediately after these tokens is never meaningful. `)`/`]` are absent because `:is(a) b` and `[x] b`
// require a space, therefore we cannot reliable elide them.
const NO_WHITESPACE_AFTER_KINDSET: KindSet = KindSet::new(&[
	Kind::Comma,
	Kind::Semicolon,
	Kind::Colon,
	Kind::LeftCurly,
	Kind::RightCurly,
	Kind::LeftParen,
	Kind::LeftSquare,
]);

impl<'a, T: SourceCursorSink<'a>> CursorCompactWriteSink<'a, T> {
	pub fn new(source_text: &'a str, sink: T) -> Self {
		Self {
			source_text,
			sink,
			last_token: None,
			after_less_than_bang: false,
			after_exponent_unit: false,
			after_percentage: false,
			after_escaped_double_dash: false,
			pending: None,
			pending_comment: None,
			unclosed_url: None,
		}
	}

	/// Would emitting `next` immediately after `last_token` change tokenisation?
	#[inline]
	fn needs_separator(&self, next: Token) -> bool {
		self.last_token.is_some_and(|t| {
			t.needs_separator_for(next)
				&& !(self.after_percentage
					&& next.char() == Some('-')
					&& next != AssociatedWhitespaceRules::EnforceBefore)
		}) || (self.after_exponent_unit && matches!(next.kind(), Kind::Number | Kind::Dimension) && next.has_sign())
			|| (self.after_escaped_double_dash && next.char() == Some('>'))
	}

	/// True when the previously-emitted token never needs a separator after it
	/// (e.g. `,`, `)`, `{`). Whitespace can be dropped and re-injection skipped.
	fn last_forbids_ws_after(&self) -> bool {
		self.last_token.is_some_and(|t| t == NO_WHITESPACE_AFTER_KINDSET)
	}

	/// True when the previously-emitted token only ends at a newline (a BadString, or a `\` delim).
	#[inline]
	fn last_needs_newline(&self) -> bool {
		self.last_token.is_some_and(|t| t == Kind::BadString || (t == Kind::Delim && t.char() == Some('\\')))
	}

	/// True when `next` is a `!` that would directly follow a `<`.
	fn bang_after_less_than(&self, next: Token) -> bool {
		next.char() == Some('!') && self.last_token.is_some_and(|t| t.char() == Some('<'))
	}

	/// True when emitting `next` straight after the last token would form a CDO (`<!--`).
	fn forms_cdo(&self, next: SourceCursor<'a>) -> bool {
		let token = next.token();
		self.bang_after_less_than(token)
			|| (self.after_less_than_bang
				&& (token == Kind::Ident || token == Kind::CdcOrCdo)
				&& next.source().starts_with("--"))
	}

	/// True when emitting `next` straight after the last token would join two delims the parser reads as one
	/// operator (`^=`, `||`, `*|`).
	fn forms_double_delim(&self, next: SourceCursor<'a>) -> bool {
		let Some(last) = self.last_token.filter(|t| *t == Kind::Delim).and_then(|t| t.char()) else {
			return false;
		};
		match next.token().char().filter(|_| next.token() == Kind::Delim) {
			Some('=') => matches!(last, '<' | '>' | '=' | '~' | '|' | '^' | '$' | '*'),
			Some('|') => matches!(last, '*' | '|'),
			_ => false,
		}
	}

	fn emit(&mut self, c: SourceCursor<'a>) {
		if c.token().is_synthetic()
			&& let Some(close) = self.unclosed_url.take()
		{
			let offset = c.cursor().offset();
			if close.starts_with('\u{FFFD}') {
				self.sink.append(SourceCursor::from(Cursor::new(offset, Token::REPLACEMENT_CHARACTER), "\u{FFFD}"));
			}
			self.sink.append(SourceCursor::from(Cursor::new(offset, Token::RIGHT_PAREN), ")"));
		} else if c.token() != Kind::Whitespace {
			self.unclosed_url = match c.token().kind() {
				Kind::Url | Kind::BadUrl => CursorWriteSink::<String>::unclosed(c.token(), c.source()),
				_ => None,
			};
		}
		let token = c.token();
		self.after_exponent_unit =
			token == Kind::Dimension && token.len() - token.numeric_len() == 1 && c.source().ends_with(['e', 'E']);
		self.after_percentage =
			token == Kind::Dimension && token.len() - token.numeric_len() == 1 && c.source().ends_with('%');
		self.after_escaped_double_dash =
			token == Kind::Ident && token.contains_escape_chars() && c.eq_ignore_ascii_case("--");
		self.after_less_than_bang = self.bang_after_less_than(token);
		self.last_token = Some(c.token());
		self.sink.append(c);
	}

	fn write(&mut self, c: SourceCursor<'a>) {
		let kind = c.token().kind();
		if kind == Kind::Comment {
			let can_separate =
				self.pending.is_none() && self.pending_comment.is_none() && !self.last_forbids_ws_after();
			if can_separate {
				self.pending_comment = Some(c);
			}
			return;
		}
		if self.pending_comment.take().is_some()
			&& kind != Kind::Whitespace
			&& kind != Kind::Eof
			&& !self.last_needs_newline()
			&& (self.forms_cdo(c)
				|| self.forms_double_delim(c)
				|| self.last_token.is_some_and(|t| {
					t.with_associated_whitespace(AssociatedWhitespaceRules::none()).needs_separator_for(c.token())
				})) {
			self.emit(SourceCursor::EMPTY_COMMENT);
		}

		if kind != PENDING_KINDSET && self.last_needs_newline() {
			self.pending.take_if(|p| p.token() == Kind::Whitespace);
			self.emit(SourceCursor::NEWLINE);
		}

		if kind == Kind::Whitespace && self.pending.is_some_and(|c| c == Kind::Semicolon) {
			return;
		}

		let enforce_before = c.token() == AssociatedWhitespaceRules::EnforceBefore;
		let significant_pending_whitespace = self.pending.is_some_and(|p| p.token().whitespace_is_significant());
		let suppress_separator = self.last_forbids_ws_after()
			&& !enforce_before
			&& !(significant_pending_whitespace
				&& self.last_token.is_some_and(|t| t == Kind::Colon)
				&& matches!(kind, Kind::Ident | Kind::Function));
		let mut separated = false;
		let before_separator = self.last_token;
		if let Some(prev) = self.pending.take() {
			let keep = match prev.token().kind() {
				Kind::Semicolon => {
					let redundant_after = if kind == Kind::Eof {
						KindSet::RIGHT_CURLY_OR_SEMICOLON
					} else {
						KindSet::new(&[Kind::Semicolon])
					};
					kind != KindSet::RIGHT_CURLY_OR_SEMICOLON && self.last_token.is_some_and(|t| t != redundant_after)
				}
				_ if suppress_separator || (!enforce_before && kind == NO_WHITESPACE_BEFORE_KINDSET) => false,
				_ => {
					(significant_pending_whitespace && self.last_token.is_some())
						|| self.needs_separator(c.token())
						|| self.forms_cdo(c)
						|| self.forms_double_delim(c)
				}
			};
			if keep {
				self.emit(prev.compact());
				separated = true;
			}
		}

		if kind == PENDING_KINDSET {
			// Adjacent whitespace tokens collapse into one, which stays significant if either part was.
			self.pending = Some(if significant_pending_whitespace && kind == Kind::Whitespace {
				c.with_significant_whitespace(true)
			} else {
				c
			});
			return;
		}
		if kind == Kind::Eof {
			return;
		}

		if !suppress_separator && self.needs_separator(c.token()) {
			self.sink.append(SourceCursor::SPACE);
			separated = true;
		}

		let out = if kind == Kind::String {
			c.with_quotes(c.fewest_escapes_quotes()).compact()
		} else if matches!(kind, Kind::Number | Kind::Dimension)
			&& before_separator.is_some_and(|t| {
				let positive_sign = c.token().has_sign() && !c.token().value().is_sign_negative();
				match t.kind() {
					Kind::Delim => match t.char() {
						Some('.' | '#') => !separated,
						Some('+' | '-') => positive_sign,
						_ => false,
					},
					kind => {
						!separated
							&& positive_sign
							&& kind
								== KindSet::new(&[
									Kind::Ident,
									Kind::AtKeyword,
									Kind::Hash,
									Kind::Number,
									Kind::Dimension,
								])
					}
				}
			}) {
			c
		} else {
			c.compact()
		};
		self.emit(out);
	}
}

impl<'a, T: SourceCursorSink<'a>> Drop for CursorCompactWriteSink<'a, T> {
	fn drop(&mut self) {
		if let Some(prev) = self.pending.take()
			&& prev == Kind::Semicolon
		{
			if self.last_needs_newline() {
				self.emit(SourceCursor::NEWLINE);
			}
			self.emit(prev);
		}
	}
}

impl<'a, T: SourceCursorSink<'a>> CursorSink for CursorCompactWriteSink<'a, T> {
	fn append(&mut self, c: Cursor) {
		self.write(SourceCursor::from(c, c.str_slice(self.source_text)))
	}

	fn append_trivia(&mut self, c: Cursor) {
		// Declaration and at-rule terminators are AST cursors. Skipped semicolons are stray.
		if c != Kind::Semicolon {
			CursorSink::append(self, c);
		}
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
	use crate::Arena;
	use crate::{ComponentValues, EmptyAtomSet, Parser, ToCursors};
	use css_lexer::Lexer;

	macro_rules! assert_format {
		($before: literal, $after: literal) => {
			assert_format!(ComponentValues, $before, $after);
		};
		($struct: ident, $before: literal, $after: literal) => {
			let source_text = $before;
			let alloc = Arena::default();
			let mut sink = String::new();
			{
				let mut stream = CursorCompactWriteSink::new(source_text, &mut sink);
				let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
				let mut parser = Parser::new(&alloc, source_text, lexer);
				parser.parse_entirely::<$struct>().with_trivia().to_cursors(&mut stream);
			}
			assert_eq!(sink, $after.trim());
		};
	}

	#[test]
	fn test_basic() {
		assert_format!("foo{bar: baz();}", r#"foo{bar:baz()}"#);
	}

	#[test]
	fn keeps_space_after_colon_in_prelude() {
		assert_format!("a: hover{}", "a: hover{}");
		assert_format!("a: /**/ has(> b){}", "a: has(> b){}");
		assert_format!("a{b: c}", "a{b:c}");
		assert_format!("supports(foo: bar)", "supports(foo:bar)");
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
		assert_format!(".a   .b", ".a .b");
	}

	#[test]
	fn test_does_not_compact_whitespace_resulting_in_new_ident() {
		assert_format!("12px - 1px", "12px - 1px");
	}

	#[test]
	fn test_keeps_minus_adjacent_to_percentage() {
		assert_format!("calc(1%- 1px - 1px)", "calc(1%- 1px - 1px)");
		assert_format!("1%- x", "1%- x");
		assert_format!("1px-x", "1px-x");
		assert_format!("1px - x", "1px - x");
	}

	#[test]
	fn test_keeps_plus_before_signed_number() {
		assert_format!("++1", "++1");
		assert_format!("++1px", "++1px");
		assert_format!("a ++1", "a ++1");
		assert_format!("+ +1", "+ +1");
		assert_format!("- +1px", "- +1px");
		assert_format!("n+3", "n+3");
		assert_format!("1+2", "1 2");
		assert_format!("1px+2px", "1px+2px");
		assert_format!("n +3", "n 3");
		assert_format!("#+1", "#+1");
		assert_format!("#+1px", "#+1px");
	}

	#[test]
	fn test_keeps_number_source_after_dot_or_hash_delim() {
		assert_format!("..0x", "..0x");
		assert_format!("..1e1", "..1e1");
		assert_format!("#.1e1", "#.1e1");
	}

	#[test]
	fn test_separates_exponent_unit_from_signed_number() {
		assert_format!("1e/**/+1", "1e 1");
		assert_format!("1E +1", "1E 1");
		assert_format!("1e1e+1", "10e 1");
		assert_format!("1ex+1", "1ex+1");
	}

	#[test]
	fn test_separates_escaped_double_dash_from_greater_than() {
		assert_format!(r"\-->", r"-- >");
		assert_format!(r"-\->", r"-\- >");
		assert_format!(r"\--a>", r"--a>");
		assert_format!(r"\--/**/>", r"-- >");
	}

	#[test]
	fn test_removes_whitespace_after_comma() {
		assert_format!("foo(a, b, c)", "foo(a,b,c)");
		assert_format!("rgb(255, 128, 0)", "rgb(255,128,0)");
	}

	#[test]
	fn test_keeps_whitespace_after_right_paren_and_square() {
		assert_format!("foo() bar", "foo() bar");
		assert_format!("rgb(0, 0, 0) solid", "rgb(0,0,0) solid");
		assert_format!("[x] bar", "[x] bar");
	}

	#[test]
	fn test_removes_whitespace_after_right_curly() {
		assert_format!("@media screen{} .foo{}", "@media screen{}.foo{}");
	}

	#[test]
	fn test_compacts_numbers_with_leading_zero() {
		assert_format!("opacity: 0.8", "opacity:.8");
		assert_format!("opacity: 0.5", "opacity:.5");
		assert_format!("opacity: 0.123", "opacity:.123");
	}

	#[test]
	fn test_compacts_numbers_with_trailing_zeros() {
		assert_format!("width: 1.0px", "width:1px");
		assert_format!("width: 1.500px", "width:1.5px");
		assert_format!("width: 2.000px", "width:2px");
	}

	#[test]
	fn test_compacts_numbers_with_sign() {
		assert_format!("margin: -0.5px", "margin:-.5px");
		assert_format!("margin: +1.5px", "margin:1.5px");
		assert_format!("margin: +0.8px", "margin:.8px");
	}

	#[test]
	fn test_compacts_edge_case_numbers() {
		assert_format!("opacity: 0", "opacity:0");
		assert_format!("opacity: 1", "opacity:1");
	}

	#[test]
	fn test_does_not_change_numbers_without_optimization() {
		assert_format!("width: 123px", "width:123px");
		assert_format!("width: .5px", "width:.5px");
	}

	#[test]
	fn test_keeps_numbers_beyond_f32_precision() {
		assert_format!("width: 4e38px", "width:4e38px");
		assert_format!("opacity: -4e38", "opacity:-4e38");
		assert_format!("width: 3e38px", "width:3e38px");
		assert_format!("z-index: 16777217", "z-index:16777217");
		assert_format!("z-index: -16777217", "z-index:-16777217");
		assert_format!("z-index: 16777215", "z-index:16777215");
	}

	#[test]
	fn test_compacts_numbers_to_scientific_notation_when_shorter() {
		assert_format!("width: 0.000001px", "width:1e-6px");
		assert_format!("width: 0.000012px", "width:12e-6px");
		assert_format!("width: -0.0001px", "width:-1e-4px");
		assert_format!("width: 0.001px", "width:.001px");
		assert_format!("width: 100px", "width:100px");
		assert_format!("width: 1000px", "width:1e3px");
		assert_format!("width: 1234px", "width:1234px");
		assert_format!("width: 12000000px", "width:12e6px");
		assert_format!("width: 1e-7px", "width:1e-7px");
		assert_format!("opacity: 0.00001", "opacity:1e-5");
		assert_format!("opacity: 1000.0", "opacity:1e3");
		assert_format!("margin: -0px", "margin:-0px");
	}

	#[test]
	fn test_preserves_numeric_type_of_unknown_values() {
		assert_format!("z-index: 1000000", "z-index:1000000");
		assert_format!("z-index: 1.0", "z-index:1.0");
		assert_format!("z-index: 1.000", "z-index:1.0");
		assert_format!("z-index: 0.0", "z-index:0.0");
		assert_format!("z-index: -2.0", "z-index:-2.0");
		assert_format!("z-index: 100.0", "z-index:1e2");
		assert_format!("a: 1000n", "a:1000n");
		assert_format!("a: 1000N", "a:1000N");
		assert_format!("a: 1000n-1", "a:1000n-1");
		assert_format!("a: 2.0n", "a:2.0n");
		assert_format!("a: 2.0px", "a:2px");
	}

	#[test]
	fn test_preserves_trailing_semicolons() {
		assert_format!("foo;", "foo;");
	}

	#[test]
	fn test_removes_trailing_semis_when_after_curly() {
		assert_format!("{foo};", "{foo}");
	}

	#[test]
	fn test_preserves_semis_after_curly_before_more_tokens() {
		assert_format!("a{--x:{x};background:tan}", "a{--x:{x};background:tan}");
		assert_format!("a{--x:{x} ; background:tan}", "a{--x:{x};background:tan}");
		assert_format!("a{--x:{x};}", "a{--x:{x}}");
		assert_format!("a{--x:{x};;;background:tan}", "a{--x:{x};background:tan}");
	}

	#[test]
	fn test_drops_trailing_whitespace() {
		assert_format!("foo  ", "foo");
		assert_format!("foo; ", "foo;");
	}

	#[test]
	fn test_preserves_comment_absence_in_custom_properties() {
		assert_format!("div{--bar:a/**/b}", "div{--bar:a/**/b}");
		assert_format!("div { --bar: a/**/b }", "div{--bar:a/**/b}");
		assert_format!("div{--bar:a /* comment */ b}", "div{--bar:a b}");
		assert_format!("div{--bar:a/**//**/b}", "div{--bar:a/**/b}");
		assert_format!("div{--bar:a /* x */ /* y */ b}", "div{--bar:a b}");
		assert_format!("div{/*comment*/--bar:a}", "div{--bar:a}");
		assert_format!("div{--bar:a/*comment*/}", "div{--bar:a}");
		assert_format!("div{--bar:a  /**/  b}", "div{--bar:a b}");
		assert_format!("@container style(--bar:a/**/b){}", "@container style(--bar:a/**/b){}");
		assert_format!("@container style(--bar:a/**/b){}", "@container style(--bar:a/**/b){}");
		assert_format!("foo /**/bar", "foo bar");
		assert_format!("foo{/**/bar}", "foo{bar}");
		assert_format!("foo:/**/bar", "foo:bar");
		assert_format!("foo(/**/bar)", "foo(bar)");
		assert_format!("foo/**/,bar", "foo,bar");
		assert_format!("/**/foo", "foo");
		assert_format!("div{--bar:a/*some really long comment text*/b}", "div{--bar:a/**/b}");
	}

	#[test]
	fn test_at_rule_no_space_before_paren() {
		assert_format!(
			"@media(prefers-reduced-motion:no-preference){:root{}}",
			"@media(prefers-reduced-motion:no-preference){:root{}}"
		);
		assert_format!(
			"@media (prefers-reduced-motion:no-preference){:root{}}",
			"@media (prefers-reduced-motion:no-preference){:root{}}"
		);
		assert_format!("@media(min-width:576px){}", "@media(min-width:576px){}");
	}

	#[test]
	fn test_writes_inserted_closers() {
		assert_format!("f(", "f()");
		assert_format!("{f(}", "{f()}");
		assert_format!("{a;f(}", "{a;f()}");
	}

	#[test]
	fn test_closes_unclosed_url_before_inserted_closer() {
		assert_format!("{url(x", "{url(x)}");
		assert_format!("{url(x\\", "{url(x\\\u{FFFD})}");
	}

	#[test]
	fn test_keeps_escaped_trailing_url_whitespace() {
		assert_format!("{url(x\\ )", "{url(x\\ )}");
		assert_format!("{url( x\\\t )", "{url(x\\\t)}");
		assert_format!("{url(x\\\\ )", "{url(x\\\\)}");
	}

	#[test]
	fn test_terminates_trailing_hex_escape() {
		assert_format!(r"(width \31   1px)", r"(width \31  1px)");
		assert_format!(r"\31   (a)", r"\31  (a)");
		assert_format!(r"\31   and (a)", r"\31  and (a)");
		assert_format!(r"c \31  !important", r"c \31  !important");
		assert_format!(r"\31(a)", r"\31(a)");
	}

	#[test]
	fn test_separates_comment_split_dashes() {
		assert_format!("-/**/-x", "-/**/-x");
		assert_format!("a{-/**/-x:1}", "a{-/**/-x:1}");
	}

	#[test]
	fn test_separates_comment_split_cdo() {
		assert_format!("<!/**/--", "<!/**/--");
		assert_format!("</**/!--", "</**/!--");
		assert_format!("a{b:<!/**/--}", "a{b:<!/**/--}");
		assert_format!("<! --", "<! --");
		assert_format!("< !--", "< !--");
		assert_format!("a{b:</**/!--}", "a{b:</**/!--}");
		assert_format!("a</**/b", "a<b");
		assert_format!("<!-/**/-x", "<!-/**/-x");
		assert_format!("<!-/**/-1", "<!-/**/-1");
	}

	fn raw_format(source_text: &str) -> String {
		let mut sink = String::new();
		{
			let mut stream = CursorCompactWriteSink::new(source_text, &mut sink);
			let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
			loop {
				let offset = lexer.offset();
				let token = lexer.advance();
				CursorSink::append(&mut stream, crate::Cursor::new(offset, token));
				if token == Kind::Eof {
					break;
				}
			}
		}
		sink
	}

	#[test]
	fn test_separates_comment_split_cdo_from_raw_tokens() {
		for (source_text, expected) in [("<!/**/-->", "<!/**/-->"), ("<!/**/--->", "<!/**/--->")] {
			assert_eq!(raw_format(source_text), expected);
		}
	}

	#[test]
	fn test_separates_split_double_delims() {
		assert_format!("a[b^/**/=c]", "a[b^/**/=c]");
		assert_format!("a[b^ =c]", "a[b^ =c]");
		assert_format!("a[b|/**/=c]", "a[b|/**/=c]");
		assert_format!("a[b*/**/|c]", "a[b*/**/|c]");
		assert_format!("a[b^=c]", "a[b^=c]");
		assert_format!("a>*|b", "a>*|b");
	}

	#[test]
	fn test_ends_bad_string_with_newline() {
		for (source_text, expected) in [
			("\"foo\n\"\n", "\"foo\n\"\n"),
			("\"fo\no\"", "\"fo\no\"\""),
			("\"a\n;b", "\"a\n;b"),
			("\"a\n/**/b", "\"a\nb"),
			("\"a\n/**/*b", "\"a\n*b"),
			("\\\n/**/*b", "\\\n*b"),
		] {
			assert_eq!(raw_format(source_text), expected);
		}
	}

	#[test]
	fn test_ends_lone_reverse_solidus_with_newline() {
		for (source_text, expected) in
			[("@\\\neB", "@\\\neB"), ("\\\n", "\\\n"), ("-\\\n", "-\\\n"), ("a\\\n b", "a\\\nb")]
		{
			assert_eq!(raw_format(source_text), expected);
		}
	}
}
