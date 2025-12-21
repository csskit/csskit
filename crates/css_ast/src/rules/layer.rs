use super::prelude::*;

// https://drafts.csswg.org/css-cascade-5/#layering
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.layer"))]
pub struct LayerRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Layer)]
	pub name: T![AtKeyword],
	pub prelude: LayerNameList<'a>,
	pub block: Option<LayerRuleBlock<'a>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub semicolon: Option<T![;]>,
}

impl<'a> NodeWithMetadata<CssMetadata> for LayerRule<'a> {
	fn metadata(&self) -> CssMetadata {
		let mut meta = if let Some(block) = &self.block { block.0.metadata() } else { CssMetadata::default() };
		meta.used_at_rules |= AtRuleId::Layer;
		meta.node_kinds |= NodeKinds::AtRule;
		meta
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LayerNameList<'a>(pub CommaSeparated<'a, LayerName<'a>>);

#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct LayerName<'a>(T![Ident], Vec<'a, (T![.], T![Ident])>);

impl<'a> Parse<'a> for LayerName<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
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

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LayerRuleBlock<'a>(pub RuleList<'a, Rule<'a>, CssMetadata>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LayerRule>(), 152);
		assert_eq!(std::mem::size_of::<LayerNameList>(), 32);
		assert_eq!(std::mem::size_of::<LayerName>(), 48);
		assert_eq!(std::mem::size_of::<LayerRuleBlock>(), 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LayerRule, "@layer foo{}");
		assert_parse!(CssAtomSet::ATOMS, LayerRule, "@layer foo;");
		assert_parse!(CssAtomSet::ATOMS, LayerRule, "@layer foo,bar;");
		assert_parse!(CssAtomSet::ATOMS, LayerRule, "@layer foo.bar,baz.bing.baz;");
		assert_parse!(CssAtomSet::ATOMS, LayerRule, "@layer foo.bar{body{color:black}}");
	}
}
