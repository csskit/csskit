use css_parse::keyword_set;

keyword_set!(
	///  https://drafts.csswg.org/css-align-3/#typedef-self-position
	///
	/// ```text,ignore
	/// <self-position> = center | start | end | self-start | self-end | flex-start | flex-end
	/// ```
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
