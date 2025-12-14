use crate::{
	ComponentValues, Cursor, CursorSink, DeclarationValue, NodeMetadata, NodeWithMetadata, Parse, Parser, Peek, Result,
	SemanticEq, Span, ToCursors, ToSpan,
};

/// Wrapper type for using ComponentValues as a rule type parameter in unknown rules.
/// This implements RuleVariants to allow ComponentValues to be used as the block type
/// for unknown qualified rules, where the rule structure is not recognized.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct UnknownRuleBlock<'a, D = ComponentValues<'a>, M = ()> {
	pub values: ComponentValues<'a>,
	#[cfg_attr(feature = "serde", serde(skip))]
	_phantom: std::marker::PhantomData<(D, M)>,
}

impl<'a, D, M> Parse<'a> for UnknownRuleBlock<'a, D, M> {
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = Cursor> + Clone,
	{
		ComponentValues::parse(p).map(|values| Self { values, _phantom: std::marker::PhantomData })
	}
}

impl<'a, D, M> Peek<'a> for UnknownRuleBlock<'a, D, M> {
	fn peek<Iter>(p: &Parser<'a, Iter>, c: Cursor) -> bool
	where
		Iter: Iterator<Item = Cursor> + Clone,
	{
		ComponentValues::peek(p, c)
	}
}

impl<'a, D, M> ToCursors for UnknownRuleBlock<'a, D, M> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		self.values.to_cursors(s)
	}
}

impl<'a, D, M> ToSpan for UnknownRuleBlock<'a, D, M> {
	fn to_span(&self) -> Span {
		self.values.to_span()
	}
}

impl<'a, D, M> SemanticEq for UnknownRuleBlock<'a, D, M> {
	fn semantic_eq(&self, other: &Self) -> bool {
		self.values.semantic_eq(&other.values)
	}
}

impl<'a, D, M: NodeMetadata> NodeWithMetadata<M> for UnknownRuleBlock<'a, D, M> {
	fn metadata(&self) -> M {
		self.values.metadata()
	}
}

impl<'a, D, M> crate::RuleVariants<'a> for UnknownRuleBlock<'a, D, M>
where
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	type DeclarationValue = D;
	type Metadata = M;

	fn is_unknown(&self) -> bool {
		// ComponentValues is a generic fallback container, not a specific rule type.
		// It should be treated as unknown so it doesn't override actual declarations.
		true
	}
}
