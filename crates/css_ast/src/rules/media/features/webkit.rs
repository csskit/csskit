use crate::units::CSSFloat;
use css_parse::{RangedFeatureKeyword, discrete_feature, keyword_set, ranged_feature};

keyword_set!(pub enum BooleanKeyword { True: "true", False: "false" });

discrete_feature!(WebkitAnimationMediaFeature, "-webkit-animation", BooleanKeyword);

discrete_feature!(WebkitTransform2dMediaFeature, "-webkit-transform-2d", BooleanKeyword);

discrete_feature!(WebkitTransform3dMediaFeature, "-webkit-transform-3d", BooleanKeyword);

discrete_feature!(WebkitTransitionMediaFeature, "-webkit-transition", BooleanKeyword);

discrete_feature!(WebkitVideoPlayableInlineMediaFeature, "-webkit-video-playable-inline", BooleanKeyword);

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

ranged_feature!(WebkitDevicePixelRatioMediaFeature, WebkitDevicePixelRatioMediaFeatureKeyword, CSSFloat);
