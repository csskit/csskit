#![allow(warnings)]
//! CSS Scrollbars Styling Module Level 1
//! https://drafts.csswg.org/css-scrollbars-1/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `scrollbar-color` as defined in [css-scrollbars-1](https://drafts.csswg.org/css-scrollbars-1/#scrollbar-color).
///
/// The scrollbar-color CSS property sets the color of the scrollbar track and thumb.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <color>{2}
/// ```
///
// https://drafts.csswg.org/css-scrollbars-1/#scrollbar-color
#[syntax(" auto | <color>{2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scrollbar-color"))]
#[visit]
pub struct ScrollbarColorStyleValue;

/// Represents the style value for `scrollbar-width` as defined in [css-scrollbars-1](https://drafts.csswg.org/css-scrollbars-1/#scrollbar-width).
///
/// The scrollbar-width CSS property sets the width of the scrollbar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | thin | none
/// ```
///
// https://drafts.csswg.org/css-scrollbars-1/#scrollbar-width
#[syntax(" auto | thin | none ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.scrollbar-width"))]
#[visit]
pub enum ScrollbarWidthStyleValue {}
