use css_parse::keyword_set;

keyword_set!(
	/// https://drafts.csswg.org/css2/#value-def-relative-size
	///
	/// ```text,ignore
	/// <relative-size> = larger | smaller
	/// ```
	pub enum RelativeSize {
		Larger: "larger",
		Smaller: "smaller"
	}
);
