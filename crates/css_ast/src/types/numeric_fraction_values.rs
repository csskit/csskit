use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#numeric-fraction-values>
///
/// ```text,ignore
/// <numeric-fraction-values> = [ diagonal-fractions | stacked-fractions ]
/// ```
#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum NumericFractionValues {
	#[atom(CssAtomSet::DiagonalFractions)]
	DiagonalFractions(T![Ident]),
	#[atom(CssAtomSet::StackedFractions)]
	StackedFractions(T![Ident]),
}
