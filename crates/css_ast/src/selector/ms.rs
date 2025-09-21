use crate::CssAtomSet;
use css_parse::{Diagnostic, pseudo_class, pseudo_element};
use csskit_derives::{ToCursors, ToSpan, Visitable};

pseudo_element!(
	#[derive(ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[visit(self)]
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
	#[derive(ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[visit(self)]
	pub enum MsPseudoClass {
		Fullscreen: CssAtomSet::_MsFullscreen,
		InputPlaceholder: CssAtomSet::_MsInputPlaceholder
	}
);
