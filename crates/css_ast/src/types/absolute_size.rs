use super::prelude::*;

/// <https://drafts.csswg.org/css2/#value-def-absolute-size>
///
/// ```text,ignore
/// <absolute-size> = [ xx-small | x-small | small | medium | large | x-large | xx-large ]
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum AbsoluteSize {
	#[atom(CssAtomSet::XxSmall)]
	XxSmall(T![Ident]),
	#[atom(CssAtomSet::XSmall)]
	XSmall(T![Ident]),
	#[atom(CssAtomSet::Small)]
	Small(T![Ident]),
	#[atom(CssAtomSet::Medium)]
	Medium(T![Ident]),
	#[atom(CssAtomSet::Large)]
	Large(T![Ident]),
	#[atom(CssAtomSet::XLarge)]
	XLarge(T![Ident]),
	#[atom(CssAtomSet::XxLarge)]
	XxLarge(T![Ident]),
}
