use css_parse::keyword_set;

keyword_set!(
	/// <https://drafts.csswg.org/css-transitions-2/#typedef-transition-behavior-value>
	///
	/// ```text,ignore
	/// <transition-behavior-value> = normal | allow-discrete
	/// ```
	pub enum TransitionBehaviorValue {
		Normal: "normal",
		AllowDiscrete: "allow-discrete"
	}
);
