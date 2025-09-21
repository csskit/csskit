use crate::CssAtomSet;
use css_parse::{Diagnostic, pseudo_class, pseudo_element};
use csskit_derives::{ToCursors, ToSpan, Visitable};

pseudo_element!(
	#[derive(ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[visit(self)]
	pub enum OPseudoElement {
		InnerSpinButton: CssAtomSet::_OInnerSpinButton,
		OuterSpinButton: CssAtomSet::_OOuterSpinButton,
		Placeholder: CssAtomSet::_OPlaceholder,
		Scrollbar: CssAtomSet::_OScrollbar,
		ScrollbarThumb: CssAtomSet::_OScrollbarThumb,
		ScrollbarTrack: CssAtomSet::_OScrollbarTrack,
		ScrollbarTrackPiece: CssAtomSet::_OScrollbarTrackPiece,
		Selection: CssAtomSet::_OSelection,
	}
);

pseudo_class!(
	#[derive(ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[visit(self)]
	pub enum OPseudoClass {
		Prefocus: CssAtomSet::_OPrefocus,
	}
);
