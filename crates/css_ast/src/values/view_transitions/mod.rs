#![allow(warnings)]
//! CSS View Transitions Module Level 2
//! https://drafts.csswg.org/css-view-transitions-2/

mod impls;
use impls::*;

/// Represents the style value for `view-transition-name` as defined in [css-view-transitions-2](https://drafts.csswg.org/css-view-transitions-2/#view-transition-name).
///
/// View transitions allow you to create animated visual transitions between different states of a document.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <custom-ident>
/// ```
///
// https://drafts.csswg.org/css-view-transitions-2/#view-transition-name
#[syntax(" none | <custom-ident> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.view-transition-name"))]
#[visit]
pub struct ViewTransitionNameStyleValue;

/// Represents the style value for `view-transition-class` as defined in [css-view-transitions-2](https://drafts.csswg.org/css-view-transitions-2/#view-transition-class).
///
/// The view-transition-class CSS property sets a name that can be used to apply styles to multiple named view transition pseudo-elements.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <custom-ident>+
/// ```
///
// https://drafts.csswg.org/css-view-transitions-2/#view-transition-class
#[syntax(" none | <custom-ident>+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.view-transition-class"))]
#[visit]
pub struct ViewTransitionClassStyleValue<'a>;

/// Represents the style value for `view-transition-group` as defined in [css-view-transitions-2](https://drafts.csswg.org/css-view-transitions-2/#view-transition-group).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | contain | nearest | <custom-ident>
/// ```
///
// https://drafts.csswg.org/css-view-transitions-2/#view-transition-group
#[syntax(" normal | contain | nearest | <custom-ident> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.view-transition-group"))]
#[visit]
pub enum ViewTransitionGroupStyleValue {}
