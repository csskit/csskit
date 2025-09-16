use css_parse::{Diagnostic, pseudo_class, pseudo_element};
use csskit_derives::Visitable;

pseudo_element!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum OPseudoElement {
		InnerSpinButton: "-o-inner-spin-button",
		OuterSpinButton: "-o-outer-spin-button",
		Placeholder: "-o-placeholder",
		Scrollbar: "-o-scrollbar",
		ScrollbarThumb: "-o-scrollbar-thumb",
		ScrollbarTrack: "-o-scrollbar-track",
		ScrollbarTrackPiece: "-o-scrollbar-track-piece",
		Selection: "-o-selection",
	}
);

pseudo_class!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum OPseudoClass {
		Prefocus: "-o-prefocus"
	}
);
