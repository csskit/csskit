use super::prelude::*;

/// <https://drafts.csswg.org/css-anchor-position-1/#typedef-try-size>
///
/// ```text,ignore
/// <try-size> = most-width | most-height | most-block-size | most-inline-size
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum TrySize {
	#[atom(CssAtomSet::MostWidth)]
	MostWidth(T![Ident]),
	#[atom(CssAtomSet::MostHeight)]
	MostHeight(T![Ident]),
	#[atom(CssAtomSet::MostBlockSize)]
	MostBlockSize(T![Ident]),
	#[atom(CssAtomSet::MostInlineSize)]
	MostInlineSize(T![Ident]),
}
