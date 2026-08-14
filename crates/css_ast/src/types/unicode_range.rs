use super::prelude::*;
use css_lexer::{Feature as LexerFeature, Lexer, SourceOffset};

/// `<unicode-range-token>` as defined in [css-syntax-3](https://drafts.csswg.org/css-syntax-3/#typedef-unicode-range-token).
///
/// ```text,ignore
/// U+0-7F | U+30?? | U+4E00-9FFF
/// ```
///
/// The token only exists in the `unicode-range` descriptor, where the value is re-tokenized with
/// unicode ranges allowed. Ordinary tokenization would split `U+4E00-9FFF` into an ident, a number
/// and a dimension, so parsing re-lexes the source from the current position and consumes every
/// token the re-lexed range covers, keeping a single [`Cursor`] over the whole range.
#[node]
#[derive(IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct UnicodeRange(Cursor);

impl UnicodeRange {
	/// The first code point of the range.
	pub fn start(&self) -> u32 {
		self.0.token().unicode_range_start()
	}

	/// The last code point of the range.
	pub fn end(&self) -> u32 {
		self.0.token().unicode_range_end()
	}
}

impl<'a> Peek<'a> for UnicodeRange {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident]);

	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if c != Kind::Ident {
			return false;
		}
		let source = p.source_text();
		if !c.str_slice(source).eq_ignore_ascii_case("u") {
			return false;
		}
		source[c.end_offset().0 as usize..].starts_with('+')
	}
}

impl<'a> Parse<'a> for UnicodeRange {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		if !p.peek::<Self>() {
			Err(Diagnostic::new(c, Diagnostic::invalid_unicode_range))?
		}
		let offset = c.offset().0 as usize;
		let token =
			Lexer::new_with_features(&CssAtomSet::ATOMS, &p.source_text()[offset..], LexerFeature::UnicodeRange)
				.advance();
		if token != Kind::UnicodeRange || token.is_bad() {
			Err(Diagnostic::new(c, Diagnostic::invalid_unicode_range))?
		}
		let end = offset + token.len() as usize;
		let skip = p.set_skip(KindSet::NONE);
		let result = loop {
			let consumed = p.next();
			match consumed.end_offset().0 as usize {
				at if at == end => break Ok(()),
				at if at > end || consumed == Kind::Eof => {
					break Err(Diagnostic::new(c, Diagnostic::invalid_unicode_range));
				}
				_ => {}
			}
		};
		p.set_skip(skip);
		result?;
		Ok(Self(Cursor::new(SourceOffset(offset as u32), token)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, UnicodeRange, "U+26");
		assert_parse!(CssAtomSet::ATOMS, UnicodeRange, "U+0-7F");
		assert_parse!(CssAtomSet::ATOMS, UnicodeRange, "u+590-5ff");
		assert_parse!(CssAtomSet::ATOMS, UnicodeRange, "U+30??");
		assert_parse!(CssAtomSet::ATOMS, UnicodeRange, "U+4E00-9FFF");
		assert_parse!(CssAtomSet::ATOMS, UnicodeRange, "U+0-10FFFF");
		assert_parse!(CssAtomSet::ATOMS, UnicodeRange, "U+??????");
	}

	#[test]
	fn test_values() {
		let alloc = css_parse::Arena::new();
		let source = "U+4E00-9FFF";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source);
		let mut p = Parser::new(&alloc, source, lexer);
		let range = p.parse_entirely::<UnicodeRange>().output.expect("parses");
		assert_eq!(range.start(), 0x4E00);
		assert_eq!(range.end(), 0x9FFF);
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, UnicodeRange, "U");
		assert_peek_false!(CssAtomSet::ATOMS, UnicodeRange, "url(a)");
		assert_peek_false!(CssAtomSet::ATOMS, UnicodeRange, "1px");
		assert_parse_error!(CssAtomSet::ATOMS, UnicodeRange, "U+ZZ");
	}
}
