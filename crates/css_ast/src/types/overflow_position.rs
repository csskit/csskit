use css_parse::keyword_set;
use csskit_derives::Visitable;

keyword_set!(
	/// <https://drafts.csswg.org/css-align-3/#typedef-overflow-position>
	///
	/// ```text,ignore
	/// <overflow-position> = unsafe | safe
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum OverflowPosition {
		Unsafe: "unsafe",
		Safe: "safe"
	}
);
