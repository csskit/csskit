use crate::{Cursor, CursorSink, Diagnostic, Parse, Parser, Peek, Result, Span, T, ToCursors, ToSpan};

/// A struct to provide to rules to disallow blocks.
///
/// Sometimes a rule will not allow a block - for example `@charset`, `@import`. In those case, assigning this struct
/// to the `Block` can be useful to ensure that the [QualifiedRule][crate::syntax::QualifiedRule] appropriately errors
/// if it enters the Block parsing context. This captures the `;` token that may optionally end a "statement-style"
/// at-rule.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NoBlockAllowed(Option<T![;]>);

impl<'a> Parse<'a> for NoBlockAllowed {
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		if p.at_end() {
			Ok(Self(None))
		} else if let Some(semicolon) = p.parse_if_peek::<T![;]>()? {
			Ok(Self(Some(semicolon)))
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
		}
	}
}

impl<'a> Peek<'a> for NoBlockAllowed {
	fn peek<Iter>(_: &Parser<'a, Iter>, _: Cursor) -> bool
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		false
	}
}

impl ToCursors for NoBlockAllowed {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		if let Some(semicolon) = self.0 {
			s.append(semicolon.into());
		}
	}
}

impl ToSpan for NoBlockAllowed {
	fn to_span(&self) -> Span {
		self.0.to_span()
	}
}

impl<M: crate::NodeMetadata> crate::NodeWithMetadata<M> for NoBlockAllowed {
	fn metadata(&self) -> M {
		M::default()
	}
}
