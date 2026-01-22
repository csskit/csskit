use super::prelude::*;

// https://drafts.csswg.org/css-transitions-2/#at-ruledef-starting-style
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(
	feature = "css_feature_data",
	derive(::csskit_derives::ToCSSFeature),
	css_feature("css.at-rules.starting-style")
)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = StartingStyle)]
pub struct StartingStyleRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::StartingStyle)]
	pub name: T![AtKeyword],
	#[metadata(delegate)]
	pub block: StartingStyleRuleBlock<'a>,
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct StartingStyleRuleBlock<'a>(#[metadata(delegate)] pub RuleList<'a, Rule<'a>, CssMetadata>);

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
