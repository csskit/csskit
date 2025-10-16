use super::prelude::*;

/// <https://drafts.csswg.org/css2/#value-def-relative-size>
///
/// ```text,ignore
/// <relative-size> = larger | smaller
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum RelativeSize {
	#[atom(CssAtomSet::Larger)]
	Larger(T![Ident]),
	#[atom(CssAtomSet::Smaller)]
	Smaller(T![Ident]),
}
