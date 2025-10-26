#![allow(warnings)]
//! https://drafts.csswg.org/css-nav-1/

mod impls;
use super::prelude::*;
use impls::*;
/// Represents the style value for `spatial-navigation-action` as defined in [css-nav-1](https://drafts.csswg.org/css-nav-1/#spatial-navigation-action).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | focus | scroll
/// ```
///
/// https://drafts.csswg.org/css-nav-1/#spatial-navigation-action
#[syntax(" auto | focus | scroll ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "scroll containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.spatial-navigation-action"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum SpatialNavigationActionStyleValue {}

/// Represents the style value for `spatial-navigation-contain` as defined in [css-nav-1](https://drafts.csswg.org/css-nav-1/#spatial-navigation-contain).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | contain
/// ```
///
/// https://drafts.csswg.org/css-nav-1/#spatial-navigation-contain
#[syntax(" auto | contain ")]
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
#[cfg_attr(
	feature = "css_feature_data",
	derive(ToCSSFeature),
	css_feature("css.properties.spatial-navigation-contain")
)]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum SpatialNavigationContainStyleValue {}

/// Represents the style value for `spatial-navigation-function` as defined in [css-nav-1](https://drafts.csswg.org/css-nav-1/#spatial-navigation-function).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | grid
/// ```
///
/// https://drafts.csswg.org/css-nav-1/#spatial-navigation-function
#[syntax(" normal | grid ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "spatial navigation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(
	feature = "css_feature_data",
	derive(ToCSSFeature),
	css_feature("css.properties.spatial-navigation-function")
)]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum SpatialNavigationFunctionStyleValue {}
