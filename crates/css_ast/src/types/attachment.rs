use super::prelude::*;

/// <https://drafts.csswg.org/css-backgrounds-3/#typedef-attachment>
///
/// ```text,ignore
/// <attachment> = scroll | fixed | local
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum Attachment {
	#[atom(CssAtomSet::Scroll)]
	Scroll(T![Ident]),
	#[atom(CssAtomSet::Fixed)]
	Fixed(T![Ident]),
	#[atom(CssAtomSet::Local)]
	Local(T![Ident]),
}
