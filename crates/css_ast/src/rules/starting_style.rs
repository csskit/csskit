use super::prelude::*;

// https://drafts.csswg.org/css-transitions-2/#at-ruledef-starting-style
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, metadata(skip))]
#[cfg_attr(
	feature = "css_feature_data",
	derive(::csskit_derives::ToCSSFeature),
	css_feature("css.at-rules.starting-style")
)]
pub struct StartingStyleRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::StartingStyle)]
	pub name: T![AtKeyword],
	pub block: StartingStyleRuleBlock<'a>,
}

impl<'a> NodeWithMetadata<CssMetadata> for StartingStyleRule<'a> {
	fn self_metadata(&self) -> CssMetadata {
		CssMetadata { used_at_rules: AtRuleId::StartingStyle, node_kinds: NodeKinds::AtRule, ..Default::default() }
	}

	fn metadata(&self) -> CssMetadata {
		self.block.0.metadata().merge(self.self_metadata())
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct StartingStyleRuleBlock<'a>(pub RuleList<'a, Rule<'a>, CssMetadata>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<StartingStyleRule>(), 112);
		assert_eq!(std::mem::size_of::<StartingStyleRuleBlock>(), 96);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, StartingStyleRule, "@starting-style{}");
		assert_parse!(CssAtomSet::ATOMS, StartingStyleRule, "@starting-style{body{color:black}}");
		assert_parse!(CssAtomSet::ATOMS, StartingStyleRule, "@starting-style{h1{background-color:transparent}}");
	}
}
