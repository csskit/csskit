use super::prelude::*;

/// <https://drafts.csswg.org/css-backgrounds-4/#typedef-bg-clip>
/// <https://drafts.csswg.org/css-box-4/#typedef-visual-box>
///
/// ```text,ignore
/// <bg-clip> = <visual-box> | border-area | text
/// <visual-box> = <visual-box> | margin-box
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum BgClip {
	#[atom(CssAtomSet::ContentBox)]
	ContentBox(T![Ident]),
	#[atom(CssAtomSet::PaddingBox)]
	LayoutBox(T![Ident]),
	#[atom(CssAtomSet::BorderBox)]
	BorderBox(T![Ident]),
	#[atom(CssAtomSet::BorderArea)]
	BorderArea(T![Ident]),
	#[atom(CssAtomSet::Text)]
	Text(T![Ident]),
}
