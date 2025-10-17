use super::prelude::*;
use crate::Computed;

// https://drafts.csswg.org/css-fonts/#font-face-rule
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.font-face"))]
pub struct FontFaceRule<'a> {
	#[atom(CssAtomSet::FontFace)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![AtKeyword],
	pub block: FontFaceRuleBlock<'a>,
}

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
pub struct FontFaceRuleBlock<'a>(DeclarationList<'a, FontFaceRuleStyleValue<'a>>);

#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
struct FontFaceRuleStyleValue<'a>(StyleValue<'a>);

impl<'a> DeclarationValue<'a> for FontFaceRuleStyleValue<'a> {
	type ComputedValue = Computed<'a>;

	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		matches!(
			p.to_atom::<CssAtomSet>(c),
			CssAtomSet::AscentOverride
				| CssAtomSet::DescentOverride
				| CssAtomSet::FontDisplay
				| CssAtomSet::FontFamily
				| CssAtomSet::FontFeatureSettings
				| CssAtomSet::FontLanguageOverride
				| CssAtomSet::FontNamedInstance
				| CssAtomSet::FontStyle
				| CssAtomSet::FontVariationSettings
				| CssAtomSet::FontWeight
				| CssAtomSet::FontWidth
				| CssAtomSet::LineGapOverride
				| CssAtomSet::Src
				| CssAtomSet::UnicodeRange
		)
	}

	fn is_unknown(&self) -> bool {
		self.0.is_unknown()
	}

	fn is_initial(&self) -> bool {
		self.0.is_initial()
	}

	fn is_inherit(&self) -> bool {
		self.0.is_inherit()
	}

	fn is_unset(&self) -> bool {
		self.0.is_unset()
	}

	fn is_revert(&self) -> bool {
		self.0.is_revert()
	}

	fn is_revert_layer(&self) -> bool {
		self.0.is_revert_layer()
	}

	fn needs_computing(&self) -> bool {
		self.0.needs_computing()
	}

	fn parse_declaration_value<I>(p: &mut Parser<'a, I>, name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(Self(StyleValue::parse_declaration_value(p, name)?))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FontFaceRule>(), 80);
		assert_eq!(std::mem::size_of::<FontFaceRuleStyleValue>(), 296);
		assert_eq!(std::mem::size_of::<FontFaceRuleBlock>(), 64);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(CssAtomSet::ATOMS, FontFaceRule, "@font-face {}");
	}
}
