use super::prelude::*;

/// <https://drafts.csswg.org/css-align-3/#typedef-baseline-position>
///
/// ```text,ignore
/// <baseline-position> = [ first | last ]? && baseline
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum BaselinePosition {
	#[atom(CssAtomSet::First)]
	First(T![Ident]),
	#[atom(CssAtomSet::Last)]
	Last(T![Ident]),
	#[atom(CssAtomSet::Baseline)]
	Baseline(T![Ident]),
}
