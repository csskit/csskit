use css_parse::{Diagnostic, pseudo_class, pseudo_element};
use csskit_derives::Visitable;

pseudo_element!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum MsPseudoElement {
		Backdrop: "-ms-backdrop",
		Browse: "-ms-browse",
		Check: "-ms-check",
		Clear: "-ms-clear",
		Expand: "-ms-expand",
		Fill: "-ms-fill",
		FillUpper: "-ms-fill-upper",
		FillLower: "-ms-fill-lower",
		InputPlaceholder: "-ms-input-placeholder",
		Placeholder: "-ms-placeholder",
		Reveal: "-ms-reveal",
		Selection: "-ms-selection",
		Thumb: "-ms-thumb",
		TicksAfter: "-ms-ticks-after",
		TicksBefore: "-ms-ticks-before",
		Tooltip: "-ms-tooltip",
		Track: "-ms-track",
		Value: "-ms-value",
	}
);

pseudo_class!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum MsPseudoClass {
		Fullscreen: "-ms-fullscreen",
		InputPlaceholder: "-ms-input-placeholder"
	}
);
