use css_parse::keyword_set;

// https://drafts.csswg.org/css-animations/#typedef-single-animation-direction
// <single-animation-direction> = normal | reverse | alternate | alternate-reverse
keyword_set!(SingleAnimationDirection {
	Normal: "normal",
	Reverse: "reverse",
	Alternate: "alternate",
	AlternateReverse: "alternate-reverse",
});
