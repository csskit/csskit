use super::prelude::*;
use crate::VisualBox;

/// <https://www.w3.org/TR/css-shapes-1/#typedef-shape-box>
///
/// ```text,ignore
/// <shape-box> = <visual-box> | margin-box
/// ```
#[derive(Parse, Peek, SemanticEq, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ShapeBox {
	VisualBox(VisualBox),
	#[atom(CssAtomSet::MarginBox)]
	MarginBox(T![Ident]),
}
