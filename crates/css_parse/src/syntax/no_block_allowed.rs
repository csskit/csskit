use crate::{Cursor, CursorSink, Diagnostic, Parse, Parser, Peek, Result, SemanticEq, Span, T, ToCursors, ToSpan};

/// A struct to provide to rules to disallow blocks.
///
/// Sometimes a rule will not allow a block - for example `@charset`, `@import`. In those case, assigning this struct
/// to the `Block` can be useful to ensure that the [QualifiedRule][crate::syntax::QualifiedRule] appropriately errors
/// if it enters the Block parsing context. This captures the `;` token that may optionally end a "statement-style"
/// at-rule.
///
/// The phantom data allows this type to be compatible with different declaration value and metadata types,
/// even though it doesn't actually use them (since no block is allowed).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NoBlockAllowed<D = (), M = ()> {
	semicolon: Option<crate::token_macros::Semicolon>,
	_phantom: std::marker::PhantomData<(D, M)>,
}

impl<'a, D, M> Parse<'a> for NoBlockAllowed<D, M> {
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		if p.at_end() {
			Ok(Self { semicolon: None, _phantom: std::marker::PhantomData })
		} else if let Some(semicolon) = p.parse_if_peek::<T![;]>()? {
			Ok(Self { semicolon: Some(semicolon), _phantom: std::marker::PhantomData })
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
		}
	}
}

impl<'a, D, M> Peek<'a> for NoBlockAllowed<D, M> {
	fn peek<Iter>(_: &Parser<'a, Iter>, _: Cursor) -> bool
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		false
	}
}

impl<D, M> ToCursors for NoBlockAllowed<D, M> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		if let Some(semicolon) = self.semicolon {
			s.append(semicolon.into());
		}
	}
}

impl<D, M> ToSpan for NoBlockAllowed<D, M> {
	fn to_span(&self) -> Span {
		self.semicolon.to_span()
	}
}

impl<D, M> SemanticEq for NoBlockAllowed<D, M> {
	fn semantic_eq(&self, other: &Self) -> bool {
		self.semicolon.semantic_eq(&other.semicolon)
	}
}

impl<D, M: crate::NodeMetadata> crate::NodeWithMetadata<M> for NoBlockAllowed<D, M> {
	fn metadata(&self) -> M {
		M::default()
	}
}

impl<'a, D, M> crate::RuleVariants<'a> for NoBlockAllowed<D, M>
where
	D: crate::DeclarationValue<'a, M>,
	M: crate::NodeMetadata,
{
	type DeclarationValue = D;
	type Metadata = M;
}
