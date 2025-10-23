#![allow(warnings)]
//! CSS Generated Content for Paged Media Module Level 4
//! https://drafts.csswg.org/css-gcpm-4/

mod impls;

use super::prelude::*;
use impls::*;

// /// Represents the style value for `copy-into` as defined in [css-gcpm-4](https://drafts.csswg.org/css-gcpm-4/#copy-into).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none |  [ [ <custom-ident>  <content-level>] [,  <custom-ident>  <content-level>]*  ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-gcpm-4/#copy-into
// #[syntax(" none |  [ [ <custom-ident>  <content-level>] [,  <custom-ident>  <content-level>]*  ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "all elements and pseudo-elements, but not ::first-line or ::first-letter.",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.copy-into"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum CopyIntoStyleValue {}

/// Represents the style value for `footnote-display` as defined in [css-gcpm-4](https://drafts.csswg.org/css-gcpm-4/#footnote-display).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// block | inline | compact
/// ```
///
// https://drafts.csswg.org/css-gcpm-4/#footnote-display
#[syntax(" block | inline | compact ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "block",
	applies_to = "elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.footnote-display"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum FootnoteDisplayStyleValue {}

/// Represents the style value for `footnote-policy` as defined in [css-gcpm-4](https://drafts.csswg.org/css-gcpm-4/#footnote-policy).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | line | block
/// ```
///
// https://drafts.csswg.org/css-gcpm-4/#footnote-policy
#[syntax(" auto | line | block ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.footnote-policy"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum FootnotePolicyStyleValue {}

/// Represents the style value for `running` as defined in [css-gcpm-4](https://drafts.csswg.org/css-gcpm-4/#running).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <custom-ident>
/// ```
///
// https://drafts.csswg.org/css-gcpm-4/#running
#[syntax(" <custom-ident> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.running"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct RunningStyleValue;

// /// Represents the style value for `string-set` as defined in [css-gcpm-4](https://drafts.csswg.org/css-gcpm-4/#string-set).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <custom-ident> <content-list> ]# | none
// /// ```
// ///
// // https://drafts.csswg.org/css-gcpm-4/#string-set
// #[syntax(" [ <custom-ident> <content-list> ]# | none ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "all elements, but not pseudo-elements",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.string-set"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum StringSetStyleValue<'a> {}
