use super::prelude::*;

/// <https://drafts.csswg.org/css-align-3/#typedef-content-position>
///
/// ```text,ignore
/// <content-position> = center | start | end | flex-start | flex-end
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ContentPosition {
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::Start)]
	Start(T![Ident]),
	#[atom(CssAtomSet::End)]
	End(T![Ident]),
	#[atom(CssAtomSet::FlexStart)]
	FlexStart(T![Ident]),
	#[atom(CssAtomSet::FlexEnd)]
	FlexEnd(T![Ident]),
}
