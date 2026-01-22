use super::prelude::*;

/// <https://drafts.csswg.org/css-align-3/#typedef-content-distribution>
///
/// ```text,ignore
/// <content-distribution> = space-between | space-around | space-evenly | stretch
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
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
