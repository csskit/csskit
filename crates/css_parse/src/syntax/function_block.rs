use crate::{ComponentValues, CursorSink, Function, Parse, Parser, Result as ParserResult, ToCursors, token_macros};
use csskit_derives::ToSpan;

#[derive(ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct FunctionBlock<'a>(Function<token_macros::Function, ComponentValues<'a>>);

// https://drafts.csswg.org/css-syntax-3/#consume-function
impl<'a> Parse<'a> for FunctionBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		p.parse::<Function<token_macros::Function, ComponentValues<'a>>>().map(Self)
	}
}

impl<'a> ToCursors for FunctionBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.0, s);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FunctionBlock>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FunctionBlock, "foo(bar)");
		assert_parse!(FunctionBlock, "foo(bar{})");
	}
}
