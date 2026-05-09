use super::prelude::*;
use crate::ShapeBox;

/// <https://www.w3.org/TR/css-masking-1/#typedef-geometry-box>
///
/// ```text,ignore
/// <geometry-box> = <shape-box> | fill-box | stroke-box | view-box
/// ```
#[derive(Parse, Peek, SemanticEq, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum GeometryBox {
	ShapeBox(ShapeBox),
	#[atom(CssAtomSet::FillBox)]
	FillBox(T![Ident]),
	#[atom(CssAtomSet::StrokeBox)]
	StrokeBox(T![Ident]),
	#[atom(CssAtomSet::ViewBox)]
	ViewBox(T![Ident]),
}
