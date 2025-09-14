use css_parse::{
	AtRule, Cursor, DeclarationList, DeclarationValue, Parser, Peek, Result as ParserResult, T, atkeyword_set,
	keyword_set, syntax::ComponentValues,
};
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, ToSpan, Visitable};

use crate::Computed;

atkeyword_set!(pub struct AtPropertyKeyword "property");

// https://drafts.csswg.org/cssom-1/#csspagerule
// https://drafts.csswg.org/css-page-3/#at-page-rule
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.property"))]
#[visit]
pub struct PropertyRule<'a>(pub AtRule<AtPropertyKeyword, PropertyPrelude, PropertyRuleBlock<'a>>);

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct PropertyPrelude(T![DashedIdent]);

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PropertyRuleBlock<'a>(DeclarationList<'a, PropertyRuleValue<'a>>);

keyword_set!(pub enum PropertyRulePropertyId { InitialValue: "initial-value", Inherits: "inherits", Syntax: "syntax" });

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub enum PropertyRuleValue<'a> {
	InitialValue(ComponentValues<'a>),
	Syntax(SyntaxValue),
	Inherits(InheritsValue),
	Unknown(ComponentValues<'a>),
}

keyword_set!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum InheritsValue {
		True: "true",
		False: "false"
	}
);

#[derive(Parse, Peek, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct SyntaxValue(T![String]);

impl<'a> DeclarationValue<'a> for PropertyRuleValue<'a> {
	type ComputedValue = Computed<'a>;

	fn valid_declaration_name(p: &Parser<'a>, c: Cursor) -> bool {
		PropertyRulePropertyId::peek(p, c)
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn is_initial(&self) -> bool {
		false
	}

	fn is_inherit(&self) -> bool {
		false
	}

	fn is_unset(&self) -> bool {
		false
	}

	fn is_revert(&self) -> bool {
		false
	}

	fn is_revert_layer(&self) -> bool {
		false
	}

	fn needs_computing(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn parse_declaration_value(p: &mut Parser<'a>, c: Cursor) -> ParserResult<Self> {
		Ok(match PropertyRulePropertyId::from_cursor(p, c) {
			Some(PropertyRulePropertyId::InitialValue(_)) => Self::InitialValue(p.parse::<ComponentValues<'a>>()?),
			Some(PropertyRulePropertyId::Inherits(_)) => Self::Inherits(p.parse::<InheritsValue>()?),
			Some(PropertyRulePropertyId::Syntax(_)) => Self::Syntax(p.parse::<SyntaxValue>()?),
			None => Self::Unknown(p.parse::<ComponentValues<'a>>()?),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PropertyRule>(), 104);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PropertyRule, r#"@property --foo{initial-value:0;inherits:false;syntax:"<length>"}"#);
	}
}
