use super::prelude::*;
use crate::units::CSSFloat;

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum ODevicePixelRatioMediaFeature<CssAtomSet::_ODevicePixelRatio | CssAtomSet::_OMinDevicePixelRatio | CssAtomSet::_OMaxDevicePixelRatio, CSSFloat>
);
