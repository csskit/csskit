use css_parse::keyword_set;
use csskit_derives::Visitable;

keyword_set!(
	/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation-composition>
	///
	/// ```text,ignore
	/// <single-animation-composition> = replace | add | accumulate
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum SingleAnimationComposition {
		Replace: "replace",
		Add: "add",
		Accumulate: "accumulate"
	}
);
