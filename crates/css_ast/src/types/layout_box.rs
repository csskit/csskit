use super::prelude::*;

/// <https://drafts.csswg.org/css-box-4/#typedef-layout-box>
///
/// ```text,ignore
/// <layout-box> = <visual-box> | margin-box
/// ```
#[derive(Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum LayoutBox {
	#[atom(CssAtomSet::ContentBox)]
	ContentBox(T![Ident]),
	#[atom(CssAtomSet::PaddingBox)]
	PaddingBox(T![Ident]),
	#[atom(CssAtomSet::BorderBox)]
	BorderBox(T![Ident]),
	#[atom(CssAtomSet::MarginBox)]
	MarginBox(T![Ident]),
}
