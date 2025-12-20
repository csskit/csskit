use super::prelude::*;
use crate::units::Length;

ranged_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
	pub enum DeviceHeightMediaFeature{CssAtomSet::DeviceHeight | CssAtomSet::MinDeviceHeight | CssAtomSet::MaxDeviceHeight, Length}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DeviceHeightMediaFeature>(), 124);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height:360px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height:35rem)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(min-device-height:35rem)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(max-device-height:35rem)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height<=800px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(1400px=device-height)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(100px<=device-height)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(100px<device-height<1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(100px>device-height<1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(100px>=device-height<=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(100px<=device-height>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height:)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height: > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(max-device-height > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(min-device-height > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(pointer: 1px)");
	}
}
