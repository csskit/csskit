use super::prelude::*;

/// <https://drafts.csswg.org/css-box-4/#typedef-coord-box>
///
/// ```text,ignore
/// <coord-box> = <paint-box> | view-box
/// ```
#[derive(Parse, Peek, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum CoordBox {
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
	#[atom(CssAtomSet::ViewBox)]
	ViewBox(T![Ident]),
}
