#![allow(warnings)]
//! CSS Color Adjustment Module Level 1
//! https://drafts.csswg.org/css-color-adjust-1/

mod impls;

use super::prelude::*;
use impls::*;

// /// Represents the style value for `color-scheme` as defined in [css-color-adjust-1](https://drafts.csswg.org/css-color-adjust-1/#color-scheme).
// ///
// /// The color-scheme CSS property sets which color schemes (light or dark) an element uses and may prevent automatic dark mode adjustments by the browser.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | [ light | dark | <custom-ident> ]+ && only?
// /// ```
// ///
// // https://drafts.csswg.org/css-color-adjust-1/#color-scheme
// #[syntax(" normal | [ light | dark | <custom-ident> ]+ && only? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "all elements and text",
// 	inherited = "yes",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.color-scheme"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum ColorSchemeStyleValue<'a> {}

/// Represents the style value for `forced-color-adjust` as defined in [css-color-adjust-1](https://drafts.csswg.org/css-color-adjust-1/#forced-color-adjust).
///
/// The forced-colors CSS @media rule detects when a user has chosen to use a forced colors mode, also known as high-contrast mode, and the forced-color-adjust CSS property sets whether forced colors apply to an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none | preserve-parent-color
/// ```
///
// https://drafts.csswg.org/css-color-adjust-1/#forced-color-adjust
#[syntax(" auto | none | preserve-parent-color ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements and text",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.forced-color-adjust"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum ForcedColorAdjustStyleValue {}

/// Represents the style value for `print-color-adjust` as defined in [css-color-adjust-1](https://drafts.csswg.org/css-color-adjust-1/#print-color-adjust).
///
/// The print-color-adjust CSS property sets whether styles of printed pages should be adjusted to use less ink, in cases such as light text on a dark background.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// economy | exact
/// ```
///
// https://drafts.csswg.org/css-color-adjust-1/#print-color-adjust
#[syntax(" economy | exact ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "economy",
	applies_to = "all elements",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.print-color-adjust"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum PrintColorAdjustStyleValue {}

/// Represents the style value for `color-adjust` as defined in [css-color-adjust-1](https://drafts.csswg.org/css-color-adjust-1/#color-adjust).
///
/// The color-adjust shorthand CSS property allows multiple performance related color adjustments to be set at once. Setting the print-color-adjust CSS property directly is preferred, as it is the only such adjustment so far defined.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'print-color-adjust'>
/// ```
///
// https://drafts.csswg.org/css-color-adjust-1/#color-adjust
#[syntax(" <'print-color-adjust'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.color-adjust"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct ColorAdjustStyleValue;
