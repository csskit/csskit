use css_parse::keyword_set;

keyword_set!(
	/// <https://drafts.csswg.org/css-anchor-position-1/#typedef-try-size>
	///
	/// ```text,ignore
	/// <try-size> = most-width | most-height | most-block-size | most-inline-size
	/// ```
	pub enum TrySize {
		MostWidth: "most-width",
		MostHeight: "most-height",
		MostBlockSize: "most-block-size",
		MostInlineSize: "most-inline-size",
	}
);
