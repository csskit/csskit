use super::prelude::*;
use crate::{CSSFloat, CSSInt};

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum MsHighContrastMediaFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Active)]
	Active(T![Ident]),
}

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MsHighContrastMediaFeature{CssAtomSet::_MsHighContrast, MsHighContrastMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MsViewStateMediaFeature{CssAtomSet::_MsViewState, MsViewStateMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MsImeAlignMediaFeatureKeyword(#[atom(CssAtomSet::Auto)] T![Ident]);

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MsImeAlignMediaFeature{CssAtomSet::_MsImeAlign, MsImeAlignMediaFeatureKeyword}
);

ranged_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MsDevicePixelRatioMediaFeature{CssAtomSet::_MsDevicePixelRatio | CssAtomSet::_MsMinDevicePixelRatio | CssAtomSet::_MsMaxDevicepixelRatio, CSSFloat}
);

ranged_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MsColumnCountMediaFeature{CssAtomSet::_MsColumnCount | CssAtomSet::_MsMinColumnCount | CssAtomSet::_MsMaxColumnCount, CSSInt}
);
