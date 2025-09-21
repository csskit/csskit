use super::prelude::*;

/// <https://drafts.csswg.org/css-align-3/#typedef-content-distribution>
///
/// ```text,ignore
/// <content-distribution> = space-between | space-around | space-evenly | stretch
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum ContentDistribution {
	#[atom(CssAtomSet::SpaceBetween)]
	SpaceBetween(T![Ident]),
	#[atom(CssAtomSet::SpaceAround)]
	SpaceAround(T![Ident]),
	#[atom(CssAtomSet::SpaceEvenly)]
	SpaceEvenly(T![Ident]),
	#[atom(CssAtomSet::Stretch)]
	Stretch(T![Ident]),
}
