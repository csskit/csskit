use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-backgrounds-3/#typedef-attachment>
	///
	/// ```text,ignore
	/// <attachment> = scroll | fixed | local
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum Attachment {
		Scroll: "scroll",
		Fixed: "fixed",
		Local: "local"
	}
);
