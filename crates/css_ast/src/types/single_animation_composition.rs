use css_parse::keyword_set;

// https://drafts.csswg.org/css-animations-2/#typedef-single-animation-composition
// <single-animation-composition> = replace | add | accumulate
keyword_set!(SingleAnimationComposition { Replace: "replace", Add: "add", Accumulate: "accumulate" });
