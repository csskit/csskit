use super::prelude::*;
use crate::types::Ratio;

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::AspectRatio)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
	#[derive(csskit_derives::NodeWithMetadata)]
	pub enum AspectRatioMediaFeature<'a>{CssAtomSet::AspectRatio | CssAtomSet::MinAspectRatio | CssAtomSet::MaxAspectRatio, Ratio<'a>}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(aspect-ratio:1/1)");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(aspect-ratio:16/9)");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(aspect-ratio:1)");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(min-aspect-ratio:1/1)");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(max-aspect-ratio:16/9)");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(aspect-ratio<=1/1)");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(aspect-ratio>=16/9)");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(aspect-ratio=1/1)");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(16/9=aspect-ratio)");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(1/1<=aspect-ratio)");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(1/1<aspect-ratio<16/9)");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(1/1>=aspect-ratio<=16/9)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(aspect-ratio:)");
		assert_parse_error!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(aspect-ratio: > 1/1)");
		assert_parse_error!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(max-aspect-ratio > 1/1)");
		assert_parse_error!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(min-aspect-ratio > 1/1)");
		assert_parse_error!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(aspect-ratio: 1px)");
		assert_parse_error!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(aspect-ratio: portrait)");
		assert_parse_error!(CssAtomSet::ATOMS, AspectRatioMediaFeature, "(pointer: 1/1)");
	}
}
