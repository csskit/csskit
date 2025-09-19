use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation-trigger-type>
	///
	/// ```text,ignore
	/// <single-animation-trigger-type> = once | repeat | alternate | state
	/// ```
	pub enum SingleAnimationTriggerType {
		Once: "once",
		Repeat: "repeat",
		Alternate: "alternate",
		State: "state"
	}
);
