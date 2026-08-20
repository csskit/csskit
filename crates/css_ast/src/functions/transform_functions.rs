use super::prelude::*;
use crate::{AngleOrZero, CalcableValue, Length, LengthPercentage, NoneOr, NumberOrPercentage, NumericValue};
use css_parse::Box;

/// <https://drafts.csswg.org/css-transforms-1/#two-d-transform-functions>
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum TransformFunction<'a> {
	Matrix(Box<'a, MatrixFunction<'a>>),
	Matrix3d(Box<'a, Matrix3dFunction<'a>>),
	Translate(TranslateFunction<'a>),
	Translate3d(Translate3dFunction<'a>),
	TranslateX(TranslatexFunction<'a>),
	TranslateY(TranslateyFunction<'a>),
	TranslateZ(TranslatezFunction<'a>),
	Scale(ScaleFunction<'a>),
	Scale3d(Scale3dFunction<'a>),
	ScaleX(ScalexFunction<'a>),
	ScaleY(ScaleyFunction<'a>),
	ScaleZ(ScalezFunction<'a>),
	Rotate(RotateFunction<'a>),
	Rotate3d(Rotate3dFunction<'a>),
	RotateX(RotatexFunction<'a>),
	RotateY(RotateyFunction<'a>),
	RotateZ(RotatezFunction<'a>),
	Skew(SkewFunction<'a>),
	SkewX(SkewxFunction<'a>),
	SkewY(SkewyFunction<'a>),
	Perspective(PerspectiveFunction<'a>),
}

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-matrix>
///
/// ```text,ignore
/// matrix() = matrix( <number>#{6} )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MatrixFunction<'a> {
	#[atom(CssAtomSet::Matrix)]
	pub name: T![Function],
	pub params: MatrixFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MatrixFunctionParams<'a>(
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
);

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-matrix3d>
///
/// ```text,ignore
/// matrix3d() = matrix3d( <number>#{16} )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Matrix3dFunction<'a> {
	#[atom(CssAtomSet::Matrix3d)]
	pub name: T![Function],
	pub params: Matrix3dFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[allow(clippy::type_complexity)] // TODO: simplify types
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Matrix3dFunctionParams<'a>(
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
);

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translate>
///
/// ```text,ignore
/// translate() = translate( <length-percentage> , <length-percentage>? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TranslateFunction<'a> {
	#[atom(CssAtomSet::Translate)]
	pub name: T![Function],
	pub x: CalcableValue<'a, LengthPercentage>,
	#[semantic_eq(skip)]
	pub comma: Option<T![,]>,
	pub y: Option<CalcableValue<'a, LengthPercentage>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-translate3d>
///
/// ```text,ignore
/// translate3d() = translate3d( <length-percentage> , <length-percentage> , <length> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Translate3dFunction<'a> {
	#[atom(CssAtomSet::Translate3d)]
	pub name: T![Function],
	pub params: Translate3dFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Translate3dFunctionParams<'a>(
	pub CalcableValue<'a, LengthPercentage>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub CalcableValue<'a, LengthPercentage>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub CalcableValue<'a, Length>,
);

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translatex>
///
/// ```text,ignore
/// translateX() = translateX( <length-percentage> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TranslatexFunction<'a> {
	#[atom(CssAtomSet::Translatex)]
	pub name: T![Function],
	pub params: CalcableValue<'a, LengthPercentage>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translatey>
///
/// ```text,ignore
/// translateY() = translateY( <length-percentage> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TranslateyFunction<'a> {
	#[atom(CssAtomSet::Translatey)]
	pub name: T![Function],
	pub params: CalcableValue<'a, LengthPercentage>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-translatez>
///
/// ```text,ignore
/// translateZ() = translateZ( <length> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TranslatezFunction<'a> {
	#[atom(CssAtomSet::Translatez)]
	pub name: T![Function],
	pub params: CalcableValue<'a, Length>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scale>
///
/// ```text,ignore
/// scale() = scale( [ <number> | <percentage> ]#{1,2} )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ScaleFunction<'a> {
	#[atom(CssAtomSet::Scale)]
	pub name: T![Function],
	#[semantic_eq(skip)]
	pub params: (NumericValue<'a, NumberOrPercentage>, Option<T![,]>, Option<NumericValue<'a, NumberOrPercentage>>),
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scale3d>
///
/// ```text,ignore
/// scale3d() = scale3d( [ <number> | <percentage> ]#{3} )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Scale3dFunction<'a> {
	#[atom(CssAtomSet::Scale3d)]
	pub name: T![Function],
	pub params: Scale3dFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Scale3dFunctionParams<'a>(
	pub NumericValue<'a, NumberOrPercentage>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, NumberOrPercentage>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, NumberOrPercentage>,
);

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scalex>
///
/// ```text,ignore
/// scaleX() = scaleX( <number> | <percentage> )
/// ````
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ScalexFunction<'a> {
	#[atom(CssAtomSet::Scalex)]
	pub name: T![Function],
	pub params: NumericValue<'a, NumberOrPercentage>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scaley>
///
/// ```text,ignore
/// scaleY() = scaleY( <number> | <percentage> )
/// ````
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ScaleyFunction<'a> {
	#[atom(CssAtomSet::Scaley)]
	pub name: T![Function],
	pub params: NumericValue<'a, NumberOrPercentage>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-scalez>
///
/// ```text,ignore
/// scaleZ() = scaleZ( <number> | <percentage> )
/// ````
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ScalezFunction<'a> {
	#[atom(CssAtomSet::Scalez)]
	pub name: T![Function],
	pub params: NumericValue<'a, NumberOrPercentage>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-rotate>
///
/// ```text,ignore
/// rotate() = rotate( [ <angle> | <zero> ] )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RotateFunction<'a> {
	#[atom(CssAtomSet::Rotate)]
	pub name: T![Function],
	pub params: CalcableValue<'a, AngleOrZero>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-rotate3d>
///
/// ```text,ignore
/// rotate3d() = rotate3d( <number> , <number> , <number> , [ <angle> | <zero> ] )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Rotate3dFunction<'a> {
	#[atom(CssAtomSet::Rotate3d)]
	pub name: T![Function],
	pub params: Rotate3dFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Rotate3dFunctionParams<'a>(
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub CalcableValue<'a, AngleOrZero>,
);

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-rotatex>
///
/// ```text,ignore
/// rotateX() = rotateX( [ <angle> | <zero> ] )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RotatexFunction<'a> {
	#[atom(CssAtomSet::Rotatex)]
	pub name: T![Function],
	pub params: CalcableValue<'a, AngleOrZero>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-rotatey>
///
/// ```text,ignore
/// rotateY() = rotateY( [ <angle> | <zero> ] )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RotateyFunction<'a> {
	#[atom(CssAtomSet::Rotatey)]
	pub name: T![Function],
	pub params: CalcableValue<'a, AngleOrZero>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-rotatez>
///
/// ```text,ignore
/// rotateZ() = rotateZ( [ <angle> | <zero> ] )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RotatezFunction<'a> {
	#[atom(CssAtomSet::Rotatez)]
	pub name: T![Function],
	pub params: CalcableValue<'a, AngleOrZero>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skew>
///
/// ```text,ignore
/// skew() = skew( [ <angle> | <zero> ] , [ <angle> | <zero> ]? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SkewFunction<'a> {
	#[atom(CssAtomSet::Skew)]
	pub name: T![Function],
	#[semantic_eq(skip)]
	pub params: (CalcableValue<'a, AngleOrZero>, Option<T![,]>, Option<CalcableValue<'a, AngleOrZero>>),
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skewx>
///
/// ```text,ignore
/// skewX() = skewX( [ <angle> | <zero> ] )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SkewxFunction<'a> {
	#[atom(CssAtomSet::Skewx)]
	pub name: T![Function],
	pub params: CalcableValue<'a, AngleOrZero>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skewy>
///
/// ```text,ignore
/// skewY() = skewY( [ <angle> | <zero> ] )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SkewyFunction<'a> {
	#[atom(CssAtomSet::Skewy)]
	pub name: T![Function],
	pub params: CalcableValue<'a, AngleOrZero>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-transforms-2/#funcdef-perspective>
///
/// ```text,ignore
/// perspective() = perspective( [ <length [0,∞]> | none ] )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PerspectiveFunction<'a> {
	#[atom(CssAtomSet::Perspective)]
	pub name: T![Function],
	pub params: NoneOr<CalcableValue<'a, Length>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_parse_span};

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
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "matrix3d(1,0,0,0,0,1,0,0,0,0,1,0,10,20,30,1)");
	}

	#[test]
	fn test_substitution() {
		// Substitution/math functions permitted in <length[-percentage]> slots.
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "translate(calc(10px + 5%))");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "translate(var(--x),var(--y))");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "translateX(calc(1rem * 2))");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "translateZ(var(--z))");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "perspective(calc(100px))");
		// Substitution/math in number/angle/scale slots.
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "rotate(calc(45deg))");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "rotate(var(--a))");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "scale(calc(1 + 1),var(--y))");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "skewX(calc(10deg))");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "matrix(calc(1),2,3,4,5,6)");
		assert_parse!(CssAtomSet::ATOMS, TransformFunction, "rotate3d(1,2,3,var(--a))");
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
