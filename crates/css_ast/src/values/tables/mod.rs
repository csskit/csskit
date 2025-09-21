#![allow(warnings)]
//! CSS Table Module Level 3
//! https://drafts.csswg.org/css-tables-3/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `table-layout` as defined in [css-tables-3](https://drafts.csswg.org/css-tables-3/#table-layout).
///
/// The <table> HTML element, with several related elements, represents tabular data in rows and columns of cells.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | fixed
/// ```
///
// https://drafts.csswg.org/css-tables-3/#table-layout
#[syntax(" auto | fixed ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "table grid boxes",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.table-layout"))]
#[visit]
pub enum TableLayoutStyleValue {}

/// Represents the style value for `border-collapse` as defined in [css-tables-3](https://drafts.csswg.org/css-tables-3/#border-collapse).
///
/// The <table> HTML element, with several related elements, represents tabular data in rows and columns of cells.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// separate | collapse
/// ```
///
// https://drafts.csswg.org/css-tables-3/#border-collapse
#[syntax(" separate | collapse ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "separate",
	applies_to = "table grid boxes",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-collapse"))]
#[visit]
pub enum BorderCollapseStyleValue {}

/// Represents the style value for `border-spacing` as defined in [css-tables-3](https://drafts.csswg.org/css-tables-3/#border-spacing).
///
/// The <table> HTML element, with several related elements, represents tabular data in rows and columns of cells.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>{1,2}
/// ```
///
// https://drafts.csswg.org/css-tables-3/#border-spacing
#[syntax(" <length>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0px 0px",
	applies_to = "table grid boxes when border-collapse is separate",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-spacing"))]
#[visit]
pub struct BorderSpacingStyleValue;

/// Represents the style value for `caption-side` as defined in [css-tables-3](https://drafts.csswg.org/css-tables-3/#caption-side).
///
/// The <table> HTML element, with several related elements, represents tabular data in rows and columns of cells.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// top | bottom
/// ```
///
// https://drafts.csswg.org/css-tables-3/#caption-side
#[syntax(" top | bottom ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "top",
	applies_to = "table-caption boxes",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.caption-side"))]
#[visit]
pub enum CaptionSideStyleValue {}

/// Represents the style value for `empty-cells` as defined in [css-tables-3](https://drafts.csswg.org/css-tables-3/#empty-cells).
///
/// The <table> HTML element, with several related elements, represents tabular data in rows and columns of cells.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// show | hide
/// ```
///
// https://drafts.csswg.org/css-tables-3/#empty-cells
#[syntax(" show | hide ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "show",
	applies_to = "table-cell boxes",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.empty-cells"))]
#[visit]
pub enum EmptyCellsStyleValue {}
