use super::prelude::*;
use crate::EncodingLabel;
use css_lexer::{QuoteStyle, Whitespace};

/// <https://drafts.csswg.org/css-syntax-3/#charset-rule>
#[node]
#[derive(Peek, ToSpan, ToCursors, SemanticEq, Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.charset"))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = Charset)]
pub struct CharsetRule {
	#[atom(CssAtomSet::Charset)]
	pub at_keyword: T![AtKeyword],
	pub space: T![' '],
	pub string: T![String],
	#[semantic_eq(skip)]
	pub semicolon: Option<T![;]>,
}

impl CharsetRule {
	/// Checks this rule is the literal byte sequence that [determining the fallback encoding] looks
	/// for: a lowercase `@charset`, one space, a double quoted label, and a semicolon.
	///
	/// Any other spelling reaches no browser, because the sequence is matched byte by byte before
	/// the stylesheet is parsed. Such a rule has no effect.
	///
	/// [determining the fallback encoding]: https://drafts.csswg.org/css-syntax-3/#determine-the-fallback-encoding
	pub fn is_byte_sequence(&self) -> bool {
		let at_keyword: Cursor = self.at_keyword.into();
		let space: Cursor = self.space.into();
		let string: Cursor = self.string.into();
		at_keyword.token().is_lower_case()
			&& !at_keyword.token().contains_escape_chars()
			&& space.token().len() == 1
			&& space.token() == Whitespace::Space
			&& string == QuoteStyle::Double
			&& !string.token().contains_escape_chars()
			&& self.semicolon.is_some()
	}

	/// Gives the label between the quotes, as written.
	///
	/// ```rust
	/// # use css_ast::{CharsetRule, CssAtomSet};
	/// # use css_parse::{Arena, Parser};
	/// # use css_lexer::{AtomSet, Lexer};
	/// let source_text = "@charset \"ISO-8859-1\";";
	/// let alloc = Arena::default();
	/// let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
	/// let rule = Parser::new(&alloc, source_text, lexer).parse_entirely::<CharsetRule>().output.unwrap();
	/// assert_eq!(rule.label(source_text), "ISO-8859-1");
	/// ```
	pub fn label<'a>(&self, source_text: &'a str) -> &'a str {
		let c: Cursor = self.string.into();
		let source = c.str_slice(source_text);
		&source[c.token().leading_len() as usize..source.len() - c.token().trailing_len() as usize]
	}

	/// Gives the encoding this rule sets, as the shortest label naming it.
	///
	/// Gives [EncodingLabel::Unknown] when the rule has no effect: it is not the byte sequence (see
	/// [CharsetRule::is_byte_sequence]), its label is not in the [Encoding Standard], or its label
	/// names an encoding that resolves to UTF-8, which is the default.
	///
	/// [Encoding Standard]: https://encoding.spec.whatwg.org/#names-and-labels
	///
	/// ```rust
	/// # use css_ast::{AtomSet, CharsetRule, CssAtomSet, EncodingLabel};
	/// # use css_parse::{Arena, Parser};
	/// # use css_lexer::Lexer;
	/// # fn encoding(source_text: &str) -> EncodingLabel {
	/// # let alloc = Arena::default();
	/// # let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
	/// # let rule = Parser::new(&alloc, source_text, lexer).parse_entirely::<CharsetRule>().output.unwrap();
	/// # rule.encoding(source_text)
	/// # }
	/// assert_eq!(encoding("@charset \"ISO-8859-1\";").to_str(), "l1");
	/// assert_eq!(encoding("@charset \"UTF-8\";"), EncodingLabel::Unknown);
	/// assert_eq!(encoding("@charset 'gbk';"), EncodingLabel::Unknown);
	/// ```
	pub fn encoding(&self, source_text: &str) -> EncodingLabel {
		if !self.is_byte_sequence() {
			return EncodingLabel::Unknown;
		}
		EncodingLabel::from_label(self.label(source_text)).compact()
	}
}

// CharsetRule is a special rule which means it cannot use standard AtRule parsing... comments below
// https://drafts.csswg.org/css-syntax-3/#determine-the-fallback-encoding
impl<'a> Parse<'a> for CharsetRule {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let at_keyword = p.parse::<T![AtKeyword]>()?;
		let c: Cursor = at_keyword.into();
		if !p.equals_atom(c, &CssAtomSet::Charset) {
			Err(Diagnostic::new(c, Diagnostic::unexpected))?;
		}
		// Charsets MUST have a space between the at keyword and the string. This
		// isn't necessary in other at rules where an at keyword can align with other
		// delims (e.g. `(`) or unambinguous tokens like strings.
		let space = p.parse::<T![' ']>()?;
		let string = p.parse::<T![String]>()?;
		let semicolon = p.parse::<T![;]>().ok();
		Ok(Self { at_keyword, space, string, semicolon })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{Parser, assert_parse};

	fn is_byte_sequence(source_text: &str) -> bool {
		let alloc = css_parse::Arena::default();
		let lexer = css_lexer::Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&alloc, source_text, lexer);
		parser.parse_entirely::<CharsetRule>().output.expect("did not parse").is_byte_sequence()
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CharsetRule, "@charset \"utf-8\";");
		assert_parse!(CssAtomSet::ATOMS, CharsetRule, "@charset \"UTF-8\";");
	}

	#[test]
	fn test_is_byte_sequence() {
		assert!(is_byte_sequence("@charset \"utf-8\";"));
		assert!(is_byte_sequence("@charset \"UTF-8\";"));
	}

	#[test]
	fn test_is_not_byte_sequence() {
		assert!(!is_byte_sequence("@CHARSET \"utf-8\";"));
		assert!(!is_byte_sequence("@charset 'utf-8';"));
		assert!(!is_byte_sequence("@charset \"\\75 tf-8\";"));
		assert!(!is_byte_sequence("@charset\t\"utf-8\";"));
		assert!(!is_byte_sequence("@charset  \"utf-8\";"));
		assert!(!is_byte_sequence("@charset \"utf-8\""));
	}
}
