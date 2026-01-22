use crate::CssAtomSet;
use css_parse::{Diagnostic, pseudo_class, pseudo_element};
use csskit_derives::{SemanticEq, ToCursors, ToSpan};

pseudo_element!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MsPseudoElement {
		Backdrop: CssAtomSet::_MsBackdrop,
		Browse: CssAtomSet::_MsBrowse,
		Check: CssAtomSet::_MsCheck,
		Clear: CssAtomSet::_MsClear,
		Expand: CssAtomSet::_MsExpand,
		Fill: CssAtomSet::_MsFill,
		FillUpper: CssAtomSet::_MsFillUpper,
		FillLower: CssAtomSet::_MsFillLower,
		InputPlaceholder: CssAtomSet::_MsInputPlaceholder,
		Placeholder: CssAtomSet::_MsPlaceholder,
		Reveal: CssAtomSet::_MsReveal,
		Selection: CssAtomSet::_MsSelection,
		Thumb: CssAtomSet::_MsThumb,
		TicksAfter: CssAtomSet::_MsTicksAfter,
		TicksBefore: CssAtomSet::_MsTicksBefore,
		Tooltip: CssAtomSet::_MsTooltip,
		Track: CssAtomSet::_MsTrack,
		Value: CssAtomSet::_MsValue,
	}
);

pseudo_class!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MsPseudoClass {
		Fullscreen: CssAtomSet::_MsFullscreen,
		InputPlaceholder: CssAtomSet::_MsInputPlaceholder
	}
);
