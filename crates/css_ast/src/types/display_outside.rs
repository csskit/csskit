use super::prelude::*;

/// <https://drafts.csswg.org/css-display-4/#typedef-display-outside>
///
/// ```text,ignore
/// <display-outside>  = block | inline | run-in
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum DisplayOutside {
	#[atom(CssAtomSet::Block)]
	Block(T![Ident]),
	#[atom(CssAtomSet::Inline)]
	Inline(T![Ident]),
	#[atom(CssAtomSet::RunIn)]
	RunIn(T![Ident]),
}
