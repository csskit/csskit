#![allow(warnings)]
//! CSS Backgrounds Module Level 4
//! https://drafts.csswg.org/css-backgrounds-4/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `background-color` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-color).
///
/// The background-color CSS property sets the fill color of an element, behind any content and background images or gradients.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color>
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-color
#[syntax(" <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "transparent",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-color"))]
#[visit]
pub struct BackgroundColorStyleValue;

/// Represents the style value for `background-image` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-image).
///
/// The background-image CSS property sets the graphics to display behind the content of an element and in front of the background color. Graphics may be any combination of images or gradients.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <bg-image>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-image
#[syntax(" <bg-image># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-image"))]
#[visit]
pub struct BackgroundImageStyleValue<'a>;

/// Represents the style value for `background-repeat` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-repeat).
///
/// The background-repeat CSS property sets how a background image is tiled.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <repeat-style>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-repeat
#[syntax(" <repeat-style># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "repeat",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-repeat"))]
#[visit]
pub struct BackgroundRepeatStyleValue<'a>;

/// Represents the style value for `background-attachment` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-attachment).
///
/// The background-attachment CSS property sets whether an element's background image or gradient moves as the element scrolls.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <attachment>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-attachment
#[syntax(" <attachment># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "scroll",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-attachment"))]
#[visit]
pub struct BackgroundAttachmentStyleValue<'a>;

// /// Represents the style value for `background-position` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-position).
// ///
// /// The background-position CSS property offsets the initial position of background images relative to the background origin.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <bg-position>#
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#background-position
// #[syntax(" <bg-position># ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0% 0%",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "refer to size of background positioning area minus size of background image; see text",
// 	canonical_order = "per grammar",
// 	animation_type = "repeatable list",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-position"))]
// #[visit]
// pub struct BackgroundPositionStyleValue<'a>;

/// Represents the style value for `background-clip` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-clip).
///
/// The background-clip CSS property sets the extent of the background: the padding box, the content box, or the default border box.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <bg-clip>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-clip
#[syntax(" <bg-clip># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "border-box",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "repeatable list"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-clip"))]
#[visit]
pub struct BackgroundClipStyleValue<'a>;

/// Represents the style value for `background-origin` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-origin).
///
/// The background-origin CSS property sets the background starting position relative to the border and padding of an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-origin
#[syntax(" <visual-box># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "padding-box",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "repeatable list"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-origin"))]
#[visit]
pub struct BackgroundOriginStyleValue<'a>;

/// Represents the style value for `background-size` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-size).
///
/// The background-size CSS property scales or stretches a background based on the size of the element (with the contain and cover keywords), a length, or percentage.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <bg-size>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-size
#[syntax(" <bg-size># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements",
	inherited = "no",
	percentages = "see text",
	canonical_order = "per grammar",
	animation_type = "repeatable list"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-size"))]
#[visit]
pub struct BackgroundSizeStyleValue<'a>;

// /// Represents the style value for `background` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background).
// ///
// /// The background CSS property is a shorthand that sets several background properties at once.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <bg-layer>#? , <final-bg-layer>
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#background
// #[syntax(" <bg-layer>#? , <final-bg-layer> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "see individual properties",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "see individual properties",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background"))]
// #[visit]
// pub struct BackgroundStyleValue<'a>;

/// Represents the style value for `border-image-source` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#border-image-source).
///
/// The border-image CSS property draws an image around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <image>
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#border-image-source
#[syntax(" none | <image> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "All elements, except internal table elements when border-collapse is collapse",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-image-source"))]
#[visit]
pub struct BorderImageSourceStyleValue<'a>;

// /// Represents the style value for `border-image-slice` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#border-image-slice).
// ///
// /// The border-image CSS property draws an image around an element.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [<number [0,∞]> | <percentage [0,∞]>]{1,4} && fill?
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#border-image-slice
// #[syntax(" [<number [0,∞]> | <percentage [0,∞]>]{1,4} && fill? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "100%",
//   applies_to = "All elements, except internal table elements when border-collapse is collapse",
// 	inherited = "no",
// 	percentages = "refer to size of the border image",
// 	canonical_order = "per grammar",
// 	animation_type = "by computed value",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-image-slice"))]
// #[visit]
// pub struct BorderImageSliceStyleValue;

// /// Represents the style value for `border-image-width` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#border-image-width).
// ///
// /// The border-image CSS property draws an image around an element.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]> | <number [0,∞]> | auto ]{1,4}
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#border-image-width
// #[syntax(" [ <length-percentage [0,∞]> | <number [0,∞]> | auto ]{1,4} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "1",
//   applies_to = "All elements, except internal table elements when border-collapse is collapse",
// 	inherited = "no",
// 	percentages = "relative to width/height of the border image area",
// 	canonical_order = "per grammar",
// 	animation_type = "by computed value",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-image-width"))]
// #[visit]
// pub struct BorderImageWidthStyleValue;

/// Represents the style value for `border-image-outset` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#border-image-outset).
///
/// The border-image CSS property draws an image around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ <length [0,∞]> | <number [0,∞]> ]{1,4}
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#border-image-outset
#[syntax(" [ <length [0,∞]> | <number [0,∞]> ]{1,4} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "All elements, except internal table elements when border-collapse is collapse",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-image-outset"))]
#[visit]
pub struct BorderImageOutsetStyleValue;

/// Represents the style value for `border-image-repeat` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#border-image-repeat).
///
/// The border-image CSS property draws an image around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ stretch | repeat | round | space ]{1,2}
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#border-image-repeat
#[syntax(" [ stretch | repeat | round | space ]{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "stretch",
	applies_to = "All elements, except internal table elements when border-collapse is collapse",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-image-repeat"))]
#[visit]
pub struct BorderImageRepeatStyleValue;

// /// Represents the style value for `border-image` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#border-image).
// ///
// /// The border-image CSS property draws an image around an element.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-image-source'> || <'border-image-slice'> [ / <'border-image-width'> | / <'border-image-width'>? / <'border-image-outset'> ]? || <'border-image-repeat'>
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#border-image
// #[syntax(
// 	" <'border-image-source'> || <'border-image-slice'> [ / <'border-image-width'> | / <'border-image-width'>? / <'border-image-outset'> ]? || <'border-image-repeat'> "
// )]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "See individual properties",
//   applies_to = "See individual properties",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-image"))]
// #[visit]
// pub struct BorderImageStyleValue;

/// Represents the style value for `background-repeat-x` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-repeat-x).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <repetition>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-repeat-x
#[syntax(" <repetition># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "repeat",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-repeat-x"))]
#[visit]
pub struct BackgroundRepeatXStyleValue<'a>;

/// Represents the style value for `background-repeat-y` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-repeat-y).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <repetition>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-repeat-y
#[syntax(" <repetition># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "repeat",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-repeat-y"))]
#[visit]
pub struct BackgroundRepeatYStyleValue<'a>;

/// Represents the style value for `background-repeat-block` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-repeat-block).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <repetition>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-repeat-block
#[syntax(" <repetition># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "repeat",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-repeat-block"))]
#[visit]
pub struct BackgroundRepeatBlockStyleValue<'a>;

/// Represents the style value for `background-repeat-inline` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-repeat-inline).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <repetition>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-repeat-inline
#[syntax(" <repetition># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "repeat",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-repeat-inline"))]
#[visit]
pub struct BackgroundRepeatInlineStyleValue<'a>;

// /// Represents the style value for `background-position-x` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-position-x).
// ///
// /// The background-position CSS property offsets the initial position of background images relative to the background origin.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ center | [ [ left | right | x-start | x-end ]? <length-percentage>? ]! ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#background-position-x
// #[syntax(" [ center | [ [ left | right | x-start | x-end ]? <length-percentage>? ]! ]# ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0%",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "refer to width of background positioning area minus width of background image",
// 	canonical_order = "per grammar",
// 	animation_type = "repeatable list",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-position-x"))]
// #[visit]
// pub enum BackgroundPositionXStyleValue<'a> {}

// /// Represents the style value for `background-position-y` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-position-y).
// ///
// /// The background-position CSS property offsets the initial position of background images relative to the background origin.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ center | [ [ top | bottom | y-start | y-end ]? <length-percentage>? ]! ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#background-position-y
// #[syntax(" [ center | [ [ top | bottom | y-start | y-end ]? <length-percentage>? ]! ]# ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0%",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "refer to height of background positioning area minus height of background image",
// 	canonical_order = "per grammar",
// 	animation_type = "repeatable list",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-position-y"))]
// #[visit]
// pub enum BackgroundPositionYStyleValue<'a> {}

// /// Represents the style value for `background-position-inline` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-position-inline).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ center | [ [ start | end ]? <length-percentage>? ]! ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#background-position-inline
// #[syntax(" [ center | [ [ start | end ]? <length-percentage>? ]! ]# ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0%",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "refer to inline-size of background positioning area minus inline-size of background image",
// 	canonical_order = "per grammar",
// 	animation_type = "repeatable list",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-position-inline"))]
// #[visit]
// pub enum BackgroundPositionInlineStyleValue<'a> {}

// /// Represents the style value for `background-position-block` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-position-block).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ center | [ [ start | end ]? <length-percentage>? ]! ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#background-position-block
// #[syntax(" [ center | [ [ start | end ]? <length-percentage>? ]! ]# ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0%",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "refer to size of background positioning area minus size of background image",
// 	canonical_order = "per grammar",
// 	animation_type = "repeatable list",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.background-position-block"))]
// #[visit]
// pub enum BackgroundPositionBlockStyleValue<'a> {}
