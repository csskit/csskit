use css_parse::keyword_set;
use csskit_derives::Visitable;

keyword_set!(
	/// <https://drafts.csswg.org/css-animations/#typedef-single-animation-fill-mode>
	///
	/// ```text,ignore
	/// <single-animation-fill-mode> = none | forwards | backwards | both
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum SingleAnimationFillMode {
		None: "none",
		Forwards: "forwards",
		Backwards: "backwards",
		Both: "both"
	}
);
