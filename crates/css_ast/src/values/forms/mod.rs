#![allow(warnings)]
//! CSS Form Control Styling Module Level 1
//! https://drafts.csswg.org/css-forms-1/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `field-sizing` as defined in [css-forms-1](https://drafts.csswg.org/css-forms-1/#field-sizing).
///
/// The field-sizing CSS property allows form controls such as <textarea> to be sized based on their content.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// fixed | content
/// ```
///
// https://drafts.csswg.org/css-forms-1/#field-sizing
#[syntax(" fixed | content ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "fixed",
	applies_to = "elements with default preferred size",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.field-sizing"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum FieldSizingStyleValue {}

/// Represents the style value for `slider-orientation` as defined in [css-forms-1](https://drafts.csswg.org/css-forms-1/#slider-orientation).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | left-to-right | right-to-left | top-to-bottom | bottom-to-top
/// ```
///
// https://drafts.csswg.org/css-forms-1/#slider-orientation
#[syntax(" auto | left-to-right | right-to-left | top-to-bottom | bottom-to-top ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.slider-orientation"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum SliderOrientationStyleValue {}

/// Represents the style value for `input-security` as defined in [css-forms-1](https://drafts.csswg.org/css-forms-1/#input-security).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none
/// ```
///
// https://drafts.csswg.org/css-forms-1/#input-security
#[syntax(" auto | none ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "sensitive text inputs",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.input-security"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum InputSecurityStyleValue {}
