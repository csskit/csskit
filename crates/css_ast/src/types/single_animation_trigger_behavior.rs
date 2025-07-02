use css_parse::keyword_set;

// https://drafts.csswg.org/css-animations-2/#typedef-single-animation-trigger-behavior
// <single-animation-trigger-behavior> = once | repeat | alternate | state
keyword_set!(SingleAnimationTriggerBehavior {
	Once: "once",
	Repeat: "repeat",
	Alternate: "alternate",
	State: "state",
});
