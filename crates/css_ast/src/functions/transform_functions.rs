use crate::{AngleOrZero, LengthPercentage};
use css_parse::{Function, T, function_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

// https://drafts.csswg.org/css-transforms-1/#two-d-transform-functions
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum TransformFunction<'a> {
	Matrix(MatrixFunction<'a>),
	Translate(TranslateFunction<'a>),
	TranslateX(TranslatexFunction<'a>),
	TranslateY(TranslateyFunction<'a>),
	Scale(ScaleFunction<'a>),
	ScaleX(ScalexFunction<'a>),
	ScaleY(ScaleyFunction<'a>),
	Rotate(RotateFunction<'a>),
	Skew(SkewFunction<'a>),
	SkewX(SkewxFunction<'a>),
	SkewY(SkewyFunction<'a>),
}

function_set!(pub struct MatrixFunctionName "matrix");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-matrix>
///
/// ```text,ignore
/// matrix() = matrix( <number>#{6} )
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[allow(clippy::type_complexity)] // TODO: simplify types
pub struct MatrixFunction<'a>(
	Function<
		'a,
		MatrixFunctionName,
		(
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
		),
	>,
);

function_set!(pub struct TranslateFunctionName "translate");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translate>
///
/// ```text,ignore
/// translate() = translate( <length-percentage> , <length-percentage>? )
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TranslateFunction<'a>(
	Function<'a, TranslateFunctionName, (LengthPercentage, Option<T![,]>, Option<LengthPercentage>)>,
);

function_set!(pub struct TranslatexFunctionName "translatex");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translatex>
///
/// ```text,ignore
/// translateX() = translateX( <length-percentage> )
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TranslatexFunction<'a>(Function<'a, TranslatexFunctionName, LengthPercentage>);

function_set!(pub struct TranslateyFunctionName "translatey");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translatey>
///
/// ```text,ignore
/// translateY() = translateY( <length-percentage> )
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TranslateyFunction<'a>(Function<'a, TranslateyFunctionName, LengthPercentage>);

function_set!(pub struct ScaleFunctionName "scale");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-scale>
///
/// ```text,ignore
/// scale() = scale( <number> , <number>? )
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ScaleFunction<'a>(Function<'a, ScaleFunctionName, (T![Number], Option<T![,]>, Option<T![Number]>)>);

function_set!(pub struct ScalexFunctionName "scalex");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-scalex>
///
/// ```text,ignore
/// scaleX() = scaleX( <number> )
/// ````
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ScalexFunction<'a>(Function<'a, ScalexFunctionName, T![Number]>);

function_set!(pub struct ScaleyFunctionName "scaley");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-scaley>
///
/// ```text,ignore
/// scaleY() = scaleY( <number> )
/// ````
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ScaleyFunction<'a>(Function<'a, ScaleyFunctionName, T![Number]>);

function_set!(pub struct RotateFunctionName "rotate");

// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-rotate>
//
// ```text,ignore
// rotate() = rotate( [ <angle> | <zero> ] )
// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct RotateFunction<'a>(Function<'a, RotateFunctionName, AngleOrZero>);

function_set!(pub struct SkewFunctionName "skew");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skew>
///
/// ```text,ignore
/// skew() = skew( [ <angle> | <zero> ] , [ <angle> | <zero> ]? )
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SkewFunction<'a>(Function<'a, SkewFunctionName, (AngleOrZero, Option<T![,]>, Option<AngleOrZero>)>);

function_set!(pub struct SkewxFunctionName "skewx");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skewx>
///
/// ```text,ignore
/// skewX() = skewX( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SkewxFunction<'a>(Function<'a, SkewxFunctionName, AngleOrZero>);

function_set!(pub struct SkewyFunctionName "skewy");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skewy>
///
/// ```text,ignore
/// skewY() = skewY( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SkewyFunction<'a>(Function<'a, SkewyFunctionName, AngleOrZero>);

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
