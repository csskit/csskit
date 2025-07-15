use css_parse::{pseudo_class, pseudo_element};
use csskit_proc_macro::visit;

use crate::{Visit, Visitable};

pseudo_element!(
	#[visit]
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

impl<'a> Visitable<'a> for MsPseudoElement {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_ms_pseudo_element(self);
	}
}

pseudo_class!(
	#[visit]
	pub enum MsPseudoClass {
		Fullscreen: "-ms-fullscreen",
		InputPlaceholder: "-ms-input-placeholder"
	}
);

impl<'a> Visitable<'a> for MsPseudoClass {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_ms_pseudo_class(self);
	}
}
