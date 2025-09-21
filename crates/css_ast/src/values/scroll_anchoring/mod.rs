#![allow(warnings)]
//! CSS Scroll Anchoring Module Level 1
//! https://drafts.csswg.org/css-scroll-anchoring-1/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `overflow-anchor` as defined in [css-scroll-anchoring-1](https://drafts.csswg.org/css-scroll-anchoring-1/#overflow-anchor).
///
/// The overflow-anchor CSS property sets an element as a possible scroll anchor, reducing unintended scrolling when document changes occur above the current scrollport. This is enabled by default where supported.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none
/// ```
///
// https://drafts.csswg.org/css-scroll-anchoring-1/#overflow-anchor
#[syntax(" auto | none ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.overflow-anchor"))]
#[visit]
pub enum OverflowAnchorStyleValue {}
