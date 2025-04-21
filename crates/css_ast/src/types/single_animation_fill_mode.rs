use css_parse::keyword_set;

// https://drafts.csswg.org/css-animations/#typedef-single-animation-fill-mode
// <single-animation-fill-mode> = none | forwards | backwards | both
keyword_set!(SingleAnimationFillMode { None: "none", Forwards: "forwards", Backwards: "backwards", Both: "both" });
