use super::prelude::*;
use crate::units::CSSFloat;

keyword_set!(pub enum BooleanKeyword { True: "true", False: "false" });

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum WebkitAnimationMediaFeature<"-webkit-animation", BooleanKeyword>
);

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum WebkitTransform2dMediaFeature<"-webkit-transform-2d", BooleanKeyword>
);

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum WebkitTransform3dMediaFeature<"-webkit-transform-3d", BooleanKeyword>
);

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum WebkitTransitionMediaFeature<"-webkit-transition", BooleanKeyword>
);

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum WebkitVideoPlayableInlineMediaFeature<"-webkit-video-playable-inline", BooleanKeyword>
);

keyword_set!(pub enum WebkitDevicePixelRatioMediaFeatureKeyword {
	DevicePixelRatio: "-webkit-device-pixel-ratio",
	MaxDevicePixelRatio: "-webkit-max-device-pixel-ratio",
	MinDevicePixelRatio: "-webkit-min-device-pixel-ratio",
});

impl RangedFeatureKeyword for WebkitDevicePixelRatioMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxDevicePixelRatio(_) | Self::MinDevicePixelRatio(_))
	}
}

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum WebkitDevicePixelRatioMediaFeature<WebkitDevicePixelRatioMediaFeatureKeyword, CSSFloat>
);
