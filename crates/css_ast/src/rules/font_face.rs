use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{
	AtRule, Declaration, NoPreludeAllowed, Parse, Parser, Peek, Result as ParserResult, RuleList, T, keyword_set,
	syntax::BangImportant,
};
use csskit_derives::{ToCursors, ToSpan, Visitable};

use crate::StyleValue;

// https://drafts.csswg.org/css-fonts/#font-face-rule
#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct FontFaceRule<'a> {
	#[visit(skip)]
	pub at_keyword: T![AtKeyword],
	pub block: FontFaceRuleBlock<'a>,
}

impl<'a> AtRule<'a> for FontFaceRule<'a> {
	const NAME: Option<&'static str> = Some("font-face");
	type Prelude = NoPreludeAllowed;
	type Block = FontFaceRuleBlock<'a>;
}

impl<'a> Parse<'a> for FontFaceRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, _, block) = Self::parse_at_rule(p)?;
		Ok(Self { at_keyword, block })
	}
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub struct FontFaceRuleBlock<'a> {
	#[visit(skip)]
	pub open: T!['{'],
	pub properties: Vec<'a, FontFaceRuleProperty<'a>>,
	#[visit(skip)]
	pub close: Option<T!['}']>,
}

impl<'a> RuleList<'a> for FontFaceRuleBlock<'a> {
	type Rule = FontFaceRuleProperty<'a>;
}

impl<'a> Parse<'a> for FontFaceRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, close) = Self::parse_rule_list(p)?;
		Ok(Self { open, properties, close })
	}
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "property"))]
#[visit]
pub struct FontFaceRuleProperty<'a> {
	#[visit(skip)]
	pub name: T![Ident],
	#[visit(skip)]
	pub colon: T![:],
	pub value: StyleValue<'a>,
	#[visit(skip)]
	pub important: Option<BangImportant>,
}

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

impl<'a> Declaration<'a> for FontFaceRuleProperty<'a> {
	type DeclarationValue = StyleValue<'a>;
	fn valid_property(p: &Parser, c: Cursor) -> bool {
		FontFaceRulePropertyId::peek(p, c)
	}
}

impl<'a> Parse<'a> for FontFaceRuleProperty<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (name, colon, value, important) = Self::parse_declaration(p)?;
		Ok(Self { name, colon, value, important })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FontFaceRule>(), 80);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(FontFaceRule, "@font-face {}");
	}
}
