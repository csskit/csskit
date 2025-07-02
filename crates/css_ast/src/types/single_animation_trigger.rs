#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

// https://drafts.csswg.org/css-animations-2/#typedef-single-animation-trigger
// <single-animation-trigger> = <single-animation-trigger-behavior> || [ none | auto | [ [ <dashed-ident> | <scroll()> | <view()> ] [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]{0,4} ] ]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SingleAnimationTrigger;

impl<'a> Peek<'a> for SingleAnimationTrigger {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		todo!();
	}
}

impl<'a> Parse<'a> for SingleAnimationTrigger {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		todo!();
	}
}

impl<'a> ToCursors for SingleAnimationTrigger {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		todo!();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
