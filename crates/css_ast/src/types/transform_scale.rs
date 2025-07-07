#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-scale
// scale() = scale( <number> , <number>? )
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Scale;

impl<'a> Peek<'a> for Scale {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		todo!();
	}
}

impl<'a> Parse<'a> for Scale {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		todo!();
	}
}

impl<'a> ToCursors for Scale {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		todo!();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Scale>(), 0);
	}

	#[test]
	fn test_writes() {}

	#[test]
	fn test_errors() {}
}
