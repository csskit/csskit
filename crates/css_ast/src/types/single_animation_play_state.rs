use css_parse::keyword_set;

// https://drafts.csswg.org/css-animations/#typedef-single-animation-play-state
// <single-animation-play-state> = running | paused
keyword_set!(SingleAnimationPlayState { Running: "running", Paused: "paused" });
