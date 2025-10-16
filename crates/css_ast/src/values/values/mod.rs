#![allow(warnings)]
//! CSS Values and Units Module Level 5
//! https://drafts.csswg.org/css-values-5/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `interpolate-size` as defined in [css-values-5](https://drafts.csswg.org/css-values-5/#interpolate-size).
///
/// The interpolate-size CSS property sets whether animations and transitions interpolate between a numeric value and a keyword value, such as from a fixed length to auto or fit-content.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// numeric-only | allow-keywords
/// ```
///
// https://drafts.csswg.org/css-values-5/#interpolate-size
#[syntax(" numeric-only | allow-keywords ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "numeric-only",
	applies_to = "all elements",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.interpolate-size"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum InterpolateSizeStyleValue {}
