use super::prelude::*;
use crate::Computed;

atkeyword_set!(pub struct AtFontFaceKeyword "font-face");

// https://drafts.csswg.org/css-fonts/#font-face-rule
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.font-face"))]
#[visit]
pub struct FontFaceRule<'a>(pub AtRule<AtFontFaceKeyword, NoPreludeAllowed, FontFaceRuleBlock<'a>>);

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub struct FontFaceRuleBlock<'a>(DeclarationList<'a, FontFaceRuleStyleValue<'a>>);

keyword_set!(pub enum FontFaceRulePropertyId {
	AscentOverride: "ascent-override",
	DescentOverride: "descent-override",
	FontDisplay: "font-display",
	FontFamily: "font-family",
	FontFeatureSettings: "font-feature-settings",
	FontLanguageOverride: "font-language-override",
	FontNamedInstance: "font-named-instance",
	FontStyle: "font-style",
	FontVariationSettings: "font-variation-settings",
	FontWeight: "font-weight",
	FontWidth: "font-width",
	LineGapOverride: "line-gap-override",
	Src: "src",
	UnicodeRange: "unicode-range",
});

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
struct FontFaceRuleStyleValue<'a>(StyleValue<'a>);

impl<'a> DeclarationValue<'a> for FontFaceRuleStyleValue<'a> {
	type ComputedValue = Computed<'a>;

	fn valid_declaration_name(p: &Parser, c: Cursor) -> bool {
		FontFaceRulePropertyId::peek(p, c)
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

	fn parse_declaration_value(p: &mut Parser<'a>, name: Cursor) -> ParserResult<Self> {
		Ok(Self(StyleValue::parse_declaration_value(p, name)?))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FontFaceRule>(), 96);
		assert_eq!(std::mem::size_of::<FontFaceRuleStyleValue>(), 296);
		assert_eq!(std::mem::size_of::<FontFaceRuleBlock>(), 64);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(FontFaceRule, "@font-face {}");
	}
}
