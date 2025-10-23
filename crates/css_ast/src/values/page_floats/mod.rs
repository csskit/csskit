#![allow(warnings)]
//! CSS Page Floats Module Level 3
//! https://drafts.csswg.org/css-page-floats-3/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `clear` as defined in [css-page-floats-3](https://drafts.csswg.org/css-page-floats-3/#clear).
///
/// The float CSS property aligns an element to either side of its container, allowing text and inline elements to flow around it. The clear CSS property sets whether an element is moved below floating elements that proceed it.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// inline-start | inline-end | block-start | block-end | left | right | top | bottom | both-inline | both-block | both | none
/// ```
///
// https://drafts.csswg.org/css-page-floats-3/#clear
#[syntax(
	" inline-start | inline-end | block-start | block-end | left | right | top | bottom | both-inline | both-block | both | none "
)]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "block-level elements, floats, regions, pages",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.clear"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum ClearStyleValue {}

/// Represents the style value for `float` as defined in [css-page-floats-3](https://drafts.csswg.org/css-page-floats-3/#float).
///
/// The float CSS property aligns an element to either side of its container, allowing text and inline elements to flow around it. The clear CSS property sets whether an element is moved below floating elements that proceed it.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// block-start | block-end | inline-start | inline-end | snap-block | <snap-block()> | snap-inline | <snap-inline()> | left | right | top | bottom | none
/// ```
///
// https://drafts.csswg.org/css-page-floats-3/#float
#[syntax(
	" block-start | block-end | inline-start | inline-end | snap-block | <snap-block()> | snap-inline | <snap-inline()> | left | right | top | bottom | none "
)]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements.",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.float"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum FloatStyleValue {}

/// Represents the style value for `float-defer` as defined in [css-page-floats-3](https://drafts.csswg.org/css-page-floats-3/#float-defer).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <integer> | last | none
/// ```
///
// https://drafts.csswg.org/css-page-floats-3/#float-defer
#[syntax(" <integer> | last | none ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "floats",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.float-defer"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum FloatDeferStyleValue {}

/// Represents the style value for `float-offset` as defined in [css-page-floats-3](https://drafts.csswg.org/css-page-floats-3/#float-offset).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-page-floats-3/#float-offset
#[syntax(" <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "floats",
	inherited = "no",
	percentages = "see prose",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.float-offset"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct FloatOffsetStyleValue;

/// Represents the style value for `float-reference` as defined in [css-page-floats-3](https://drafts.csswg.org/css-page-floats-3/#float-reference).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// inline | column | region | page
/// ```
///
// https://drafts.csswg.org/css-page-floats-3/#float-reference
#[syntax(" inline | column | region | page ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "inline",
	applies_to = "all elements.",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.float-reference"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum FloatReferenceStyleValue {}
