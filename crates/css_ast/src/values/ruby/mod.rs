#![allow(warnings)]
//! CSS Ruby Annotation Layout Module Level 1
//! https://drafts.csswg.org/css-ruby-1/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `ruby-align` as defined in [css-ruby-1](https://drafts.csswg.org/css-ruby-1/#ruby-align).
///
/// The ruby-align CSS property sets the spacing and alignment of ruby annotation text when it does not fill its available space.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// start | center | space-between | space-around
/// ```
///
// https://drafts.csswg.org/css-ruby-1/#ruby-align
#[syntax(" start | center | space-between | space-around ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "space-around",
	applies_to = "ruby bases, ruby annotations, ruby base containers, ruby annotation containers",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.ruby-align"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum RubyAlignStyleValue {}

/// Represents the style value for `ruby-merge` as defined in [css-ruby-1](https://drafts.csswg.org/css-ruby-1/#ruby-merge).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// separate | merge | auto
/// ```
///
// https://drafts.csswg.org/css-ruby-1/#ruby-merge
#[syntax(" separate | merge | auto ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "separate",
	applies_to = "interlinear ruby annotation containers",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.ruby-merge"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum RubyMergeStyleValue {}

/// Represents the style value for `ruby-overhang` as defined in [css-ruby-1](https://drafts.csswg.org/css-ruby-1/#ruby-overhang).
///
/// The ruby-overhang CSS property sets whether ruby annotations may overlap adjacent text.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none
/// ```
///
// https://drafts.csswg.org/css-ruby-1/#ruby-overhang
#[syntax(" auto | none ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "ruby annotation containers",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.ruby-overhang"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum RubyOverhangStyleValue {}

// /// Represents the style value for `ruby-position` as defined in [css-ruby-1](https://drafts.csswg.org/css-ruby-1/#ruby-position).
// ///
// /// The ruby-position CSS property sets the position of a ruby annotation in relation to its base text. Annotations can display over, under, or interleaved with the base text.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ alternate || [ over | under ] ] | inter-character
// /// ```
// ///
// // https://drafts.csswg.org/css-ruby-1/#ruby-position
// #[syntax(" [ alternate || [ over | under ] ] | inter-character ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "alternate",
//   applies_to = "ruby annotation containers",
// 	inherited = "yes",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.ruby-position"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum RubyPositionStyleValue {}
