#![allow(warnings)]
//! CSS Scroll Snap Module Level 2
//! https://drafts.csswg.org/css-scroll-snap-2/

mod impls;
use impls::*;

// /// Represents the style value for `scroll-snap-type` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-type).
// ///
// /// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ x | y | block | inline | both ] [ mandatory | proximity ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-type
// #[syntax(" none | [ x | y | block | inline | both ] [ mandatory | proximity ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-snap-type"))]
// #[visit]
// pub enum ScrollSnapTypeStyleValue {}

/// Represents the style value for `scroll-padding` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ auto | <length-percentage [0,∞]> ]{1,4}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding
#[syntax(" [ auto | <length-percentage [0,∞]> ]{1,4} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "relative to the corresponding dimension of the scroll container’s scrollport",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-padding"))]
#[visit]
pub struct ScrollPaddingStyleValue;

/// Represents the style value for `scroll-margin` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>{1,4}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin
#[syntax(" <length>{1,4} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-margin"))]
#[visit]
pub struct ScrollMarginStyleValue;

/// Represents the style value for `scroll-snap-align` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-align).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ none | start | end | center ]{1,2}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-align
#[syntax(" [ none | start | end | center ]{1,2} ")]
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
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-snap-align"))]
#[visit]
pub struct ScrollSnapAlignStyleValue;

/// Represents the style value for `scroll-snap-stop` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-stop).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | always
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-stop
#[syntax(" normal | always ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-snap-stop"))]
#[visit]
pub enum ScrollSnapStopStyleValue {}

/// Represents the style value for `scroll-padding-top` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-top).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-top
#[syntax(" auto | <length-percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "relative to the scroll container’s scrollport",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-padding-top"))]
#[visit]
pub struct ScrollPaddingTopStyleValue;

/// Represents the style value for `scroll-padding-right` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-right).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-right
#[syntax(" auto | <length-percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "relative to the scroll container’s scrollport",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-padding-right"))]
#[visit]
pub struct ScrollPaddingRightStyleValue;

/// Represents the style value for `scroll-padding-bottom` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-bottom).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-bottom
#[syntax(" auto | <length-percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "relative to the scroll container’s scrollport",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-padding-bottom"))]
#[visit]
pub struct ScrollPaddingBottomStyleValue;

/// Represents the style value for `scroll-padding-left` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-left).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-left
#[syntax(" auto | <length-percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "relative to the scroll container’s scrollport",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-padding-left"))]
#[visit]
pub struct ScrollPaddingLeftStyleValue;

/// Represents the style value for `scroll-padding-inline-start` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline-start).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline-start
#[syntax(" auto | <length-percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "relative to the scroll container’s scrollport",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(
	feature = "css_feature_data",
	derive(ToCSSFeature),
	css_feature("css.properties.scroll-padding-inline-start")
)]
#[visit]
pub struct ScrollPaddingInlineStartStyleValue;

/// Represents the style value for `scroll-padding-block-start` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block-start).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block-start
#[syntax(" auto | <length-percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "relative to the scroll container’s scrollport",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(
	feature = "css_feature_data",
	derive(ToCSSFeature),
	css_feature("css.properties.scroll-padding-block-start")
)]
#[visit]
pub struct ScrollPaddingBlockStartStyleValue;

/// Represents the style value for `scroll-padding-inline-end` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline-end).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline-end
#[syntax(" auto | <length-percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "relative to the scroll container’s scrollport",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-padding-inline-end"))]
#[visit]
pub struct ScrollPaddingInlineEndStyleValue;

/// Represents the style value for `scroll-padding-block-end` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block-end).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block-end
#[syntax(" auto | <length-percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "relative to the scroll container’s scrollport",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-padding-block-end"))]
#[visit]
pub struct ScrollPaddingBlockEndStyleValue;

/// Represents the style value for `scroll-padding-block` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ auto | <length-percentage [0,∞]> ]{1,2}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block
#[syntax(" [ auto | <length-percentage [0,∞]> ]{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "relative to the scroll container’s scrollport",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-padding-block"))]
#[visit]
pub struct ScrollPaddingBlockStyleValue;

/// Represents the style value for `scroll-padding-inline` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ auto | <length-percentage [0,∞]> ]{1,2}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline
#[syntax(" [ auto | <length-percentage [0,∞]> ]{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "relative to the scroll container’s scrollport",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-padding-inline"))]
#[visit]
pub struct ScrollPaddingInlineStyleValue;

/// Represents the style value for `scroll-margin-top` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-top).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-top
#[syntax(" <length> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-margin-top"))]
#[visit]
pub struct ScrollMarginTopStyleValue;

/// Represents the style value for `scroll-margin-right` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-right).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-right
#[syntax(" <length> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-margin-right"))]
#[visit]
pub struct ScrollMarginRightStyleValue;

/// Represents the style value for `scroll-margin-bottom` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-bottom).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-bottom
#[syntax(" <length> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-margin-bottom"))]
#[visit]
pub struct ScrollMarginBottomStyleValue;

/// Represents the style value for `scroll-margin-left` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-left).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-left
#[syntax(" <length> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-margin-left"))]
#[visit]
pub struct ScrollMarginLeftStyleValue;

/// Represents the style value for `scroll-margin-block-start` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block-start).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block-start
#[syntax(" <length> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-margin-block-start"))]
#[visit]
pub struct ScrollMarginBlockStartStyleValue;

/// Represents the style value for `scroll-margin-inline-start` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline-start).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline-start
#[syntax(" <length> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(
	feature = "css_feature_data",
	derive(ToCSSFeature),
	css_feature("css.properties.scroll-margin-inline-start")
)]
#[visit]
pub struct ScrollMarginInlineStartStyleValue;

/// Represents the style value for `scroll-margin-block-end` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block-end).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block-end
#[syntax(" <length> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-margin-block-end"))]
#[visit]
pub struct ScrollMarginBlockEndStyleValue;

/// Represents the style value for `scroll-margin-inline-end` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline-end).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline-end
#[syntax(" <length> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-margin-inline-end"))]
#[visit]
pub struct ScrollMarginInlineEndStyleValue;

/// Represents the style value for `scroll-margin-block` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>{1,2}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block
#[syntax(" <length>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-margin-block"))]
#[visit]
pub struct ScrollMarginBlockStyleValue;

/// Represents the style value for `scroll-margin-inline` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline).
///
/// CSS scroll snap controls the panning and scrolling behavior within a scroll container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>{1,2}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline
#[syntax(" <length>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-margin-inline"))]
#[visit]
pub struct ScrollMarginInlineStyleValue;

/// Represents the style value for `scroll-initial-target` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-initial-target).
///
/// The scroll-initial-target: nearest CSS declaration sets the initial scroll position of its scroll container to the top of the element, much like scrolling to a URL fragment.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | nearest
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-initial-target
#[syntax(" none | nearest ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "none"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scroll-initial-target"))]
#[visit]
pub enum ScrollInitialTargetStyleValue {}
