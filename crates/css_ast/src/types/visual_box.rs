use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-box-4/#typedef-visual-box>
	///
	/// ```text,ignore
	/// <visual-box> = content-box | padding-box | border-box
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum VisualBox {
		ContentBox: "content-box",
		PaddingBox: "padding-box",
		BorderBox: "border-box"
	}
);
