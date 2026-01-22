use super::prelude::*;

/// <https://drafts.csswg.org/css-backgrounds-3/#typedef-attachment>
///
/// ```text,ignore
/// <attachment> = scroll | fixed | local
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum Attachment {
	#[atom(CssAtomSet::Scroll)]
	Scroll(T![Ident]),
	#[atom(CssAtomSet::Fixed)]
	Fixed(T![Ident]),
	#[atom(CssAtomSet::Local)]
	Local(T![Ident]),
}
