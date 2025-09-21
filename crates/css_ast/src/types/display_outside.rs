use super::prelude::*;

/// <https://drafts.csswg.org/css-display-4/#typedef-display-outside>
///
/// ```text,ignore
/// <display-outside>  = block | inline | run-in
/// ```
#[derive(Parse, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum DisplayOutside {
	#[atom(CssAtomSet::Block)]
	Block(T![Ident]),
	#[atom(CssAtomSet::Inline)]
	Inline(T![Ident]),
	#[atom(CssAtomSet::RunIn)]
	RunIn(T![Ident]),
}
