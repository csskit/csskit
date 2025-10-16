use super::prelude::*;
use crate::{AngleOrZero, Length, LengthPercentage, NoneOr, NumberOrPercentage};

// https://drafts.csswg.org/css-transforms-1/#two-d-transform-functions
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
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
	ScaleZ(ScalezFunction),
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

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-matrix>
///
/// ```text,ignore
/// matrix() = matrix( <number>#{6} )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct MatrixFunction {
	#[atom(CssAtomSet::Matrix)]
	pub name: T![Function],
	pub params: MatrixFunctionParams,
	pub close: T![')'],
}

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

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-matrix3d>
///
/// ```text,ignore
/// matrix3d() = matrix3d( <number>#{16} )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct Matrix3dFunction {
	#[atom(CssAtomSet::Matrix3d)]
	pub name: T![Function],
	pub params: Matrix3dFunctionParams,
	pub close: T![')'],
}

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

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translate>
///
/// ```text,ignore
/// translate() = translate( <length-percentage> , <length-percentage>? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct TranslateFunction {
	#[atom(CssAtomSet::Translate)]
	pub name: T![Function],
	pub x: LengthPercentage,
	pub comma: Option<T![,]>,
	pub y: Option<LengthPercentage>,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-translate3d>
///
/// ```text,ignore
/// translate3d() = translate3d( <length-percentage> , <length-percentage> , <length> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct Translate3dFunction {
	#[atom(CssAtomSet::Translate3d)]
	pub name: T![Function],
	pub params: Translate3dFunctionParams,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Translate3dFunctionParams(
	pub LengthPercentage,
	pub Option<T![,]>,
	pub LengthPercentage,
	pub Option<T![,]>,
	pub Length,
);

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translatex>
///
/// ```text,ignore
/// translateX() = translateX( <length-percentage> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct TranslatexFunction {
	#[atom(CssAtomSet::Translatex)]
	pub name: T![Function],
	pub params: LengthPercentage,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translatey>
///
/// ```text,ignore
/// translateY() = translateY( <length-percentage> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct TranslateyFunction {
	#[atom(CssAtomSet::Translatey)]
	pub name: T![Function],
	pub params: LengthPercentage,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-translatez>
///
/// ```text,ignore
/// translateZ() = translateZ( <length> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct TranslatezFunction {
	#[atom(CssAtomSet::Translatez)]
	pub name: T![Function],
	pub params: Length,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scale>
///
/// ```text,ignore
/// scale() = scale( [ <number> | <percentage> ]#{1,2} )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct ScaleFunction {
	#[atom(CssAtomSet::Scale)]
	pub name: T![Function],
	pub params: (NumberOrPercentage, Option<T![,]>, Option<NumberOrPercentage>),
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scale3d>
///
/// ```text,ignore
/// scale3d() = scale3d( [ <number> | <percentage> ]#{3} )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct Scale3dFunction {
	#[atom(CssAtomSet::Scale3d)]
	pub name: T![Function],
	pub params: Scale3dFunctionParams,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Scale3dFunctionParams(
	pub NumberOrPercentage,
	pub Option<T![,]>,
	pub NumberOrPercentage,
	pub Option<T![,]>,
	pub NumberOrPercentage,
);

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scalex>
///
/// ```text,ignore
/// scaleX() = scaleX( <number> | <percentage> )
/// ````
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct ScalexFunction {
	#[atom(CssAtomSet::Scalex)]
	pub name: T![Function],
	pub params: NumberOrPercentage,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scaley>
///
/// ```text,ignore
/// scaleY() = scaleY( <number> | <percentage> )
/// ````
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct ScaleyFunction {
	#[atom(CssAtomSet::Scaley)]
	pub name: T![Function],
	pub params: NumberOrPercentage,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scalez>
///
/// ```text,ignore
/// scaleZ() = scaleZ( <number> | <percentage> )
/// ````
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct ScalezFunction {
	#[atom(CssAtomSet::Scalez)]
	pub name: T![Function],
	pub params: NumberOrPercentage,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-rotate>
///
/// ```text,ignore
/// rotate() = rotate( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct RotateFunction {
	#[atom(CssAtomSet::Rotate)]
	pub name: T![Function],
	pub params: AngleOrZero,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-rotate3d>
///
/// ```text,ignore
/// rotate3d() = rotate3d( <number> , <number> , <number> , [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct Rotate3dFunction {
	#[atom(CssAtomSet::Rotate3d)]
	pub name: T![Function],
	pub params: Rotate3dFunctionParams,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Rotate3dFunctionParams(
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub T![Number],
	pub Option<T![,]>,
	pub AngleOrZero,
);

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-rotatex>
///
/// ```text,ignore
/// rotateX() = rotateX( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct RotatexFunction {
	#[atom(CssAtomSet::Rotatex)]
	pub name: T![Function],
	pub params: AngleOrZero,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-rotatey>
///
/// ```text,ignore
/// rotateY() = rotateY( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct RotateyFunction {
	#[atom(CssAtomSet::Rotatey)]
	pub name: T![Function],
	pub params: AngleOrZero,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-rotatez>
///
/// ```text,ignore
/// rotateZ() = rotateZ( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct RotatezFunction {
	#[atom(CssAtomSet::Rotatez)]
	pub name: T![Function],
	pub params: AngleOrZero,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skew>
///
/// ```text,ignore
/// skew() = skew( [ <angle> | <zero> ] , [ <angle> | <zero> ]? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct SkewFunction {
	#[atom(CssAtomSet::Skew)]
	pub name: T![Function],
	pub params: (AngleOrZero, Option<T![,]>, Option<AngleOrZero>),
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skewx>
///
/// ```text,ignore
/// skewX() = skewX( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct SkewxFunction {
	#[atom(CssAtomSet::Skewx)]
	pub name: T![Function],
	pub params: AngleOrZero,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skewy>
///
/// ```text,ignore
/// skewY() = skewY( [ <angle> | <zero> ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct SkewyFunction {
	#[atom(CssAtomSet::Skewy)]
	pub name: T![Function],
	pub params: AngleOrZero,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-perspective>
///
/// ```text,ignore
/// perspective() = perspective( [ <length [0,∞]> | none ] )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct PerspectiveFunction {
	#[atom(CssAtomSet::Perspective)]
	pub name: T![Function],
	pub params: NoneOr<Length>,
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_parse_span};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TransformFunction>(), 456);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "matrix(1,2,3,4,5,6)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "matrix(1 2 3 4 5 6)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "matrix(0,0,0,0,0,0)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "matrix(-1,-2,-3,-4,-5,-6)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "matrix(1.5,2.5,3.5,4.5,5.5,6.5)");

		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "translate(10px)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "translate(10px,20px)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "translate(45%)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "translate(2rem)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "translateX(1rem)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "translateY(1rem)");

		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "scale(1,2)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "scale(0,0)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "scale(1)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "scale(1.5,2.5)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "scaleX(2)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "scaleY(2)");

		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "rotate(45deg)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "rotate(0)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "rotate(2turn)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "rotate(20rad)");

		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "skew(1deg,2deg)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "skew(0,0)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "skew(1deg)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "skewX(1deg)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "skewX(0)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "skewY(1deg)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "skewY(0)");

		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "scale3d(10%,10%,10%)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "rotate3d(1,2,3,10deg)");
	}

	#[test]
	fn test_span() {
		assert_parse_span!(
			CssAtomSet::ATOMS,
			TransformFunction,
			r#"
				matrix(1,2,3,4,5,6) translate(0)
				^^^^^^^^^^^^^^^^^^^
		"#
		);
		assert_parse_span!(
			CssAtomSet::ATOMS,
			TransformFunction,
			r#"
				translate(0) foo
				^^^^^^^^^^^^
		"#
		);
		assert_parse_span!(
			CssAtomSet::ATOMS,
			TranslateFunction,
			r#"
				translate(0) bar
				^^^^^^^^^^^^
		"#
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "matrix()");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "matrix(1)");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "matrix(1,2)");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "matrix(one,two,three,four,five,size)");

		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "translate()");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "translate(foo)");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "translateX()");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "translateX(foo)");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "translateY()");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "translateY(foo)");

		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "scale()");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "scale(foo)");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "scaleX()");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "scaleX(foo)");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "scaleY()");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "scaleY(foo)");

		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "rotate()");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "rotate(45px)");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "rotate(all the way around)");

		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "skew()");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "skew(foo)");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "skewX()");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "skewX(foo)");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "skewY()");
		assert_parse_error!(CssAtomSet::ATOMS, TransformFunction, "skewY(foo)");
	}
}
