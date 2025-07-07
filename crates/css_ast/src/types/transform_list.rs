#![allow(warnings)]
use bumpalo::collections::Vec;
use css_lexer::{Cursor, Kind, Span};
use css_parse::{
	AtRule, Block, Build, ConditionKeyword, CursorSink, FeatureConditionList, Parse, Parser, Peek, PreludeList,
	Result as ParserResult, T, ToCursors, diagnostics, keyword_set,
};
use csskit_derives::{Parse, Peek, ToCursors};

use crate::TransformFunction;

// https://drafts.csswg.org/css-transforms-1/#typedef-transform-list
// <transform-list> = <transform-function>+
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TransformList<'a>(Vec<'a, TransformFunction>);

impl<'a> Peek<'a> for TransformList<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<TransformFunction>::peek(p, c)
	}
}

impl<'a> Parse<'a> for TransformList<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut list = Vec::new_in(p.bump());
		while let Some(transform) = p.parse_if_peek::<TransformFunction>()? {
			list.push(transform);
		}
		Ok(TransformList(list))
	}
}

impl<'a> ToCursors for TransformList<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for transform in &self.0 {
			ToCursors::to_cursors(transform, s);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		// assert_eq!(std::mem::size_of::<ColorFunction>(), 160);
	}

	#[test]
	fn test_writes() {
		// assert_parse!(TransformList, "rotate(45deg)");
		// assert_parse!(TransformList, "scale(2)");
		// assert_parse!(TransformList, "scaleX(1.5)");
		// assert_parse!(TransformList, "scaleY(0.5)");
		// assert_parse!(TransformList, "skew(10deg)");
		// assert_parse!(TransformList, "skewX(30deg)");
		// assert_parse!(TransformList, "skewY(-15deg)");
		// assert_parse!(TransformList, "matrix(1,0,0,1,0,0)");
		// assert_parse!(TransformList, "rotate(180deg)scale(2,3)");
		// assert_parse!(TransformList, "skewX(10deg)skewY(20deg)rotate(45deg)");
		// assert_parse!(TransformList, "scale(1.5)rotate(90deg)skew(15deg,30deg)");
		// assert_parse!(TransformList, "skew(1deg)rotate(45deg)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(TransformList, "rotate(45deg) auto");
		assert_parse_error!(TransformList, "auto rotate(45deg)");
	}
}
