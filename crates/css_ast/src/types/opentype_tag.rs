use super::prelude::*;

/// `<opentype-tag>` as defined in [css-fonts-4](https://drafts.csswg.org/css-fonts-4/#font-feature-settings-prop).
///
/// An OpenType feature or variation axis tag. Tags are case-sensitive and must be
/// exactly 4 ASCII characters (U+20-7E).
///
/// Wraps `T![String]` for parsing purposes.
#[derive(IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OpentypeTag(pub T![String]);

impl<'a> Peek<'a> for OpentypeTag {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![String]>::peek(p, c)
	}
}

impl<'a> Parse<'a> for OpentypeTag {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let string = p.parse::<T![String]>()?;
		let sc = p.to_source_cursor(string.into());
		let source = sc.source();
		let token = sc.token();
		let has_close = token.has_close_quote();
		let inner = &source[1..(token.len() as usize) - has_close as usize];

		if inner.len() != 4 {
			return Err(Diagnostic::new(string.into(), Diagnostic::opentype_tag_length));
		}

		for byte in inner.bytes() {
			if !(0x20..=0x7E).contains(&byte) {
				return Err(Diagnostic::new(string.into(), Diagnostic::opentype_tag_ascii));
			}
		}

		Ok(Self(string))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OpentypeTag>(), 12);
	}

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, OpentypeTag, "\"kern\"");
		assert_parse!(CssAtomSet::ATOMS, OpentypeTag, "\"liga\"");
		assert_parse!(CssAtomSet::ATOMS, OpentypeTag, "\"smcp\"");
		assert_parse!(CssAtomSet::ATOMS, OpentypeTag, "\"wght\"");
		assert_parse!(CssAtomSet::ATOMS, OpentypeTag, "\"cv01\"");
		assert_parse!(CssAtomSet::ATOMS, OpentypeTag, "\"ss01\"");
		assert_parse!(CssAtomSet::ATOMS, OpentypeTag, "'kern'");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, OpentypeTag, "\"ker\"");
		assert_parse_error!(CssAtomSet::ATOMS, OpentypeTag, "\"kerns\"");
		assert_parse_error!(CssAtomSet::ATOMS, OpentypeTag, "\"\"");
		assert_parse_error!(CssAtomSet::ATOMS, OpentypeTag, "kern");
	}
}
