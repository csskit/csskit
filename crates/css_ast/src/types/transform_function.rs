#![allow(warnings)]

use crate::units::{Angle, LengthPercentage};
use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Build, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics, function_set};
use csskit_derives::{Parse, ToCursors};

#[derive(Parse, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
enum AngleZeroKind {
	Angle(Angle),
	Zero(T![Number]),
}

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
#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum TransformFunction {
	// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-matrix
	// matrix() = matrix( <number>#{6} )
	Matrix(T![Function], T![Number], Option<T![,]>, T![Number], Option<T![,]>, T![Number], Option<T![,]>, T![Number], Option<T![,]>, T![Number], Option<T![,]>, T![Number], Option<T![')']>),
	// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translate
	// translate() = translate( <length-percentage> , <length-percentage>? )
	Translate(T![Function], LengthPercentage, Option<T![,]>, Option<LengthPercentage>, Option<T![')']>),
	// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translatex
	// translateX() = translateX( <length-percentage> )
	TranslateX(T![Function], LengthPercentage, Option<T![')']>),
	// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translatey
	// translateY() = translateY( <length-percentage> )
	TranslateY(T![Function], LengthPercentage, Option<T![')']>),
	// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-scale
	// scale() = scale( <number> , <number>? )
	Scale(T![Function], T![Number], Option<T![,]>, Option<T![Number]>, Option<T![')']>),
	// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-scalex
	// scaleX() = scaleX( <number> )
	ScaleX(T![Function], T![Number], Option<T![')']>),
	// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-scaley
	// scaleX() = scaleX( <number> )
	ScaleY(T![Function], T![Number], Option<T![')']>),
	// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-rotate
	// rotate() = rotate( [ <angle> | <zero> ] )
	Rotate(T![Function], AngleZeroKind, Option<T![')']>),
	// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skew
	// skew() = skew( [ <angle> | <zero> ] , [ <angle> | <zero> ]? )
	Skew(T![Function], AngleZeroKind, Option<T![,]>, AngleZeroKind, Option<T![')']>),
	// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skewx
	// skewX() = skewX( [ <angle> | <zero> ] )
	SkewX(T![Function], AngleZeroKind, Option<T![')']>),
	// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skewy
	// skewY() = skewX( [ <angle> | <zero> ] )
	SkewY(T![Function], AngleZeroKind, Option<T![')']>),
}

impl<'a> Peek<'a> for TransformFunction {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		TransformFunctionName::peek(p, c)
	}
}

impl<'a> Parse<'a> for TransformFunction {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		match TransformFunctionName::parse(p)? {
			TransformFunctionName::Matrix(cursor) => Ok(Self::Matrix(<T![Function]>::build(p, cursor), p.parse::<T![Number]>()?, p.parse_if_peek::<T![,]>()?, p.parse::<T![Number]>()?, p.parse_if_peek::<T![,]>()?, p.parse::<T![Number]>()?, p.parse_if_peek::<T![,]>()?, p.parse::<T![Number]>()?, p.parse_if_peek::<T![,]>()?, p.parse::<T![Number]>()?, p.parse_if_peek::<T![,]>()?, p.parse::<T![Number]>()?, p.parse_if_peek::<T![')']>()?)),
			TransformFunctionName::Translate(cursor) => Ok(Self::Translate(<T![Function]>::build(p, cursor), p.parse::<LengthPercentage>()?, p.parse_if_peek::<T![,]>()?, p.parse_if_peek::<LengthPercentage>()?, p.parse_if_peek::<T![')']>()?)),
			TransformFunctionName::TranslateX(cursor) => Ok(Self::TranslateX(<T![Function]>::build(p, cursor), p.parse::<LengthPercentage>()?, p.parse_if_peek::<T![')']>()?)),
			TransformFunctionName::TranslateY(cursor) => Ok(Self::TranslateY(<T![Function]>::build(p, cursor), p.parse::<LengthPercentage>()?, p.parse_if_peek::<T![')']>()?)),
			TransformFunctionName::Scale(cursor) => Ok(Self::Scale(<T![Function]>::build(p, cursor), p.parse::<T![Number]>()?, p.parse_if_peek::<T![,]>()?, p.parse_if_peek::<T![Number]>()?, p.parse_if_peek::<T![')']>()?)),
			TransformFunctionName::ScaleY(cursor) => Ok(Self::ScaleY(<T![Function]>::build(p, cursor), p.parse::<T![Number]>()?, p.parse_if_peek::<T![')']>()?)),
			TransformFunctionName::ScaleX(cursor) => Ok(Self::ScaleX(<T![Function]>::build(p, cursor), p.parse::<T![Number]>()?, p.parse_if_peek::<T![')']>()?)),
			TransformFunctionName::Rotate(cursor) => Ok(Self::Rotate(<T![Function]>::build(p, cursor), p.parse::<AngleZeroKind>()?, p.parse_if_peek::<T![')']>()?)),
			TransformFunctionName::Skew(cursor) => Ok(Self::Skew(<T![Function]>::build(p, cursor), p.parse::<AngleZeroKind>()?, p.parse_if_peek::<T![,]>()?, p.parse::<AngleZeroKind>()?, p.parse_if_peek::<T![')']>()?)),
			TransformFunctionName::SkewX(cursor) => Ok(Self::SkewX(<T![Function]>::build(p, cursor), p.parse::<AngleZeroKind>()?, p.parse_if_peek::<T![')']>()?)),
			TransformFunctionName::SkewY(cursor) => Ok(Self::SkewY(<T![Function]>::build(p, cursor), p.parse::<AngleZeroKind>()?, p.parse_if_peek::<T![')']>()?)),
			_ => todo!()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		// assert_eq!(std::mem::size_of::<TransformFunction>(), 180);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TransformFunction, "matrix(1,0,0,1,0,0)");
		assert_parse!(TransformFunction, "translate(1rem)");
		assert_parse!(TransformFunction, "translate(1rem,2rem)");
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
