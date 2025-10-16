#![allow(warnings)]
//! CSS Overscroll Behavior Module Level 1
//! https://drafts.csswg.org/css-overscroll-1/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `overscroll-behavior` as defined in [css-overscroll-1](https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior).
///
/// The overscroll-behavior CSS property disables default scrolling behaviors when the edges of a scrolling area are reached.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ contain | none | auto ]{1,2}
/// ```
///
// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior
#[syntax(" [ contain | none | auto ]{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto auto",
	applies_to = "scroll container elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.overscroll-behavior"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct OverscrollBehaviorStyleValue;

/// Represents the style value for `overscroll-behavior-x` as defined in [css-overscroll-1](https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-x).
///
/// The overscroll-behavior CSS property disables default scrolling behaviors when the edges of a scrolling area are reached.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// contain | none | auto
/// ```
///
// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-x
#[syntax(" contain | none | auto ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll container elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.overscroll-behavior-x"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum OverscrollBehaviorXStyleValue {}

/// Represents the style value for `overscroll-behavior-y` as defined in [css-overscroll-1](https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-y).
///
/// The overscroll-behavior CSS property disables default scrolling behaviors when the edges of a scrolling area are reached.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// contain | none | auto
/// ```
///
// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-y
#[syntax(" contain | none | auto ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll container elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.overscroll-behavior-y"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum OverscrollBehaviorYStyleValue {}

/// Represents the style value for `overscroll-behavior-inline` as defined in [css-overscroll-1](https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-inline).
///
/// The overscroll-behavior CSS property disables default scrolling behaviors when the edges of a scrolling area are reached.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// contain | none | auto
/// ```
///
// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-inline
#[syntax(" contain | none | auto ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll container elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(
	feature = "css_feature_data",
	derive(ToCSSFeature),
	css_feature("css.properties.overscroll-behavior-inline")
)]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum OverscrollBehaviorInlineStyleValue {}

/// Represents the style value for `overscroll-behavior-block` as defined in [css-overscroll-1](https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-block).
///
/// The overscroll-behavior CSS property disables default scrolling behaviors when the edges of a scrolling area are reached.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// contain | none | auto
/// ```
///
// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-block
#[syntax(" contain | none | auto ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll container elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.overscroll-behavior-block"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum OverscrollBehaviorBlockStyleValue {}
