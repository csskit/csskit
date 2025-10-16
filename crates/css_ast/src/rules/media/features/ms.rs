use super::prelude::*;
use crate::{CSSFloat, CSSInt};

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum MsHighContrastMediaFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Active)]
	Active(T![Ident]),
}

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MsHighContrastMediaFeature{CssAtomSet::_MsHighContrast, MsHighContrastMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum MsViewStateMediaFeatureKeyword {
	#[atom(CssAtomSet::Snapped)]
	Snapped(T![Ident]),
	#[atom(CssAtomSet::FullscreenPortait)]
	FullscreenPortait(T![Ident]),
	#[atom(CssAtomSet::FullscreenLandscape)]
	FullscreenLandscape(T![Ident]),
}

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MsViewStateMediaFeature{CssAtomSet::_MsViewState, MsViewStateMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MsImeAlignMediaFeatureKeyword(#[atom(CssAtomSet::Auto)] T![Ident]);

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MsImeAlignMediaFeature{CssAtomSet::_MsImeAlign, MsImeAlignMediaFeatureKeyword}
);

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MsDevicePixelRatioMediaFeature{CssAtomSet::_MsDevicePixelRatio | CssAtomSet::_MsMinDevicePixelRatio | CssAtomSet::_MsMaxDevicepixelRatio, CSSFloat}
);

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MsColumnCountMediaFeature{CssAtomSet::_MsColumnCount | CssAtomSet::_MsMinColumnCount | CssAtomSet::_MsMaxColumnCount, CSSInt}
);
