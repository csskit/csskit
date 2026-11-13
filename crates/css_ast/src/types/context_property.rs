use super::prelude::*;

/// Identifies a paint property passed from an embedding element to an SVG image.
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(self))]
#[derive(NodeWithMetadata)]
pub enum ContextProperty {
	#[atom(CssAtomSet::Fill)]
	Fill(T![Ident]),
	#[atom(CssAtomSet::FillOpacity)]
	FillOpacity(T![Ident]),
	#[atom(CssAtomSet::Stroke)]
	Stroke(T![Ident]),
	#[atom(CssAtomSet::StrokeOpacity)]
	StrokeOpacity(T![Ident]),
}
