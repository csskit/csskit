use css_parse::keyword_set;

keyword_set!(
	/// https://drafts.csswg.org/css2/#value-def-absolute-size
	///
	/// ```text,ignore
	/// <absolute-size> = [ xx-small | x-small | small | medium | large | x-large | xx-large ]
	/// ```
	pub enum AbsoluteSize {
		XxSmall: "xx-small",
		XSmall: "x-small",
		Small: "small",
		Medium: "medium",
		Large: "large",
		XLarge: "x-large",
		XxLarge: "xx-large",
	}
);
