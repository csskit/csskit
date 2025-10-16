use super::prelude::*;
use crate::units::CSSFloat;

// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#media_features

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MozDevicePixelRatioMediaFeature{CssAtomSet::_MozDevicePixelRatio | CssAtomSet::_MozMinDevicePixelRatio | CssAtomSet::_MozMaxDevicePixelRatio, CSSFloat}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum MozDeviceOrientationMediaFeatureKeyword {
	#[atom(CssAtomSet::Portrait)]
	Portrait(T![Ident]),
	#[atom(CssAtomSet::Landscape)]
	Landscape(T![Ident]),
}

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MozDeviceOrientationMediaFeature{CssAtomSet::_MozDeviceOrientation, MozDeviceOrientationMediaFeatureKeyword}
);

boolean_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MozMacGraphiteThemeMediaFeature{CssAtomSet::_MozMacGraphiteTheme}
);

boolean_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MozMaemoClassicMediaFeature{CssAtomSet::_MozMaemoClassicTheme}
);

boolean_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MozImagesInMenusMediaFeature{CssAtomSet::_MozMaemoClassicTheme}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum MozOsVersionMediaFeatureKeyword {
	#[atom(CssAtomSet::WindowsVista)]
	WindowsVista(T![Ident]),
	#[atom(CssAtomSet::WindowsXp)]
	WindowsXp(T![Ident]),
	#[atom(CssAtomSet::WindowsWin7)]
	WindowsWin7(T![Ident]),
	#[atom(CssAtomSet::WindowsWin8)]
	WindowsWin8(T![Ident]),
	#[atom(CssAtomSet::WindowsWin10)]
	WindowsWin10(T![Ident]),
}

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MozOsVersionMediaFeature{CssAtomSet::_MozOsVersion, MozOsVersionMediaFeatureKeyword}
);

boolean_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MozTouchEnabledMediaFeature{CssAtomSet::_MozTouchEnabled}
);
