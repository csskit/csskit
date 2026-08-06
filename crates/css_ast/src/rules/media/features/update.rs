use super::prelude::*;

discrete_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::Update)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	#[derive(csskit_derives::NodeWithMetadata)]
	pub enum UpdateMediaFeature{CssAtomSet::Update, UpdateMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
#[node]
pub enum UpdateMediaFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Slow)]
	Slow(T![Ident]),
	#[atom(CssAtomSet::Fast)]
	Fast(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, UpdateMediaFeature, "(update)");
		assert_parse!(CssAtomSet::ATOMS, UpdateMediaFeature, "(update:none)");
		assert_parse!(CssAtomSet::ATOMS, UpdateMediaFeature, "(update:slow)");
		assert_parse!(CssAtomSet::ATOMS, UpdateMediaFeature, "(update:fast)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, UpdateMediaFeature, "(update:)");
		assert_parse_error!(CssAtomSet::ATOMS, UpdateMediaFeature, "(update: quick)");
		assert_parse_error!(CssAtomSet::ATOMS, UpdateMediaFeature, "(pointer: fast)");
	}
}
