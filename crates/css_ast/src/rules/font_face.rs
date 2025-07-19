use crate::StyleValue;
use css_lexer::Cursor;
use css_parse::{
	AtRule, DeclarationList, DeclarationValue, NoPreludeAllowed, Parser, Peek, Result as ParserResult, atkeyword_set,
	keyword_set,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

atkeyword_set!(struct AtFontFaceKeyword "font-face");

// https://drafts.csswg.org/css-fonts/#font-face-rule
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct FontFaceRule<'a>(AtRule<'a, AtFontFaceKeyword, NoPreludeAllowed, FontFaceRuleBlock<'a>>);

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
	fn valid_declaration_name(p: &Parser, c: Cursor) -> bool {
		FontFaceRulePropertyId::peek(p, c)
	}

	fn parse_declaration_value(p: &mut Parser<'a>, name: Cursor) -> ParserResult<Self> {
		Ok(Self(StyleValue::parse_declaration_value(p, name)?))
	}

	fn is_unknown(&self) -> bool {
		self.0.is_unknown()
	}

	fn needs_computing(&self) -> bool {
		self.0.needs_computing()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FontFaceRule>(), 96);
		assert_eq!(std::mem::size_of::<FontFaceRuleStyleValue>(), 328);
		assert_eq!(std::mem::size_of::<FontFaceRuleBlock>(), 64);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(FontFaceRule, "@font-face {}");
	}
}
