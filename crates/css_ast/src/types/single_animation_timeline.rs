use css_parse::keyword_set;

keyword_set!(
	/// https://drafts.csswg.org/css-animations-2/#typedef-single-animation-timeline
	///
	/// ```text,ignore
	/// <single-animation-timeline> = auto | none | <dashed-ident> | <scroll()> | <view()>
	/// ```
	pub enum SingleAnimationTimeline {
		Auto: "auto",
		None: "none"
	}
);
