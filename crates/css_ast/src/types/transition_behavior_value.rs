use css_parse::keyword_set;

// https://drafts.csswg.org/css-transitions-2/#typedef-transition-behavior-value
// <transition-behavior-value> = normal | allow-discrete
keyword_set!(TransitionBehaviorValue { Normal: "normal", AllowDiscrete: "allow-discrete" });
