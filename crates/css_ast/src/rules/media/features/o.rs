use super::prelude::*;
use crate::units::CSSFloat;

ranged_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum ODevicePixelRatioMediaFeature{CssAtomSet::_ODevicePixelRatio | CssAtomSet::_OMinDevicePixelRatio | CssAtomSet::_OMaxDevicePixelRatio, CSSFloat}
);
