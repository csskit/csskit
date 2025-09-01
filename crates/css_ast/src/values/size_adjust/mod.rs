#![allow(warnings)]
//! CSS Mobile Text Size Adjustment Module Level 1
//! https://drafts.csswg.org/css-size-adjust-1/

mod impls;
use impls::*;

/// Represents the style value for `text-size-adjust` as defined in [css-size-adjust-1](https://drafts.csswg.org/css-size-adjust-1/#text-size-adjust).
///
/// The text-size-adjust CSS property disables or modifies the browser's default text size adjustment for small screen sizes.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none | <percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-size-adjust-1/#text-size-adjust
#[syntax(" auto | none | <percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements",
	inherited = "yes",
	percentages = "see below",
	canonical_order = "n/a",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-size-adjust"))]
#[visit]
pub struct TextSizeAdjustStyleValue;
