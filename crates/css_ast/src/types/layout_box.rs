use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-box-4/#typedef-layout-box>
	///
	/// ```text,ignore
	/// <layout-box> = <visual-box> | margin-box
	/// ```
	pub enum LayoutBox {
		ContentBox: "content-box",
		LayoutBox: "padding-box",
		BorderBox: "border-box",
		MarginBox: "margin-box",
	}
);
