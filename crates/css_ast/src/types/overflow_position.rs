use css_parse::keyword_set;

keyword_set!(
	/// <https://drafts.csswg.org/css-align-3/#typedef-overflow-position>
	///
	/// ```text,ignore
	/// <overflow-position> = unsafe | safe
	/// ```
	pub enum OverflowPosition {
		Unsafe: "unsafe",
		Safe: "safe"
	}
);
