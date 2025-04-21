use css_parse::keyword_set;

// https://drafts.csswg.org/css-animations-2/#typedef-single-animation-timeline
// <single-animation-timeline> = auto | none | <dashed-ident> | <scroll()> | <view()>
keyword_set!(SingleAnimationTimeline {
	Auto: "auto",
	None: "none",
});
