#![allow(warnings)]
//! CSS Color Module Level 4
//! https://drafts.csswg.org/css-color-6/

mod impls;
use impls::*;

/// Represents the style value for `color` as defined in [css-color-6](https://drafts.csswg.org/css-color-6/#color).
///
/// The color CSS property sets the primary foreground color of an element, which is used for text, the default border color, and text decorations.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color>
/// ```
///
// https://drafts.csswg.org/css-color-6/#color
#[syntax(" <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "CanvasText",
	applies_to = "all elements and text",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.color"))]
#[visit]
pub struct ColorStyleValue;

/// Represents the style value for `opacity` as defined in [css-color-6](https://drafts.csswg.org/css-color-6/#opacity).
///
/// The opacity CSS property sets the transparency of an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <opacity-value>
/// ```
///
// https://drafts.csswg.org/css-color-6/#opacity
#[syntax(" <opacity-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "1",
	applies_to = "all elements",
	inherited = "no",
	percentages = "map to the range [0,1]",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.opacity"))]
#[visit]
pub struct OpacityStyleValue;
