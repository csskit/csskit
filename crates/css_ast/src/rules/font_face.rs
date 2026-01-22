use css_parse::DeclarationValue;

use super::prelude::*;
use crate::{Computed, CssMetadata};

// https://drafts.csswg.org/css-fonts/#font-face-rule
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.font-face"))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = FontFace)]
pub struct FontFaceRule<'a> {
	#[atom(CssAtomSet::FontFace)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![AtKeyword],
	#[metadata(delegate)]
	pub block: FontFaceRuleBlock<'a>,
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceRuleBlock<'a>(#[metadata(delegate)] DeclarationList<'a, FontFaceRuleStyleValue<'a>, CssMetadata>);

#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontFaceRuleStyleValue<'a>(#[metadata(delegate)] pub StyleValue<'a>);

impl<'a> DeclarationValue<'a, CssMetadata> for FontFaceRuleStyleValue<'a> {
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
		Ok(Self(<StyleValue as DeclarationValue<CssMetadata>>::parse_declaration_value(p, name)?))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FontFaceRule>(), 112);
		assert_eq!(std::mem::size_of::<FontFaceRuleStyleValue>(), 416);
		assert_eq!(std::mem::size_of::<FontFaceRuleBlock>(), 96);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(CssAtomSet::ATOMS, FontFaceRule, "@font-face {}");
	}
}
