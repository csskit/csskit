use crate::{
	BadDeclaration, CursorSink, Declaration, DeclarationValue, NodeMetadata, NodeWithMetadata, SemanticEq, Span,
	ToCursors, ToSpan,
};

/// Either a valid declaration or a bad declaration consumed for error recovery.
///
/// Per the CSS spec, when parsing fails for both a declaration and a rule,
/// we consume the remnants as a bad declaration to maintain error recovery.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum DeclarationOrBad<'a, D, M>
where
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	Declaration(Declaration<'a, D, M>),
	Bad(BadDeclaration<'a>),
}

impl<'a, D, M> ToCursors for DeclarationOrBad<'a, D, M>
where
	D: DeclarationValue<'a, M> + ToCursors,
	M: NodeMetadata,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Declaration(d) => d.to_cursors(s),
			Self::Bad(b) => b.to_cursors(s),
		}
	}
}

impl<'a, D, M> ToSpan for DeclarationOrBad<'a, D, M>
where
	D: DeclarationValue<'a, M> + ToSpan,
	M: NodeMetadata,
{
	fn to_span(&self) -> Span {
		match self {
			Self::Declaration(d) => d.to_span(),
			Self::Bad(b) => b.to_span(),
		}
	}
}

impl<'a, D, M> SemanticEq for DeclarationOrBad<'a, D, M>
where
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn semantic_eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Declaration(a), Self::Declaration(b)) => a.semantic_eq(b),
			(Self::Bad(a), Self::Bad(b)) => a.semantic_eq(b),
			_ => false,
		}
	}
}

impl<'a, D, M> NodeWithMetadata<M> for DeclarationOrBad<'a, D, M>
where
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn metadata(&self) -> M {
		match self {
			Self::Declaration(d) => d.metadata(),
			Self::Bad(b) => b.metadata(),
		}
	}
}
