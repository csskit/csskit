use css_parse::keyword_set;

keyword_set!(
	/// https://drafts.csswg.org/css-animations/#typedef-single-animation-direction
	///
	/// ```text,ignore
	/// <single-animation-direction> = normal | reverse | alternate | alternate-reverse
	/// ```
	pub enum SingleAnimationDirection {
		Normal: "normal",
		Reverse: "reverse",
		Alternate: "alternate",
		AlternateReverse: "alternate-reverse",
	}
);
