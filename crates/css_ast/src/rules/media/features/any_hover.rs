use super::prelude::*;

discrete_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::AnyHover)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum AnyHoverMediaFeature{CssAtomSet::AnyHover, AnyHoverMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
#[node]
pub enum AnyHoverMediaFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Hover)]
	Hover(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{ConditionalFeature, CssAtomSet, FeatureMetadata};
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AnyHoverMediaFeature, "(any-hover)");
		assert_parse!(CssAtomSet::ATOMS, AnyHoverMediaFeature, "(any-hover:hover)");
		assert_parse!(CssAtomSet::ATOMS, AnyHoverMediaFeature, "(any-hover:none)");
	}

	#[test]
	fn test_feature_metadata_bare() {
		assert_parse!(CssAtomSet::ATOMS, AnyHoverMediaFeature, "(any-hover)", |node| {
			assert!(matches!(
				node.feature_metadata(),
				ConditionalFeature::Plain { name: CssAtomSet::AnyHover, value: None }
			));
		});
	}

	#[test]
	fn test_feature_metadata_with_value() {
		assert_parse!(CssAtomSet::ATOMS, AnyHoverMediaFeature, "(any-hover:hover)", |node| {
			assert!(matches!(
				node.feature_metadata(),
				ConditionalFeature::Plain { name: CssAtomSet::AnyHover, value: Some(_) }
			));
		});
	}
}
