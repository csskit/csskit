use css_parse::keyword_set;

keyword_set!(
	/// <https://drafts.csswg.org/css-align-3/#typedef-content-distribution>
	///
	/// ```text,ignore
	/// <content-distribution> = space-between | space-around | space-evenly | stretch
	/// ```
	pub enum ContentDistribution {
		SpaceBetween: "space-between",
		SpaceAround: "space-around",
		SpaceEvenly: "space-evenly",
		Stretch: "stretch",
	}
);
