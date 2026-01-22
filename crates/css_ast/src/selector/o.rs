use crate::CssAtomSet;
use css_parse::{Diagnostic, pseudo_class, pseudo_element};
use csskit_derives::{SemanticEq, ToCursors, ToSpan};

pseudo_element!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
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
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum OPseudoClass {
		Prefocus: CssAtomSet::_OPrefocus,
	}
);
