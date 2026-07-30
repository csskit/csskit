use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#numeric-figure-values>
///
/// ```text,ignore
/// <numeric-figure-values> = [ lining-nums | oldstyle-nums ]
/// ```
#[node]
#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum NumericFigureValues {
	#[atom(CssAtomSet::LiningNums)]
	LiningNums(T![Ident]),
	#[atom(CssAtomSet::OldstyleNums)]
	OldstyleNums(T![Ident]),
}
