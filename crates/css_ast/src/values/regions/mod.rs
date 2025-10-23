#![allow(warnings)]
//! CSS Regions Module Level 1
//! https://drafts.csswg.org/css-regions-1/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `flow-from` as defined in [css-regions-1](https://drafts.csswg.org/css-regions-1/#flow-from).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <custom-ident> | none
/// ```
///
// https://drafts.csswg.org/css-regions-1/#flow-from
#[syntax(" <custom-ident> | none ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "Non-replaced block containers.  This might be expanded in future versions of the specification to allow other types of containers to receive flow content.",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.flow-from"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct FlowFromStyleValue;

// /// Represents the style value for `flow-into` as defined in [css-regions-1](https://drafts.csswg.org/css-regions-1/#flow-into).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | <custom-ident> [element | content]?
// /// ```
// ///
// // https://drafts.csswg.org/css-regions-1/#flow-into
// #[syntax(" none | <custom-ident> [element | content]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "All elements, but not pseudo-elements such as ::first-line, ::first-letter, ::before or ::after.",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.flow-into"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum FlowIntoStyleValue {}

/// Represents the style value for `region-fragment` as defined in [css-regions-1](https://drafts.csswg.org/css-regions-1/#region-fragment).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | break
/// ```
///
// https://drafts.csswg.org/css-regions-1/#region-fragment
#[syntax(" auto | break ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "CSS Regions",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.region-fragment"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum RegionFragmentStyleValue {}
