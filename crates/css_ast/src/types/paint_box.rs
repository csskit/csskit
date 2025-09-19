use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-box-4/#typedef-paint-box>
	///
	/// ```text,ignore
	/// <paint-box> = <visual-box> | fill-box | stroke-box
	/// ```
	pub enum PaintBox {
		ContentBox: "content-box",
		PaddingBox: "padding-box",
		BorderBox: "border-box",
		FillBox: "fill-box",
		StrokeBox: "stroke-box",
	}
);
