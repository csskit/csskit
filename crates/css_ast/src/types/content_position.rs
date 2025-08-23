use css_parse::keyword_set;
use csskit_derives::Visitable;

keyword_set!(
	/// <https://drafts.csswg.org/css-align-3/#typedef-content-position>
	///
	/// ```text,ignore
	/// <content-position> = center | start | end | flex-start | flex-end
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum ContentPosition {
		Center: "center",
		Start: "start",
		End: "end",
		FlexStart: "flex-start",
		FlexEnd: "flex-end",
	}
);
