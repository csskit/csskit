use super::prelude::*;

/// <https://drafts.csswg.org/css-box-4/#typedef-paint-box>
///
/// ```text,ignore
/// <paint-box> = <visual-box> | fill-box | stroke-box
/// ```
#[derive(Parse, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum PaintBox {
	#[atom(CssAtomSet::ContentBox)]
	ContentBox(T![Ident]),
	#[atom(CssAtomSet::PaddingBox)]
	PaddingBox(T![Ident]),
	#[atom(CssAtomSet::BorderBox)]
	BorderBox(T![Ident]),
	#[atom(CssAtomSet::FillBox)]
	FillBox(T![Ident]),
	#[atom(CssAtomSet::StrokeBox)]
	StrokeBox(T![Ident]),
}
