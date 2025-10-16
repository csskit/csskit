#![allow(warnings)]
//! CSS Grid Layout Module Level 3
//! https://drafts.csswg.org/css-grid-3/

mod impls;

use super::prelude::*;
use impls::*;

// /// Represents the style value for `grid-template-columns` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-template-columns).
// ///
// /// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | <track-list> | <auto-track-list> | subgrid <line-name-list>?
// /// ```
// ///
// // https://drafts.csswg.org/css-grid-3/#grid-template-columns
// #[syntax(" none | <track-list> | <auto-track-list> | subgrid <line-name-list>? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "grid containers",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the content area",
// 	canonical_order = "per grammar",
// 	animation_type = "if the list lengths match, by computed value type per item in the computed track list (see § 7.2.5 computed value of a track listing and § 7.2.3.3 interpolation/combination of repeat()); discrete otherwise",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-template-columns"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum GridTemplateColumnsStyleValue {}

// /// Represents the style value for `grid-template-rows` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-template-rows).
// ///
// /// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | <track-list> | <auto-track-list> | subgrid <line-name-list>?
// /// ```
// ///
// // https://drafts.csswg.org/css-grid-3/#grid-template-rows
// #[syntax(" none | <track-list> | <auto-track-list> | subgrid <line-name-list>? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "grid containers",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the content area",
// 	canonical_order = "per grammar",
// 	animation_type = "if the list lengths match, by computed value type per item in the computed track list (see § 7.2.5 computed value of a track listing and § 7.2.3.3 interpolation/combination of repeat()); discrete otherwise",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-template-rows"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum GridTemplateRowsStyleValue {}

/// Represents the style value for `grid-template-areas` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-template-areas).
///
/// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <string>+
/// ```
///
// https://drafts.csswg.org/css-grid-3/#grid-template-areas
#[syntax(" none | <string>+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "grid containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-template-areas"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct GridTemplateAreasStyleValue<'a>;

// /// Represents the style value for `grid-template` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-template).
// ///
// /// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ <'grid-template-rows'> / <'grid-template-columns'> ] | [ <line-names>? <string> <track-size>? <line-names>? ]+ [ / <explicit-track-list> ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-grid-3/#grid-template
// #[syntax(
// 	" none | [ <'grid-template-rows'> / <'grid-template-columns'> ] | [ <line-names>? <string> <track-size>? <line-names>? ]+ [ / <explicit-track-list> ]? "
// )]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "grid containers",
// 	inherited = "see individual properties",
// 	percentages = "see individual properties",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-template"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum GridTemplateStyleValue<'a> {}

/// Represents the style value for `grid-auto-columns` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-auto-columns).
///
/// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <track-size>+
/// ```
///
// https://drafts.csswg.org/css-grid-3/#grid-auto-columns
#[syntax(" <track-size>+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "grid containers",
	inherited = "no",
	percentages = "see track sizing",
	canonical_order = "per grammar",
	animation_type = "if the list lengths match, by computed value type per item; discrete otherwise"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-auto-columns"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct GridAutoColumnsStyleValue<'a>;

/// Represents the style value for `grid-auto-rows` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-auto-rows).
///
/// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <track-size>+
/// ```
///
// https://drafts.csswg.org/css-grid-3/#grid-auto-rows
#[syntax(" <track-size>+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "grid containers",
	inherited = "no",
	percentages = "see track sizing",
	canonical_order = "per grammar",
	animation_type = "if the list lengths match, by computed value type per item; discrete otherwise"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-auto-rows"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct GridAutoRowsStyleValue<'a>;

// /// Represents the style value for `grid-auto-flow` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-auto-flow).
// ///
// /// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ row | column ] || dense
// /// ```
// ///
// // https://drafts.csswg.org/css-grid-3/#grid-auto-flow
// #[syntax(" [ row | column ] || dense ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "row",
//   applies_to = "grid containers",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-auto-flow"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct GridAutoFlowStyleValue;

// /// Represents the style value for `grid` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid).
// ///
// /// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'grid-template'> | <'grid-template-rows'> / [ auto-flow && dense? ] <'grid-auto-columns'>? | [ auto-flow && dense? ] <'grid-auto-rows'>? / <'grid-template-columns'>
// /// ```
// ///
// // https://drafts.csswg.org/css-grid-3/#grid
// #[syntax(
// 	" <'grid-template'> | <'grid-template-rows'> / [ auto-flow && dense? ] <'grid-auto-columns'>? | [ auto-flow && dense? ] <'grid-auto-rows'>? / <'grid-template-columns'> "
// )]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "grid containers",
// 	inherited = "see individual properties",
// 	percentages = "see individual properties",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum GridStyleValue {}

/// Represents the style value for `grid-row-start` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-row-start).
///
/// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <grid-line>
/// ```
///
// https://drafts.csswg.org/css-grid-3/#grid-row-start
#[syntax(" <grid-line> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "grid items and absolutely-positioned boxes whose containing block is a grid container",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-row-start"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct GridRowStartStyleValue;

/// Represents the style value for `grid-column-start` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-column-start).
///
/// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <grid-line>
/// ```
///
// https://drafts.csswg.org/css-grid-3/#grid-column-start
#[syntax(" <grid-line> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "grid items and absolutely-positioned boxes whose containing block is a grid container",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-column-start"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct GridColumnStartStyleValue;

/// Represents the style value for `grid-row-end` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-row-end).
///
/// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <grid-line>
/// ```
///
// https://drafts.csswg.org/css-grid-3/#grid-row-end
#[syntax(" <grid-line> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "grid items and absolutely-positioned boxes whose containing block is a grid container",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-row-end"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct GridRowEndStyleValue;

/// Represents the style value for `grid-column-end` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-column-end).
///
/// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <grid-line>
/// ```
///
// https://drafts.csswg.org/css-grid-3/#grid-column-end
#[syntax(" <grid-line> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "grid items and absolutely-positioned boxes whose containing block is a grid container",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-column-end"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct GridColumnEndStyleValue;

// /// Represents the style value for `grid-row` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-row).
// ///
// /// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <grid-line> [ / <grid-line> ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-grid-3/#grid-row
// #[syntax(" <grid-line> [ / <grid-line> ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "auto",
//   applies_to = "grid items and absolutely-positioned boxes whose containing block is a grid container",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-row"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct GridRowStyleValue;

// /// Represents the style value for `grid-column` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-column).
// ///
// /// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <grid-line> [ / <grid-line> ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-grid-3/#grid-column
// #[syntax(" <grid-line> [ / <grid-line> ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "auto",
//   applies_to = "grid items and absolutely-positioned boxes whose containing block is a grid container",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-column"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct GridColumnStyleValue;

// /// Represents the style value for `grid-area` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#grid-area).
// ///
// /// CSS grid is a two-dimensional layout system, which lays content out in rows and columns.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <grid-line> [ / <grid-line> ]{0,3}
// /// ```
// ///
// // https://drafts.csswg.org/css-grid-3/#grid-area
// #[syntax(" <grid-line> [ / <grid-line> ]{0,3} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "auto",
//   applies_to = "grid items and absolutely-positioned boxes whose containing block is a grid container",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.grid-area"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct GridAreaStyleValue;

/// Represents the style value for `item-tolerance` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#item-tolerance).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <length-percentage> | infinite
/// ```
///
// https://drafts.csswg.org/css-grid-3/#item-tolerance
#[syntax(" normal | <length-percentage> | infinite ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "masonry containers",
	inherited = "no",
	percentages = "relative to the grid-axis content box size of the masonry container",
	canonical_order = "per grammar",
	animation_type = "as length"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.item-tolerance"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum ItemToleranceStyleValue {}

/// Represents the style value for `item-direction` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#item-direction).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | row | column | row-reverse | column-reverse
/// ```
///
// https://drafts.csswg.org/css-grid-3/#item-direction
#[syntax(" auto | row | column | row-reverse | column-reverse ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "flex containers, grid containers, masonry containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.item-direction"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum ItemDirectionStyleValue {}

/// Represents the style value for `item-track` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#item-track).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | row | column | row-reverse | column-reverse
/// ```
///
// https://drafts.csswg.org/css-grid-3/#item-track
#[syntax(" auto | row | column | row-reverse | column-reverse ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "flex containers, grid containers, masonry containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.item-track"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum ItemTrackStyleValue {}

// /// Represents the style value for `item-wrap` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#item-wrap).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ auto | nowrap | wrap ] || [ normal | reverse ] | wrap-reverse
// /// ```
// ///
// // https://drafts.csswg.org/css-grid-3/#item-wrap
// #[syntax(" [ auto | nowrap | wrap ] || [ normal | reverse ] | wrap-reverse ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "auto",
//   applies_to = "flex containers, grid containers, masonry containers",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.item-wrap"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum ItemWrapStyleValue {}

// /// Represents the style value for `item-cross` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#item-cross).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ auto | nowrap | wrap ] || [ normal | reverse ] | wrap-reverse
// /// ```
// ///
// // https://drafts.csswg.org/css-grid-3/#item-cross
// #[syntax(" [ auto | nowrap | wrap ] || [ normal | reverse ] | wrap-reverse ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "auto",
//   applies_to = "flex containers, grid containers, masonry containers",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.item-cross"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum ItemCrossStyleValue {}

// /// Represents the style value for `item-pack` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#item-pack).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | dense || balance
// /// ```
// ///
// // https://drafts.csswg.org/css-grid-3/#item-pack
// #[syntax(" normal | dense || balance ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "flex containers, grid containers, masonry containers",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.item-pack"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum ItemPackStyleValue {}

// /// Represents the style value for `item-flow` as defined in [css-grid-3](https://drafts.csswg.org/css-grid-3/#item-flow).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'item-direction'> || <'item-wrap'> || <'item-pack'> || <'item-tolerance'>
// /// ```
// ///
// // https://drafts.csswg.org/css-grid-3/#item-flow
// #[syntax(" <'item-direction'> || <'item-wrap'> || <'item-pack'> || <'item-tolerance'> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "see individual properties",
//   applies_to = "see individual properties",
// 	inherited = "see individual properties",
// 	percentages = "see individual properties",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.item-flow"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct ItemFlowStyleValue;
