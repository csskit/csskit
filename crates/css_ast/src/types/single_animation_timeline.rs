use css_parse::keyword_set;
use csskit_derives::Visitable;

keyword_set!(
	/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation-timeline>
	///
	/// ```text,ignore
	/// <single-animation-timeline> = auto | none | <dashed-ident> | <scroll()> | <view()>
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum SingleAnimationTimeline {
		Auto: "auto",
		None: "none"
	}
);
