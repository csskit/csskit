use css_parse::keyword_set;

keyword_set!(
	/// https://drafts.csswg.org/css-backgrounds-3/#typedef-attachment
	///
	/// ```text,ignore
	/// <attachment> = scroll | fixed | local
	/// ```
	pub enum Attachment {
		Scroll: "scroll",
		Fixed: "fixed",
		Local: "local"
	}
);
