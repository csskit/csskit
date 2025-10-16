#![allow(warnings)]
//! CSS Round Display Module Level 1
//! https://drafts.csswg.org/css-round-display-1/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `border-boundary` as defined in [css-round-display-1](https://drafts.csswg.org/css-round-display-1/#border-boundary).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | parent | display
/// ```
///
// https://drafts.csswg.org/css-round-display-1/#border-boundary
#[syntax(" none | parent | display ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-boundary"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderBoundaryStyleValue {}
