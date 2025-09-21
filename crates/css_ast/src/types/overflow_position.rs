use super::prelude::*;

/// <https://drafts.csswg.org/css-align-3/#typedef-overflow-position>
///
/// ```text,ignore
/// <overflow-position> = unsafe | safe
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum OverflowPosition {
	#[atom(CssAtomSet::Unsafe)]
	Unsafe(T![Ident]),
	#[atom(CssAtomSet::Safe)]
	Safe(T![Ident]),
}
