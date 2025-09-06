use crate::{AngleOrNumber, NoneOr, NumberOrPercentage};
use css_parse::{Function, T, function_set, keyword_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

keyword_set!(pub enum ColorSpace {
	Srgb: "srgb",
	SrgbLinear: "srgb-linear",
	DisplayP3: "display-p3",
	A98Rgb: "a98-rgb",
	ProphotoRgb: "prophoto-rgb",
	Rec2020: "rec2020",
	Xyz: "xyz",
	XyzD50: "xyz-d50",
	XyzD65: "xyz-d65",
});

function_set!(pub struct ColorFunctionName "color");
function_set!(pub struct RgbFunctionName "rgb");
function_set!(pub struct RgbaFunctionName "rgba");
function_set!(pub struct HslFunctionName "hsl");
function_set!(pub struct HslaFunctionName "hsla");
function_set!(pub struct HwbFunctionName "hwb");
function_set!(pub struct LabFunctionName "lab");
function_set!(pub struct LchFunctionName "lch");
function_set!(pub struct OklabFunctionName "oklab");
function_set!(pub struct OklchFunctionName "oklch");

/// <https://drafts.csswg.org/css-color/#typedef-color-function>
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum ColorFunction {
	Color(ColorFunctionColor),
	Rgb(RgbFunction),
	Rgba(RgbaFunction),
	Hsl(HslFunction),
	Hsla(HslaFunction),
	Hwb(HwbFunction),
	Lab(LabFunction),
	Lch(LchFunction),
	Oklab(OklabFunction),
	Oklch(OklchFunction),
}

/// <https://drafts.csswg.org/css-color/#funcdef-color>
///
/// ```text,ignore
/// color() = color( <colorspace-params> [ / [ <alpha-value> | none ] ]? )
/// <colorspace-params> = [ <predefined-rgb-params> | <xyz-params>]
/// <predefined-rgb-params> = <predefined-rgb> [ <number> | <percentage> | none ]{3}
/// <predefined-rgb> = srgb | srgb-linear | display-p3 | a98-rgb | prophoto-rgb | rec2020
/// <xyz-params> = <xyz-space> [ <number> | <percentage> | none ]{3}
/// <xyz-space> = xyz | xyz-d50 | xyz-d65
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ColorFunctionColor(Function<ColorFunctionName, ColorFunctionColorParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ColorFunctionColorParams(
	pub ColorSpace,
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![/]>,
	pub Option<NoneOr<NumberOrPercentage>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-rgb>
///
/// ```text,ignore
/// rgb() = [ <legacy-rgb-syntax> | <modern-rgb-syntax> ]
/// rgba() = [ <legacy-rgba-syntax> | <modern-rgba-syntax> ]
/// <legacy-rgb-syntax> =   rgb( <percentage>#{3} , <alpha-value>? ) |
///                   rgb( <number>#{3} , <alpha-value>? )
/// <legacy-rgba-syntax> = rgba( <percentage>#{3} , <alpha-value>? ) |
///                   rgba( <number>#{3} , <alpha-value>? )
/// <modern-rgb-syntax> = rgb(
///   [ <number> | <percentage> | none]{3}
///   [ / [<alpha-value> | none] ]?  )
/// <modern-rgba-syntax> = rgba(
///   [ <number> | <percentage> | none]{3}
///   [ / [<alpha-value> | none] ]?  )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RgbFunction(Function<RgbFunctionName, RgbFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RgbaFunction(Function<RgbaFunctionName, RgbFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RgbFunctionParams(
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![,]>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![,]>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![,]>,
	pub Option<T![/]>,
	pub Option<NoneOr<NumberOrPercentage>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-hsl>
///
/// ```text,ignore
/// hsl() = [ <legacy-hsl-syntax> | <modern-hsl-syntax> ]
/// hsla() = [ <legacy-hsla-syntax> | <modern-hsla-syntax> ]
/// <modern-hsl-syntax> = hsl(
///     [<hue> | none]
///     [<percentage> | <number> | none]
///     [<percentage> | <number> | none]
///     [ / [<alpha-value> | none] ]? )
/// <modern-hsla-syntax> = hsla(
///     [<hue> | none]
///     [<percentage> | <number> | none]
///     [<percentage> | <number> | none]
///     [ / [<alpha-value> | none] ]? )
/// <legacy-hsl-syntax> = hsl( <hue>, <percentage>, <percentage>, <alpha-value>? )
/// <legacy-hsla-syntax> = hsla( <hue>, <percentage>, <percentage>, <alpha-value>? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct HslFunction(Function<HslFunctionName, HslFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct HslaFunction(Function<HslaFunctionName, HslFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct HslFunctionParams(
	pub NoneOr<AngleOrNumber>,
	pub Option<T![,]>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![,]>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![,]>,
	pub Option<T![/]>,
	pub Option<NoneOr<NumberOrPercentage>>,
);

// https://drafts.csswg.org/css-color/#funcdef-hwb
// hwb() = hwb(
//  [<hue> | none]
//  [<percentage> | <number> | none]
//  [<percentage> | <number> | none]
//  [ / [<alpha-value> | none] ]? )
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct HwbFunction(Function<HwbFunctionName, HwbFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct HwbFunctionParams(
	pub NoneOr<AngleOrNumber>,
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![/]>,
	pub Option<NoneOr<NumberOrPercentage>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-lab>
///
/// ```text,ignore
/// lab() = lab( [<percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ / [<alpha-value> | none] ]? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LabFunction(Function<LabFunctionName, LabFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LabFunctionParams(
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![/]>,
	pub Option<NoneOr<NumberOrPercentage>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-lch>
///
/// ```text,ignore
/// lch() = lch( [<percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ <hue> | none]
///  [ / [<alpha-value> | none] ]? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LchFunction(Function<LchFunctionName, LchFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LchFunctionParams(
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<AngleOrNumber>,
	pub Option<T![/]>,
	pub Option<NoneOr<NumberOrPercentage>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-oklab>
///
/// ```text,ignore
/// oklab() = oklab( [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ / [<alpha-value> | none] ]? )
///  ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct OklabFunction(Function<OklabFunctionName, LabFunctionParams>);

/// <https://drafts.csswg.org/css-color/#funcdef-oklch>
///
/// ```text,ignore
/// oklab() = oklab( [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ / [<alpha-value> | none] ]? )
///  ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct OklchFunction(Function<OklchFunctionName, LchFunctionParams>);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorFunction>(), 160);
		assert_eq!(std::mem::size_of::<ColorFunctionColor>(), 124);
		assert_eq!(std::mem::size_of::<RgbFunction>(), 156);
		assert_eq!(std::mem::size_of::<RgbaFunction>(), 156);
		assert_eq!(std::mem::size_of::<HslFunction>(), 156);
		assert_eq!(std::mem::size_of::<HslaFunction>(), 156);
		assert_eq!(std::mem::size_of::<HwbFunction>(), 108);
		assert_eq!(std::mem::size_of::<LabFunction>(), 108);
		assert_eq!(std::mem::size_of::<LchFunction>(), 108);
		assert_eq!(std::mem::size_of::<OklabFunction>(), 108);
		assert_eq!(std::mem::size_of::<OklchFunction>(), 108);
	}
}
