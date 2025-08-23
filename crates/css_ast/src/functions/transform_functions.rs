use crate::{AngleOrZero, Length, LengthOrNone, LengthPercentage, NumberOrPercentage};
use css_parse::{Function, T, function_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

// https://drafts.csswg.org/css-transforms-1/#two-d-transform-functions
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[allow(clippy::large_enum_variant)] // TODO: matrix3d should probably be boxed
pub enum TransformFunction {
	Matrix(MatrixFunction),
	Matrix3d(Matrix3dFunction),
	Translate(TranslateFunction),
	Translate3d(Translate3dFunction),
	TranslateX(TranslatexFunction),
	TranslateY(TranslateyFunction),
	TranslateZ(TranslatezFunction),
	Scale(ScaleFunction),
	Scale3d(Scale3dFunction),
	ScaleX(ScalexFunction),
	ScaleY(ScaleyFunction),
	ScaleZ(ScalexFunction),
	Rotate(RotateFunction),
	Rotate3d(Rotate3dFunction),
	RotateX(RotatexFunction),
	RotateY(RotateyFunction),
	RotateZ(RotatezFunction),
	Skew(SkewFunction),
	SkewX(SkewxFunction),
	SkewY(SkewyFunction),
	Perspective(PerspectiveFunction),
}

function_set!(pub struct MatrixFunctionName "matrix");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-matrix>
///
/// ```text,ignore
/// matrix() = matrix( <number>#{6} )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct MatrixFunction(pub Function<MatrixFunctionName, MatrixFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MatrixFunctionParams(
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
);

function_set!(pub struct Matrix3dFunctionName "matrix3d");

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-matrix3d>
///
/// ```text,ignore
/// matrix3d() = matrix3d( <number>#{16} )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct Matrix3dFunction(pub Function<Matrix3dFunctionName, Matrix3dFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[allow(clippy::type_complexity)] // TODO: simplify types
pub struct Matrix3dFunctionParams(
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
);

function_set!(pub struct TranslateFunctionName "translate");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translate>
///
/// ```text,ignore
/// translate() = translate( <length-percentage> , <length-percentage>? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct TranslateFunction(
	pub Function<TranslateFunctionName, (LengthPercentage, Option<T![,]>, Option<LengthPercentage>)>,
);

function_set!(pub struct Translate3dFunctionName "translate3d");

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-translate3d>
///
/// ```text,ignore
/// translate3d() = translate3d( <length-percentage> , <length-percentage> , <length> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct Translate3dFunction(pub Function<Translate3dFunctionName, Translate3dFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Translate3dFunctionParams(
	pub LengthPercentage,
	pub Option<T![,]>,
	pub LengthPercentage,
	pub Option<T![,]>,
	pub Length,
);

function_set!(pub struct TranslatexFunctionName "translatex");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translatex>
///
/// ```text,ignore
/// translateX() = translateX( <length-percentage> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct TranslatexFunction(pub Function<TranslatexFunctionName, LengthPercentage>);

function_set!(pub struct TranslateyFunctionName "translatey");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translatey>
///
/// ```text,ignore
/// translateY() = translateY( <length-percentage> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct TranslateyFunction(pub Function<TranslateyFunctionName, LengthPercentage>);

function_set!(pub struct TranslatezFunctionName "translatez");

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-translatez>
///
/// ```text,ignore
/// translateZ() = translateZ( <length> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct TranslatezFunction(pub Function<TranslatezFunctionName, Length>);

function_set!(pub struct ScaleFunctionName "scale");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-scale>
///
/// ```text,ignore
/// scale() = scale( <number> , <number>? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ScaleFunction(pub Function<ScaleFunctionName, (NumberOrPercentage, Option<T![,]>, Option<T![Number]>)>);

function_set!(pub struct Scale3dFunctionName "scale3d");

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scale3d>
///
/// ```text,ignore
/// scale3d() = scale3d( [ <number> | <percentage> ]#{3} )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct Scale3dFunction(pub Function<Scale3dFunctionName, Scale3dFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Scale3dFunctionParams(
	pub NumberOrPercentage,
	pub Option<T![,]>,
	pub NumberOrPercentage,
	pub Option<T![,]>,
	pub NumberOrPercentage,
);

function_set!(pub struct ScalexFunctionName "scalex");

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scalex>
///
/// ```text,ignore
/// scaleX() = scaleX( <number> | <percentage> )
/// ````
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ScalexFunction(pub Function<ScalexFunctionName, NumberOrPercentage>);

function_set!(pub struct ScaleyFunctionName "scaley");

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scaley>
///
/// ```text,ignore
/// scaleY() = scaleY( <number> | <percentage> )
/// ````
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ScaleyFunction(pub Function<ScaleyFunctionName, NumberOrPercentage>);

function_set!(pub struct ScalezFunctionName "scalez");

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scalez>
///
/// ```text,ignore
/// scaleZ() = scaleZ( <number> | <percentage> )
/// ````
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ScalezFunction(pub Function<ScalezFunctionName, NumberOrPercentage>);

function_set!(pub struct RotateFunctionName "rotate");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-rotate>
///
/// ```text,ignore
/// rotate() = rotate( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RotateFunction(pub Function<RotateFunctionName, AngleOrZero>);

function_set!(pub struct Rotate3dFunctionName "rotate3d");

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-rotate3d>
///
/// ```text,ignore
/// rotate3d() = rotate3d( <number> , <number> , <number> , [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct Rotate3dFunction(pub Function<Rotate3dFunctionName, Rotate3dFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Rotate3dFunctionParams(
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub Option<AngleOrZero>,
);

function_set!(pub struct RotatexFunctionName "rotatex");

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-rotatex>
///
/// ```text,ignore
/// rotateX() = rotateX( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RotatexFunction(pub Function<RotatexFunctionName, AngleOrZero>);

function_set!(pub struct RotateyFunctionName "rotatey");

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-rotatey>
///
/// ```text,ignore
/// rotateY() = rotateY( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RotateyFunction(pub Function<RotateyFunctionName, AngleOrZero>);

function_set!(pub struct RotatezFunctionName "rotatez");

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-rotatez>
///
/// ```text,ignore
/// rotateZ() = rotateZ( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RotatezFunction(pub Function<RotatezFunctionName, AngleOrZero>);

function_set!(pub struct SkewFunctionName "skew");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skew>
///
/// ```text,ignore
/// skew() = skew( [ <angle> | <zero> ] , [ <angle> | <zero> ]? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct SkewFunction(pub Function<SkewFunctionName, (AngleOrZero, Option<T![,]>, Option<AngleOrZero>)>);

function_set!(pub struct SkewxFunctionName "skewx");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skewx>
///
/// ```text,ignore
/// skewX() = skewX( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct SkewxFunction(pub Function<SkewxFunctionName, AngleOrZero>);

function_set!(pub struct SkewyFunctionName "skewy");

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skewy>
///
/// ```text,ignore
/// skewY() = skewY( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct SkewyFunction(pub Function<SkewyFunctionName, AngleOrZero>);

function_set!(pub struct PerspectiveFunctionName "perspective");

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-perspective>
///
/// ```text,ignore
/// perspective() = perspective( [ <length [0,∞]> | none ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct PerspectiveFunction(pub Function<PerspectiveFunctionName, LengthOrNone>);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TransformFunction>(), 460);
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

		assert_parse!(TransformFunction, "scale3d(10%,10%,10%)");
		assert_parse!(TransformFunction, "rotate3d(1,2,3,10deg)");
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
