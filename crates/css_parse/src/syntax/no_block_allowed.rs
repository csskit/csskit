use crate::{Cursor, CursorSink, Parse, Parser, Peek, Result, Span, T, ToCursors, ToSpan, diagnostics};

/// A struct to provide to [AtRule][crate::AtRule] to disallow blocks.
///
/// Sometimes [AtRules][crate::syntax::AtRule] do not have a block - for example `@charset`, `@import`. In those case, assigning
/// this struct to the `Block` can be useful to ensure that the [AtRule][crate::syntax::AtRule] appropriately errors if it enters the
/// Block parsing context. This captures the `;` token that may optionally end a "statement-style" at-rule.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NoBlockAllowed(Option<T![;]>);

impl<'a> Parse<'a> for NoBlockAllowed {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.at_end() {
			Ok(Self(None))
		} else if let Some(semicolon) = p.parse_if_peek::<T![;]>()? {
			Ok(Self(Some(semicolon)))
		} else {
			Err(diagnostics::Unexpected(p.next()))?
		}
	}
}

impl<'a> Peek<'a> for NoBlockAllowed {
	fn peek(_: &Parser<'a>, _: Cursor) -> bool {
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
