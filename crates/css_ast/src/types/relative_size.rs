use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css2/#value-def-relative-size>
	///
	/// ```text,ignore
	/// <relative-size> = larger | smaller
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum RelativeSize {
		Larger: "larger",
		Smaller: "smaller"
	}
);
