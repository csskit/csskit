use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-backgrounds-4/#typedef-bg-clip>
	/// <https://drafts.csswg.org/css-box-4/#typedef-visual-box>
	///
	/// ```text,ignore
	/// <bg-clip> = <visual-box> | border-area | text
	/// <visual-box> = <visual-box> | margin-box
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum BgClip {
		ContentBox: "content-box",
		LayoutBox: "padding-box",
		BorderBox: "border-box",
		BorderArea: "border-area",
		Text: "text",
	}
);
