use super::prelude::*;

boolean_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::Grid)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	#[derive(csskit_derives::NodeWithMetadata)]
	pub enum GridMediaFeature{CssAtomSet::Grid}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, GridMediaFeature, "(grid:1)");
		assert_parse!(CssAtomSet::ATOMS, GridMediaFeature, "(grid)");
	}
}
