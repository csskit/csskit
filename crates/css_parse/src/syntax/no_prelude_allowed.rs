use crate::{Cursor, CursorSink, Parse, Parser, Peek, Result, Span, T, ToCursors, ToSpan, diagnostics};

/// A struct to provide to [AtRule][crate::AtRule] to disallow preludes.
///
/// Sometimes [AtRules][crate::syntax::AtRule] do not have a prelude. In those case, assigning this struct to the
/// `Prelude` can be useful to ensure that the [AtRule][crate::syntax::AtRule] appropriately errors if it enters the
/// Prelude parsing context.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NoPreludeAllowed;

impl<'a> Parse<'a> for NoPreludeAllowed {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<T![LeftCurly]>() || p.peek::<T![;]>() {
			Ok(Self {})
		} else {
			let c = p.peek_next();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
	}
}

impl<'a> Peek<'a> for NoPreludeAllowed {
	fn peek(_: &Parser<'a>, _: Cursor) -> bool {
		false
	}
}

impl ToCursors for NoPreludeAllowed {
	fn to_cursors(&self, _: &mut impl CursorSink) {
		// No cursors
	}
}

impl ToSpan for NoPreludeAllowed {
	fn to_span(&self) -> Span {
		Span::ZERO
	}
}
