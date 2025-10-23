#![allow(warnings)]
//! CSS Exclusions Module Level 1
//! https://drafts.csswg.org/css-exclusions-1/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `wrap-flow` as defined in [css-exclusions-1](https://drafts.csswg.org/css-exclusions-1/#wrap-flow).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | both | start | end | minimum | maximum | clear
/// ```
///
// https://drafts.csswg.org/css-exclusions-1/#wrap-flow
#[syntax(" auto | both | start | end | minimum | maximum | clear ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "block-level elements.",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.wrap-flow"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum WrapFlowStyleValue {}

/// Represents the style value for `wrap-through` as defined in [css-exclusions-1](https://drafts.csswg.org/css-exclusions-1/#wrap-through).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// wrap | none
/// ```
///
// https://drafts.csswg.org/css-exclusions-1/#wrap-through
#[syntax(" wrap | none ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "wrap",
	applies_to = "block-level elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.wrap-through"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum WrapThroughStyleValue {}
