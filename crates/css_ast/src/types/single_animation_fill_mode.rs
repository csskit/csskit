use css_parse::keyword_set;

keyword_set!(
	/// <https://drafts.csswg.org/css-animations/#typedef-single-animation-fill-mode>
	///
	/// ```text,ignore
	/// <single-animation-fill-mode> = none | forwards | backwards | both
	/// ```
	pub enum SingleAnimationFillMode {
		None: "none",
		Forwards: "forwards",
		Backwards: "backwards",
		Both: "both"
	}
);
