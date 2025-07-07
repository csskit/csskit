mod matrix;
mod rotate;
mod scale;
mod scale_x;
mod scale_y;
mod skew;
mod skew_x;
mod skew_y;
mod translate;
mod translate_x;
mod translate_y;

use skew::SkewKind;

use css_lexer::Cursor;
use css_parse::{Parser, Peek, function_set};
use csskit_derives::{Parse, ToCursors};

function_set!(TransformFunctionName {
	Matrix: "matrix",
	Translate: "translate",
	TranslateX: "translatex",
	TranslateY: "translatey",
	Scale: "scale",
	ScaleX: "scalex",
	ScaleY: "scaley",
	Rotate: "rotate",
	Skew: "skew",
	SkewX: "skewx",
	SkewY: "skewy",
});

// https://drafts.csswg.org/css-transforms-1/#two-d-transform-functions
#[derive(Parse, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum TransformFunction {
	Matrix(matrix::Matrix),
	Translate(translate::Translate),
	TranslateX(translate_x::TranslateX),
	TranslateY(translate_y::TranslateY),
	Scale(scale::Scale),
	ScaleX(scale_x::ScaleX),
	ScaleY(scale_y::ScaleY),
	Rotate(rotate::Rotate),
	Skew(skew::Skew),
	SkewX(skew_x::SkewX),
	SkewY(skew_y::SkewY),
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
		assert_parse!(TransformFunction, "matrix(1,0,0,1,0,0)");
		assert_parse!(TransformFunction, "translate(1rem)");
		assert_parse!(TransformFunction, "translateX(1rem)");
		assert_parse!(TransformFunction, "translateY(1rem)");
		assert_parse!(TransformFunction, "scale(2)");
		assert_parse!(TransformFunction, "scale(1,2)");
		assert_parse!(TransformFunction, "scaleX(2)");
		assert_parse!(TransformFunction, "scaleY(2)");
		assert_parse!(TransformFunction, "rotate(45deg)");
		assert_parse!(TransformFunction, "skew(1deg,2deg)");
		assert_parse!(TransformFunction, "skewX(1deg)");
		assert_parse!(TransformFunction, "skewY(1deg)");
	}
}
