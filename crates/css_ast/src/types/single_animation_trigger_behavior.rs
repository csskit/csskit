use css_parse::keyword_set;

keyword_set!(
	/// https://drafts.csswg.org/css-animations-2/#typedef-single-animation-trigger-behavior
	///
	/// ```text,ignore
	/// <single-animation-trigger-behavior> = once | repeat | alternate | state
	/// ```
	pub enum SingleAnimationTriggerBehavior {
		Once: "once",
		Repeat: "repeat",
		Alternate: "alternate",
		State: "state"
	}
);
