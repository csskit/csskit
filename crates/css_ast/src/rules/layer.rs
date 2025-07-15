use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{AtRule, Parse, Parser, Result as ParserResult, RuleList, T, diagnostics, syntax::CommaSeparated};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

use crate::stylesheet::Rule;

// https://drafts.csswg.org/css-cascade-5/#layering
#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct LayerRule<'a> {
	#[visit(skip)]
	pub at_keyword: T![AtKeyword],
	pub names: Option<LayerNameList<'a>>,
	pub block: OptionalLayerRuleBlock<'a>,
}

// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for LayerRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, names, block) = Self::parse_at_rule(p)?;
		if let Some(ref names) = names {
			if matches!(block, OptionalLayerRuleBlock::Block(_)) && names.0.len() > 1 {
				let c: Cursor = names.0[0].0.0.into();
				Err(diagnostics::DisallowedLayerBlockWithMultipleNames(c.into()))?
			}
		}
		Ok(Self { at_keyword, names, block })
	}
}

impl<'a> AtRule<'a> for LayerRule<'a> {
	const NAME: Option<&'static str> = Some("layer");
	type Prelude = LayerNameList<'a>;
	type Block = OptionalLayerRuleBlock<'a>;
}

#[derive(Peek, Parse, ToCursors, Visitable, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LayerNameList<'a>(pub CommaSeparated<'a, LayerName<'a>>);

#[derive(Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LayerName<'a>(T![Ident], Vec<'a, (T![.], T![Ident])>);

impl<'a> Parse<'a> for LayerName<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut parts = Vec::new_in(p.bump());
		let first = p.parse::<T![Ident]>()?;
		loop {
			if p.peek::<T![.]>() {
				let dot = p.parse::<T![.]>()?;
				let ident = p.parse::<T![Ident]>()?;
				parts.push((dot, ident));
			} else {
				return Ok(Self(first, parts));
			}
		}
	}
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum OptionalLayerRuleBlock<'a> {
	#[visit(skip)]
	None(T![;]),
	Block(LayerRuleBlock<'a>),
}

impl<'a> Parse<'a> for OptionalLayerRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(semicolon) = p.parse_if_peek::<T![;]>()? {
			Ok(Self::None(semicolon))
		} else {
			Ok(Self::Block(p.parse::<LayerRuleBlock>()?))
		}
	}
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct LayerRuleBlock<'a> {
	#[visit(skip)]
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, Rule<'a>>,
	#[visit(skip)]
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for LayerRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, rules, close) = Self::parse_rule_list(p)?;
		Ok(Self { open, rules, close })
	}
}

impl<'a> RuleList<'a> for LayerRuleBlock<'a> {
	type Rule = Rule<'a>;
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LayerRule>(), 112);
		assert_eq!(std::mem::size_of::<LayerNameList>(), 32);
		assert_eq!(std::mem::size_of::<LayerName>(), 48);
		assert_eq!(std::mem::size_of::<OptionalLayerRuleBlock>(), 64);
		assert_eq!(std::mem::size_of::<LayerRuleBlock>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(LayerRule, "@layer foo{}");
		assert_parse!(LayerRule, "@layer foo;");
		assert_parse!(LayerRule, "@layer foo,bar;");
		assert_parse!(LayerRule, "@layer foo.bar,baz.bing.baz;");
		assert_parse!(LayerRule, "@layer foo.bar{body{color:black}}");
	}
}
