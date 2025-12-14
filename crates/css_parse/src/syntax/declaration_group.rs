use crate::{
	CursorSink, DeclarationOrBad, DeclarationValue, NodeMetadata, NodeWithMetadata, SemanticEq, Span, ToCursors, ToSpan,
};
use bumpalo::collections::Vec;

/// A group of declarations that can be interleaved with rules.
///
/// Per [CSS Syntax § 5.4.4](https://drafts.csswg.org/css-syntax-3/#consume-block-contents),
/// blocks return a list containing either rules or lists of declarations. This allows
/// declarations to be properly interleaved with nested rules while maintaining their order.
///
/// For example, in `a { color: red; b { } color: blue; }`, the declarations need to be
/// grouped separately before and after the nested `b` rule.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct DeclarationGroup<'a, D, M>
where
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	pub declarations: Vec<'a, DeclarationOrBad<'a, D, M>>,
}

impl<'a, D, M> ToCursors for DeclarationGroup<'a, D, M>
where
	D: DeclarationValue<'a, M> + ToCursors,
	M: NodeMetadata,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for decl in &self.declarations {
			decl.to_cursors(s);
		}
	}
}

impl<'a, D, M> ToSpan for DeclarationGroup<'a, D, M>
where
	D: DeclarationValue<'a, M> + ToSpan,
	M: NodeMetadata,
{
	fn to_span(&self) -> Span {
		self.declarations.to_span()
	}
}

impl<'a, D, M> SemanticEq for DeclarationGroup<'a, D, M>
where
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn semantic_eq(&self, other: &Self) -> bool {
		self.declarations.semantic_eq(&other.declarations)
	}
}

impl<'a, D, M> NodeWithMetadata<M> for DeclarationGroup<'a, D, M>
where
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn metadata(&self) -> M {
		let mut meta = M::default();
		for decl in &self.declarations {
			meta.merge(&decl.metadata());
		}
		meta
	}
}
