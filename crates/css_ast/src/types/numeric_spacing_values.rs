use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#numeric-spacing-values>
///
/// ```text,ignore
/// <numeric-spacing-values> = [ proportional-nums | tabular-nums ]
/// ```
#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum NumericSpacingValues {
	#[atom(CssAtomSet::ProportionalNums)]
	ProportionalNums(T![Ident]),
	#[atom(CssAtomSet::TabularNums)]
	TabularNums(T![Ident]),
}
