use super::prelude::*;
use crate::types::Ratio;

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::DeviceAspectRatio)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
	#[derive(csskit_derives::NodeWithMetadata)]
	pub enum DeviceAspectRatioMediaFeature<'a>{CssAtomSet::DeviceAspectRatio | CssAtomSet::MinDeviceAspectRatio | CssAtomSet::MaxDeviceAspectRatio, Ratio<'a>}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(device-aspect-ratio:1/1)");
		assert_parse!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(device-aspect-ratio:16/9)");
		assert_parse!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(min-device-aspect-ratio:1/1)");
		assert_parse!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(max-device-aspect-ratio:16/9)");
		assert_parse!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(device-aspect-ratio<=1/1)");
		assert_parse!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(device-aspect-ratio>=16/9)");
		assert_parse!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(device-aspect-ratio=1/1)");
		assert_parse!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(16/9=device-aspect-ratio)");
		assert_parse!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(1/1<=device-aspect-ratio)");
		assert_parse!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(1/1<device-aspect-ratio<16/9)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(device-aspect-ratio:)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(device-aspect-ratio: > 1/1)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(max-device-aspect-ratio > 1/1)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(min-device-aspect-ratio > 1/1)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(device-aspect-ratio: 1px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceAspectRatioMediaFeature, "(aspect-ratio: 1/1)");
	}
}
