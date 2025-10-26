#![allow(warnings)]
//! https://drafts.csswg.org/css-flexbox-1/

mod impls;
use super::prelude::*;
use impls::*;
// /// Represents the style value for `flex` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex).
// ///
// /// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ <'flex-grow'> <'flex-shrink'>? || <'flex-basis'> ]
// /// ```
// ///
// /// https://drafts.csswg.org/css-flexbox-1/#flex
// #[syntax(" none | [ <'flex-grow'> <'flex-shrink'>? || <'flex-basis'> ] ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     StyleValue,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash
// )]
// #[style_value(
//     initial = "0 1 auto",
//     applies_to = "flex items",
//     inherited = "no",
//     percentages = "see individual properties",
//     canonical_order = "per grammar",
//     animation_type = "by computed value type",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.flex")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct FlexStyleValue;

/// Represents the style value for `flex-basis` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex-basis).
///
/// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// content | <'width'>
/// ```
///
/// https://drafts.csswg.org/css-flexbox-1/#flex-basis
#[syntax(" content | <'width'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "flex items",
	inherited = "no",
	percentages = "relative to the flex container’s inner main size",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.flex-basis"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum FlexBasisStyleValue {}

/// Represents the style value for `flex-direction` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex-direction).
///
/// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// row | row-reverse | column | column-reverse
/// ```
///
/// https://drafts.csswg.org/css-flexbox-1/#flex-direction
#[syntax(" row | row-reverse | column | column-reverse ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "row",
	applies_to = "flex containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.flex-direction"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum FlexDirectionStyleValue {}

/// Represents the style value for `flex-flow` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex-flow).
///
/// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'flex-direction'> || <'flex-wrap'>
/// ```
///
/// https://drafts.csswg.org/css-flexbox-1/#flex-flow
#[syntax(" <'flex-direction'> || <'flex-wrap'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.flex-flow"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct FlexFlowStyleValue;

/// Represents the style value for `flex-grow` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex-grow).
///
/// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <number [0,∞]>
/// ```
///
/// https://drafts.csswg.org/css-flexbox-1/#flex-grow
#[syntax(" <number [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "flex items",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.flex-grow"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct FlexGrowStyleValue;

/// Represents the style value for `flex-shrink` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex-shrink).
///
/// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <number [0,∞]>
/// ```
///
/// https://drafts.csswg.org/css-flexbox-1/#flex-shrink
#[syntax(" <number [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "1",
	applies_to = "flex items",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "number"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.flex-shrink"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct FlexShrinkStyleValue;

/// Represents the style value for `flex-wrap` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex-wrap).
///
/// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// nowrap | wrap | wrap-reverse
/// ```
///
/// https://drafts.csswg.org/css-flexbox-1/#flex-wrap
#[syntax(" nowrap | wrap | wrap-reverse ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "nowrap",
	applies_to = "flex containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.flex-wrap"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum FlexWrapStyleValue {}
