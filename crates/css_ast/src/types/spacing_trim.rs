use css_parse::keyword_set;

keyword_set!(
	/// <https://drafts.csswg.org/css-text-4/#typedef-spacing-trim>
	///
	/// ```text,ignore
	/// <spacing-trim> = space-all | normal | space-first | trim-start | trim-both | trim-all
	/// ```
	pub enum SpacingTrim {
		SpaceAll: "space-all",
		Normal: "normal",
		SpaceFirst: "space-first",
		TrimStart: "trim-start",
		TrimBoth: "trim-both",
		TrimAll: "trim-all",
	}
);
