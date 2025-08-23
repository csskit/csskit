use css_parse::keyword_set;
use csskit_derives::Visitable;

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
