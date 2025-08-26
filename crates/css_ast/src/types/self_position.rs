use css_parse::keyword_set;
use csskit_derives::Visitable;

keyword_set!(
	///  https://drafts.csswg.org/css-align-3/#typedef-self-position
	///
	/// ```text,ignore
	/// <self-position> = center | start | end | self-start | self-end | flex-start | flex-end
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum SelfPosition {
		Center: "center",
		Start: "start",
		End: "end",
		SelfStart: "self-start",
		SelfEnd: "self-end",
		FlexStart: "flex-start",
		FlexEnd: "flex-end",
	}
);
