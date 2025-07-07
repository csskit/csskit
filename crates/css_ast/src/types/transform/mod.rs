use css_lexer::Cursor;
use css_parse::{Parser, Peek, function_set};
use crate::types;
use csskit_derives::{Parse, ToCursors};

function_set!(TransformFunctionName {
	Matrix: "matrix",
	Rotate: "rotate",
});

// https://drafts.csswg.org/css-transforms-1/#two-d-transform-functions
#[derive(Parse, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum TransformFunction {
	Matrix(types::Matrix),
	Rotate(types::Rotate),
}

impl<'a> Peek<'a> for TransformFunction {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		TransformFunctionName::peek(p, c)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		// assert_eq!(std::mem::size_of::<ColorFunction>(), 160);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TransformFunction, "rotate(45deg)");
		assert_parse!(TransformFunction, "matrix(1,0,0,1,0,0)");
	}
}
