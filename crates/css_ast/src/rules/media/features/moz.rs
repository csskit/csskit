use crate::units::CSSFloat;
use css_parse::{RangedFeatureKeyword, boolean_feature, discrete_feature, keyword_set, ranged_feature};

// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#media_features

keyword_set!(pub enum MozDevicePixelRatioMediaFeatureKeyword {
	DevicePixelRatio: "-moz-device-pixel-ratio",
	MaxDevicePixelRatio: "-moz-max-device-pixel-ratio",
	MinDevicePixelRatio: "-moz-min-device-pixel-ratio",
});

impl RangedFeatureKeyword for MozDevicePixelRatioMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxDevicePixelRatio(_) | Self::MinDevicePixelRatio(_))
	}
}

ranged_feature!(pub enum MozDevicePixelRatioMediaFeature<MozDevicePixelRatioMediaFeatureKeyword, CSSFloat>);

keyword_set!(pub enum MozDeviceOrientationMediaFeatureKeyword { Portrait: "portrait", Landscape: "landscape" });

discrete_feature!(pub enum MozDeviceOrientationMediaFeature<"-moz-device-orientation", MozDeviceOrientationMediaFeatureKeyword>);

boolean_feature!(pub enum MozMacGraphiteThemeMediaFeature<"-moz-mac-graphite-theme">);

boolean_feature!(pub enum MozMaemoClassicMediaFeature<"-moz-maemo-classic-theme">);

boolean_feature!(pub enum MozImagesInMenusMediaFeature<"-moz-maemo-classic-theme">);

keyword_set!(pub enum MozOsVersionMediaFeatureKeyword {
	WindowsVista: "windows-vista",
	WindowsXp: "windows-xp",
	WindowsWin7: "windows-win7",
	WindowsWin8: "windows-win8",
	WindowsWin10: "windows-win10",
});

discrete_feature!(pub enum MozOsVersionMediaFeature<"-moz-os-version", MozOsVersionMediaFeatureKeyword>);

boolean_feature!(pub enum MozTouchEnabledMediaFeature<"-moz-touch-enabled">);
