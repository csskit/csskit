#![allow(warnings)]
//! CSS Box Alignment Module Level 3
//! https://drafts.csswg.org/css-align-3/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `align-content` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#align-content).
///
/// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <baseline-position> | <content-distribution> | <overflow-position>? <content-position>
/// ```
///
// https://drafts.csswg.org/css-align-3/#align-content
#[syntax(" normal | <baseline-position> | <content-distribution> | <overflow-position>? <content-position> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "block containers, multicol containers, flex containers, and grid containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.align-content"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum AlignContentStyleValue {}

// /// Represents the style value for `align-items` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#align-items).
// ///
// /// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | stretch | <baseline-position> | [ <overflow-position>? <self-position> ]
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#align-items
// #[syntax(" normal | stretch | <baseline-position> | [ <overflow-position>? <self-position> ] ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.align-items"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum AlignItemsStyleValue {}

/// Represents the style value for `align-self` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#align-self).
///
/// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | normal | stretch | <baseline-position> | <overflow-position>? <self-position>
/// ```
///
// https://drafts.csswg.org/css-align-3/#align-self
#[syntax(" auto | normal | stretch | <baseline-position> | <overflow-position>? <self-position> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "flex items, grid items, and absolutely-positioned boxes",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.align-self"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum AlignSelfStyleValue {}

/// Represents the style value for `column-gap` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#column-gap).
///
/// Multi-column layout flows an element's content across one or more columns in a single row, without affecting the display property of its children.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-align-3/#column-gap
#[syntax(" normal | <length-percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "multi-column containers, flex containers, grid containers",
	inherited = "no",
	percentages = "see § 8.3 percentages in gap properties",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.column-gap"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum ColumnGapStyleValue {}

/// Represents the style value for `gap` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#gap).
///
/// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'row-gap'> <'column-gap'>?
/// ```
///
// https://drafts.csswg.org/css-align-3/#gap
#[syntax(" <'row-gap'> <'column-gap'>? ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "multi-column containers, flex containers, grid containers",
	inherited = "no",
	percentages = "refer to corresponding dimension of the content area",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.gap"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct GapStyleValue;

// /// Represents the style value for `justify-content` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#justify-content).
// ///
// /// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | <content-distribution> | <overflow-position>? [ <content-position> | left | right ]
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#justify-content
// #[syntax(" normal | <content-distribution> | <overflow-position>? [ <content-position> | left | right ] ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "multicol containers, flex containers, and grid containers",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.justify-content"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum JustifyContentStyleValue {}

// /// Represents the style value for `justify-items` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#justify-items).
// ///
// /// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | stretch | <baseline-position> | <overflow-position>? [ <self-position> | left | right ] | legacy | legacy && [ left | right | center ]
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#justify-items
// #[syntax(
// 	" normal | stretch | <baseline-position> | <overflow-position>? [ <self-position> | left | right ] | legacy | legacy && [ left | right | center ] "
// )]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "legacy",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.justify-items"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum JustifyItemsStyleValue {}

// /// Represents the style value for `justify-self` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#justify-self).
// ///
// /// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | normal | stretch | <baseline-position> | <overflow-position>? [ <self-position> | left | right ]
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#justify-self
// #[syntax(" auto | normal | stretch | <baseline-position> | <overflow-position>? [ <self-position> | left | right ] ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "auto",
//   applies_to = "block-level boxes, absolutely-positioned boxes, and grid items",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.justify-self"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum JustifySelfStyleValue {}

// /// Represents the style value for `place-content` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#place-content).
// ///
// /// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'align-content'> <'justify-content'>?
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#place-content
// #[syntax(" <'align-content'> <'justify-content'>? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "see individual properties",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.place-content"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct PlaceContentStyleValue;

// /// Represents the style value for `place-items` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#place-items).
// ///
// /// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'align-items'> <'justify-items'>?
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#place-items
// #[syntax(" <'align-items'> <'justify-items'>? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "see individual properties",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.place-items"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct PlaceItemsStyleValue;

// /// Represents the style value for `place-self` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#place-self).
// ///
// /// Flexbox is a one-dimensional layout system, which places content either horizontally or vertically, with optional wrapping.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'align-self'> <'justify-self'>?
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#place-self
// #[syntax(" <'align-self'> <'justify-self'>? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "auto",
//   applies_to = "see individual properties",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.place-self"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct PlaceSelfStyleValue;

/// Represents the style value for `row-gap` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#row-gap).
///
/// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-align-3/#row-gap
#[syntax(" normal | <length-percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "multi-column containers, flex containers, grid containers",
	inherited = "no",
	percentages = "see § 8.3 percentages in gap properties",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.row-gap"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum RowGapStyleValue {}
