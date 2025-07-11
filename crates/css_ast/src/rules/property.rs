use crate::{Visit, VisitMut, Visitable as VisitableTrait, VisitableMut};
use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{
	AtRule, Build, Declaration, DeclarationList, DeclarationValue, Parse, Parser, Peek, Result as ParserResult, T,
	diagnostics, keyword_set, syntax::ComponentValues,
};
use csskit_derives::{ToCursors, ToSpan, Visitable};

// https://drafts.csswg.org/cssom-1/#csspagerule
// https://drafts.csswg.org/css-page-3/#at-page-rule
#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct PropertyRule<'a> {
	#[visit(skip)]
	pub at_keyword: T![AtKeyword],
	#[visit(skip)]
	pub name: T![DashedIdent],
	pub block: PropertyRuleBlock<'a>,
}

// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for PropertyRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, name, block) = Self::parse_at_rule(p)?;
		if let Some(name) = name {
			Ok(Self { at_keyword, name, block })
		} else {
			let c: Cursor = at_keyword.into();
			Err(diagnostics::MissingAtRulePrelude(c.into()))?
		}
	}
}

impl<'a> AtRule<'a> for PropertyRule<'a> {
	const NAME: Option<&'static str> = Some("property");
	type Prelude = T![DashedIdent];
	type Block = PropertyRuleBlock<'a>;
}

#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PropertyRuleBlock<'a> {
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, (PropertyRuleProperty<'a>, Option<T![;]>)>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for PropertyRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, close) = Self::parse_declaration_list(p)?;
		Ok(Self { open, properties, close })
	}
}

impl<'a> DeclarationList<'a> for PropertyRuleBlock<'a> {
	type Declaration = PropertyRuleProperty<'a>;
}

impl<'a> VisitableTrait for PropertyRuleBlock<'a> {
	fn accept<V: Visit>(&self, v: &mut V) {
		for (property, _) in &self.properties {
			property.accept(v);
		}
	}
}

impl<'a> VisitableMut for PropertyRuleBlock<'a> {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for (property, _) in &mut self.properties {
			property.accept_mut(v);
		}
	}
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct PropertyRuleProperty<'a> {
	pub name: T![Ident],
	pub colon: T![:],
	pub value: PropertyRuleStyleValue<'a>,
}

impl<'a> Parse<'a> for PropertyRuleProperty<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (name, colon, value, important) = Self::parse_declaration(p)?;
		if let Some(important) = important {
			let c: Cursor = important.bang.into();
			Err(diagnostics::DisallowedImportant(c.into()))?
		}
		Ok(Self { name, colon, value })
	}
}

impl<'a> Declaration<'a> for PropertyRuleProperty<'a> {
	type DeclarationValue = PropertyRuleStyleValue<'a>;
}

#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PropertyRuleStyleValue<'a> {
	InitialValue(ComponentValues<'a>),
	Syntax(T![String]),
	Inherits(InheritsStyleValue),
	Unknown(ComponentValues<'a>),
}

keyword_set!(pub enum PropertyRulePropertyId { InitialValue: "initial-value", Inherits: "inherits", Syntax: "syntax" });

keyword_set!(pub enum InheritsStyleValue { True: "true", False: "false" });

impl<'a> DeclarationValue<'a> for PropertyRuleStyleValue<'a> {
	fn parse_declaration_value(p: &mut Parser<'a>, c: Cursor) -> ParserResult<Self> {
		if !PropertyRulePropertyId::peek(p, c) {
			Ok(Self::Unknown(p.parse::<ComponentValues<'a>>()?))
		} else {
			Ok(match PropertyRulePropertyId::build(p, c) {
				PropertyRulePropertyId::InitialValue(_) => Self::InitialValue(p.parse::<ComponentValues<'a>>()?),
				PropertyRulePropertyId::Inherits(_) => Self::Inherits(p.parse::<InheritsStyleValue>()?),
				PropertyRulePropertyId::Syntax(_) => Self::Syntax(p.parse::<T![String]>()?),
			})
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PropertyRule>(), 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PropertyRule, r#"@property --foo{initial-value:0;inherits:false;syntax:"<length>"}"#);
	}
}
