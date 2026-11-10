use super::prelude::*;
use crate::GeometryBox;

/// A mask box, including the legacy WebKit box keywords.
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(self))]
#[derive(NodeWithMetadata)]
pub enum WebkitMaskBox {
	#[atom(CssAtomSet::Border)]
	Border(T![Ident]),
	#[atom(CssAtomSet::Padding)]
	Padding(T![Ident]),
	#[atom(CssAtomSet::Content)]
	Content(T![Ident]),
	#[atom(CssAtomSet::Text)]
	Text(T![Ident]),
	GeometryBox(GeometryBox),
}
