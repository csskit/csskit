use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation-trigger-behavior>
	///
	/// ```text,ignore
	/// <single-animation-trigger-behavior> = once | repeat | alternate | state
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum SingleAnimationTriggerBehavior {
		Once: "once",
		Repeat: "repeat",
		Alternate: "alternate",
		State: "state"
	}
);
