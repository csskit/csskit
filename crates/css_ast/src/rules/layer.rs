use bumpalo::collections::Vec;
use css_parse::{AtRule, Parse, Parser, Result as ParserResult, RuleList, T, atkeyword_set, syntax::CommaSeparated};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

use crate::stylesheet::Rule;

atkeyword_set!(struct AtLayerKeyword "layer");

// https://drafts.csswg.org/css-cascade-5/#layering
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct LayerRule<'a>(AtRule<AtLayerKeyword, LayerNameList<'a>, Option<LayerRuleBlock<'a>>>);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LayerRuleBlock<'a>(RuleList<'a, Rule<'a>>);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LayerRule>(), 128);
		assert_eq!(std::mem::size_of::<LayerNameList>(), 32);
		assert_eq!(std::mem::size_of::<LayerName>(), 48);
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
