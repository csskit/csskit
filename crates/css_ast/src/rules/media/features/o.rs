use super::prelude::*;
use crate::units::CSSFloat;

keyword_set!(pub enum ODevicePixelRatioMediaFeatureKeyword {
	DevicePixelRatio: "-o-device-pixel-ratio",
	MaxDevicePixelRatio: "-o-max-device-pixel-ratio",
	MinDevicePixelRatio: "-o-min-device-pixel-ratio",
});

impl RangedFeatureKeyword for ODevicePixelRatioMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxDevicePixelRatio(_) | Self::MinDevicePixelRatio(_))
	}
}

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum ODevicePixelRatioMediaFeature<ODevicePixelRatioMediaFeatureKeyword, CSSFloat>
);
