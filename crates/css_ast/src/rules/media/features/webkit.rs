use crate::units::CSSFloat;
use css_parse::{RangedFeatureKeyword, discrete_feature, keyword_set, ranged_feature};

keyword_set!(pub enum BooleanKeyword { True: "true", False: "false" });

discrete_feature!(pub enum WebkitAnimationMediaFeature<"-webkit-animation", BooleanKeyword>);

discrete_feature!(pub enum WebkitTransform2dMediaFeature<"-webkit-transform-2d", BooleanKeyword>);

discrete_feature!(pub enum WebkitTransform3dMediaFeature<"-webkit-transform-3d", BooleanKeyword>);

discrete_feature!(pub enum WebkitTransitionMediaFeature<"-webkit-transition", BooleanKeyword>);

discrete_feature!(pub enum WebkitVideoPlayableInlineMediaFeature<"-webkit-video-playable-inline", BooleanKeyword>);

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

ranged_feature!(pub enum WebkitDevicePixelRatioMediaFeature<WebkitDevicePixelRatioMediaFeatureKeyword, CSSFloat>);
