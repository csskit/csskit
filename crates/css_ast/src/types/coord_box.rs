use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-box-4/#typedef-coord-box>
	///
	/// ```text,ignore
	/// <coord-box> = <paint-box> | view-box
	/// ```
	pub enum CoordBox {
		ContentBox: "content-box",
		PaddingBox: "padding-box",
		BorderBox: "border-box",
		FillBox: "fill-box",
		StrokeBox: "stroke-box",
		ViewBox: "view-box",
	}
);
