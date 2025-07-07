use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{AtRule, CommaSeparatedPreludeList, Parse, Parser, Result as ParserResult, RuleList, T, diagnostics};
use csskit_derives::ToCursors;
use csskit_proc_macro::visit;

use crate::{Visit, Visitable, stylesheet::Rule};

// https://drafts.csswg.org/css-cascade-5/#layering
#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct LayerRule<'a> {
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

impl<'a> Visitable<'a> for LayerRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_layer_rule(self);
		if let Some(names) = &self.names {
			Visitable::accept(names, v);
		}
		Visitable::accept(&self.block, v);
	}
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LayerNameList<'a>(pub Vec<'a, (LayerName<'a>, Option<T![,]>)>);

impl<'a> CommaSeparatedPreludeList<'a> for LayerNameList<'a> {
	type PreludeItem = LayerName<'a>;
}

impl<'a> Parse<'a> for LayerNameList<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_prelude_list(p)?))
	}
}

impl<'a> Visitable<'a> for LayerNameList<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for (name, _) in &self.0 {
			Visitable::accept(name, v);
		}
	}
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
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

impl<'a> Visitable<'a> for LayerName<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_layer_name(self);
	}
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum OptionalLayerRuleBlock<'a> {
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

impl<'a> Visitable<'a> for OptionalLayerRuleBlock<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		if let Self::Block(block) = self {
			Visitable::accept(block, v);
		}
	}
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct LayerRuleBlock<'a> {
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, Rule<'a>>,
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

impl<'a> Visitable<'a> for LayerRuleBlock<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for rule in &self.rules {
			Visitable::accept(rule, v);
		}
	}
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
