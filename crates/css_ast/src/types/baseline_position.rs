use css_parse::keyword_set;
use csskit_derives::Visitable;

keyword_set!(
	/// <https://drafts.csswg.org/css-align-3/#typedef-baseline-position>
	///
	/// ```text,ignore
	/// <baseline-position> = [ first | last ]? && baseline
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum BaselinePosition {
		First: "first",
		Last: "last",
		Baseline: "baseline"
	}
);
