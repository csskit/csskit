#![allow(warnings)]

use crate::units::{Angle, LengthPercentage};
use css_lexer::Cursor;
use css_parse::{
	Build, CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics, function_set,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

#[derive(Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum TransformFunction {
	// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-matrix
	// matrix() = matrix( <number>#{6} )
	Matrix(
		T![Function],
		T![Number],
		Option<T![,]>,
		T![Number],
		Option<T![,]>,
		T![Number],
		Option<T![,]>,
		T![Number],
		Option<T![,]>,
		T![Number],
		Option<T![,]>,
		T![Number],
		Option<T![')']>,
	),
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
	Skew(T![Function], AngleZeroKind, Option<T![,]>, Option<AngleZeroKind>, Option<T![')']>),
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
			TransformFunctionName::Matrix(cursor) => Ok(Self::Matrix(
				<T![Function]>::build(p, cursor),
				p.parse::<T![Number]>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse::<T![Number]>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse::<T![Number]>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse::<T![Number]>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse::<T![Number]>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse::<T![Number]>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			TransformFunctionName::Translate(cursor) => Ok(Self::Translate(
				<T![Function]>::build(p, cursor),
				p.parse::<LengthPercentage>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse_if_peek::<LengthPercentage>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			TransformFunctionName::TranslateX(cursor) => Ok(Self::TranslateX(
				<T![Function]>::build(p, cursor),
				p.parse::<LengthPercentage>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			TransformFunctionName::TranslateY(cursor) => Ok(Self::TranslateY(
				<T![Function]>::build(p, cursor),
				p.parse::<LengthPercentage>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			TransformFunctionName::Scale(cursor) => Ok(Self::Scale(
				<T![Function]>::build(p, cursor),
				p.parse::<T![Number]>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse_if_peek::<T![Number]>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			TransformFunctionName::ScaleY(cursor) => Ok(Self::ScaleY(
				<T![Function]>::build(p, cursor),
				p.parse::<T![Number]>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			TransformFunctionName::ScaleX(cursor) => Ok(Self::ScaleX(
				<T![Function]>::build(p, cursor),
				p.parse::<T![Number]>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			TransformFunctionName::Rotate(cursor) => Ok(Self::Rotate(
				<T![Function]>::build(p, cursor),
				p.parse::<AngleZeroKind>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			TransformFunctionName::Skew(cursor) => Ok(Self::Skew(
				<T![Function]>::build(p, cursor),
				p.parse::<AngleZeroKind>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse_if_peek::<AngleZeroKind>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			TransformFunctionName::SkewX(cursor) => Ok(Self::SkewX(
				<T![Function]>::build(p, cursor),
				p.parse::<AngleZeroKind>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			TransformFunctionName::SkewY(cursor) => Ok(Self::SkewY(
				<T![Function]>::build(p, cursor),
				p.parse::<AngleZeroKind>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TransformFunction>(), 180);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TransformFunction, "matrix(1,2,3,4,5,6)");
		assert_parse!(TransformFunction, "matrix(1 2 3 4 5 6)");
		assert_parse!(TransformFunction, "matrix(0,0,0,0,0,0)");
		assert_parse!(TransformFunction, "matrix(-1,-2,-3,-4,-5,-6)");
		assert_parse!(TransformFunction, "matrix(1.5,2.5,3.5,4.5,5.5,6.5)");

		assert_parse!(TransformFunction, "translate(10px)");
		assert_parse!(TransformFunction, "translate(10px,20px)");
		assert_parse!(TransformFunction, "translate(45%)");
		assert_parse!(TransformFunction, "translate(2rem)");
		assert_parse!(TransformFunction, "translateX(1rem)");
		assert_parse!(TransformFunction, "translateY(1rem)");

		assert_parse!(TransformFunction, "scale(1,2)");
		assert_parse!(TransformFunction, "scale(0,0)");
		assert_parse!(TransformFunction, "scale(1)");
		assert_parse!(TransformFunction, "scale(1.5,2.5)");
		assert_parse!(TransformFunction, "scaleX(2)");
		assert_parse!(TransformFunction, "scaleY(2)");

		assert_parse!(TransformFunction, "rotate(45deg)");
		assert_parse!(TransformFunction, "rotate(0)");
		assert_parse!(TransformFunction, "rotate(2turn)");
		assert_parse!(TransformFunction, "rotate(20rad");

		assert_parse!(TransformFunction, "skew(1deg,2deg)");
		assert_parse!(TransformFunction, "skew(0,0)");
		assert_parse!(TransformFunction, "skew(1deg)");
		assert_parse!(TransformFunction, "skewX(1deg)");
		assert_parse!(TransformFunction, "skewX(0)");
		assert_parse!(TransformFunction, "skewY(1deg)");
		assert_parse!(TransformFunction, "skewY(0)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(TransformFunction, "matrix()");
		assert_parse_error!(TransformFunction, "matrix(1)");
		assert_parse_error!(TransformFunction, "matrix(1,2)");
		assert_parse_error!(TransformFunction, "matrix(one,two,three,four,five,size)");

		assert_parse_error!(TransformFunction, "translate()");
		assert_parse_error!(TransformFunction, "translate(foo)");
		assert_parse_error!(TransformFunction, "translateX()");
		assert_parse_error!(TransformFunction, "translateX(foo)");
		assert_parse_error!(TransformFunction, "translateY()");
		assert_parse_error!(TransformFunction, "translateY(foo)");

		assert_parse_error!(TransformFunction, "scale()");
		assert_parse_error!(TransformFunction, "scale(foo)");
		assert_parse_error!(TransformFunction, "scaleX()");
		assert_parse_error!(TransformFunction, "scaleX(foo)");
		assert_parse_error!(TransformFunction, "scaleY()");
		assert_parse_error!(TransformFunction, "scaleY(foo)");

		assert_parse_error!(TransformFunction, "rotate()");
		assert_parse_error!(TransformFunction, "rotate(45px)");
		assert_parse_error!(TransformFunction, "rotate(all the way around)");

		assert_parse_error!(TransformFunction, "skew()");
		assert_parse_error!(TransformFunction, "skew(foo)");
		assert_parse_error!(TransformFunction, "skewX()");
		assert_parse_error!(TransformFunction, "skewX(foo)");
		assert_parse_error!(TransformFunction, "skewY()");
		assert_parse_error!(TransformFunction, "skewY(foo)");
	}
}
