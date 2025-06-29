use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SingleAnimationTriggerBehavior;

impl<'a> Peek<'a> for SingleAnimationTriggerBehavior {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		todo!();
	}
}

impl<'a> Parse<'a> for SingleAnimationTriggerBehavior {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		todo!();
	}
}

impl<'a> ToCursors for SingleAnimationTriggerBehavior {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		todo!();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
