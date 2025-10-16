use super::prelude::*;

// https://drafts.csswg.org/css-syntax-3/#charset-rule
#[derive(ToSpan, ToCursors, Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.charset"))]
pub struct CharsetRule {
	at_keyword: T![AtKeyword],
	space: T![' '],
	string: T![String],
	semicolon: Option<T![;]>,
}

// CharsetRule is a special rule which means it cannot use standard AtRule parsing... comments below
// https://drafts.csswg.org/css-syntax-3/#determine-the-fallback-encoding
impl<'a> Parse<'a> for CharsetRule {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let at_keyword = p.parse::<T![AtKeyword]>()?;
		let c: Cursor = at_keyword.into();
		// CharsetRule MUST be all lowercase, alt cases such as CHARSET or charSet aren't
		// valid here, compares to other at-rules which are case insensitive.
		if !p.equals_atom(c, &CssAtomSet::Charset) {
			Err(Diagnostic::new(c, Diagnostic::unexpected))?;
		}
		// Charsets MUST have a space between the at keyword and the string. This
		// isn't necessary in other at rules where an at keyword can align with other
		// delims (e.g. `(`) or unambinguous tokens like strings.
		let space = p.parse::<T![' ']>()?;
		let string = p.parse::<T![String]>()?;
		// TODO: check quote style as it should be "
		let semicolon = p.parse::<T![;]>().ok();
		Ok(Self { at_keyword, space, string, semicolon })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CharsetRule>(), 52);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CharsetRule, "@charset \"utf-8\";");
		assert_parse!(CssAtomSet::ATOMS, CharsetRule, "@charset \"UTF-8\";");
	}
}
