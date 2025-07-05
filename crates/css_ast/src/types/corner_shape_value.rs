#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

// https://drafts.csswg.org/css-borders-4/#typedef-corner-shape-value
// <corner-shape-value> = round | scoop | bevel | notch | square | squircle | <superellipse()>
// superellipse() = superellipse(<number [-∞,∞]> | infinity | -infinity)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CornerShapeValue;

impl<'a> Peek<'a> for CornerShapeValue {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		todo!();
	}
}

impl<'a> Parse<'a> for CornerShapeValue {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		todo!();
	}
}

impl<'a> ToCursors for CornerShapeValue {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		todo!();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
