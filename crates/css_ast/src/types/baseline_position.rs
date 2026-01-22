use super::prelude::*;

/// <https://drafts.csswg.org/css-align-3/#typedef-baseline-position>
///
/// ```text,ignore
/// <baseline-position> = [ first | last ]? && baseline
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum BaselinePosition {
	#[atom(CssAtomSet::First)]
	First(T![Ident]),
	#[atom(CssAtomSet::Last)]
	Last(T![Ident]),
	#[atom(CssAtomSet::Baseline)]
	Baseline(T![Ident]),
}
