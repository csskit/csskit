use css_parse::keyword_set;

// https://drafts.csswg.org/css-animations-2/#typedef-single-animation-trigger-type
// <single-animation-trigger-type> = once | repeat | alternate | state
keyword_set!(SingleAnimationTriggerType { Once: "once", Repeat: "repeat", Alternate: "alternate", State: "state" });
