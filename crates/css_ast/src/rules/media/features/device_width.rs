use super::prelude::*;
use crate::units::Length;

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum DeviceWidthMediaFeature<CssAtomSet::DeviceWidth | CssAtomSet::MinDeviceWidth | CssAtomSet::MaxDeviceWidth, Length>
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DeviceWidthMediaFeature>(), 124);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width:360px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width:35rem)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(min-device-width:35rem)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(max-device-width:35rem)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width<=800px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(1400px=device-width)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(100px<=device-width)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(100px<device-width<1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(100px>device-width<1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(100px>=device-width<=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(100px<=device-width>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width:)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width: > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(max-device-width > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(min-device-width > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(pointer: 1px)");
	}
}
