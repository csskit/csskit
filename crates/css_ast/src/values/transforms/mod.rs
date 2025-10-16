#![allow(warnings)]
//! CSS Transforms Module Level 2
//! https://drafts.csswg.org/css-transforms-2/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `transform` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#transform).
///
/// The transform CSS property and its 2D transform functions allow rotating, scaling, skewing, and translating an element. Arbitrary 2D transforms are also possible using a transformation matrix.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <transform-list>
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#transform
#[syntax(" none | <transform-list> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "transformable elements",
	inherited = "no",
	percentages = "refer to the size of reference box",
	canonical_order = "per grammar",
	animation_type = "transform list, see interpolation rules"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.transform"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct TransformStyleValue<'a>;

// /// Represents the style value for `transform-origin` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#transform-origin).
// ///
// /// The transform CSS property and its 2D transform functions allow rotating, scaling, skewing, and translating an element. Arbitrary 2D transforms are also possible using a transformation matrix.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ left | center | right | top | bottom | <length-percentage> ] |   [ left | center | right | <length-percentage> ]  [ top | center | bottom | <length-percentage> ] <length>? |  [ [ center | left | right ] && [ center | top | bottom ] ] <length>?
// /// ```
// ///
// // https://drafts.csswg.org/css-transforms-2/#transform-origin
// #[syntax(
// 	" [ left | center | right | top | bottom | <length-percentage> ] |   [ left | center | right | <length-percentage> ]  [ top | center | bottom | <length-percentage> ] <length>? |  [ [ center | left | right ] && [ center | top | bottom ] ] <length>? "
// )]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "50% 50%",
//   applies_to = "transformable elements",
// 	inherited = "no",
// 	percentages = "refer to the size of reference box",
// 	canonical_order = "per grammar",
// 	animation_type = "by computed value",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.transform-origin"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum TransformOriginStyleValue {}

/// Represents the style value for `transform-box` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#transform-box).
///
/// The transform-box CSS property sets the position and dimensions of the reference box relative to which an element's transformations are calculated.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// content-box | border-box | fill-box | stroke-box | view-box
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#transform-box
#[syntax(" content-box | border-box | fill-box | stroke-box | view-box ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "view-box",
	applies_to = "transformable elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.transform-box"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum TransformBoxStyleValue {}

// /// Represents the style value for `translate` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#translate).
// ///
// /// The translate, rotate, and scale CSS properties apply single transformations independently, as opposed to applying multiple transformations with the transform CSS property.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | <length-percentage> [ <length-percentage> <length>? ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-transforms-2/#translate
// #[syntax(" none | <length-percentage> [ <length-percentage> <length>? ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "transformable elements",
// 	inherited = "no",
// 	percentages = "relative to the width of the reference box (for the first value) or the height (for the second value)",
// 	canonical_order = "per grammar",
// 	animation_type = "by computed value, but see below for none",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.translate"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct TranslateStyleValue;

// /// Represents the style value for `rotate` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#rotate).
// ///
// /// The translate, rotate, and scale CSS properties apply single transformations independently, as opposed to applying multiple transformations with the transform CSS property.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | <angle> | [ x | y | z | <number>{3} ] && <angle>
// /// ```
// ///
// // https://drafts.csswg.org/css-transforms-2/#rotate
// #[syntax(" none | <angle> | [ x | y | z | <number>{3} ] && <angle> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "transformable elements",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "as slerp, but see below for none",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.rotate"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum RotateStyleValue {}

/// Represents the style value for `scale` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#scale).
///
/// The translate, rotate, and scale CSS properties apply single transformations independently, as opposed to applying multiple transformations with the transform CSS property.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | [ <number> | <percentage> ]{1,3}
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#scale
#[syntax(" none | [ <number> | <percentage> ]{1,3} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "transformable elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value, but see below for none"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scale"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct ScaleStyleValue;

/// Represents the style value for `transform-style` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#transform-style).
///
/// The transform CSS property and its 3D transform functions allow rotations and other transforms in three dimensions, including perspective transforms.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// flat | preserve-3d
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#transform-style
#[syntax(" flat | preserve-3d ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "flat",
	applies_to = "transformable elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.transform-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum TransformStyleStyleValue {}

/// Represents the style value for `perspective` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#perspective).
///
/// The transform CSS property and its 3D transform functions allow rotations and other transforms in three dimensions, including perspective transforms.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#perspective
#[syntax(" none | <length [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "transformable elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.perspective"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct PerspectiveStyleValue;

/// Represents the style value for `perspective-origin` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#perspective-origin).
///
/// The transform CSS property and its 3D transform functions allow rotations and other transforms in three dimensions, including perspective transforms.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <position>
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#perspective-origin
#[syntax(" <position> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "50% 50%",
	applies_to = "transformable elements",
	inherited = "no",
	percentages = "refer to the size of the reference box",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.perspective-origin"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct PerspectiveOriginStyleValue;

/// Represents the style value for `backface-visibility` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#backface-visibility).
///
/// The transform CSS property and its 3D transform functions allow rotations and other transforms in three dimensions, including perspective transforms.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// visible | hidden
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#backface-visibility
#[syntax(" visible | hidden ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "visible",
	applies_to = "transformable elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.backface-visibility"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BackfaceVisibilityStyleValue {}
