#![allow(warnings)]
//! https://drafts.csswg.org/css-contain-4/

mod impls;
use super::prelude::*;
use impls::*;
// /// Represents the style value for `contain` as defined in [css-contain-4](https://drafts.csswg.org/css-contain-4/#contain).
// ///
// /// The contain CSS property sets limits to the scope of styles, layout, and paint rendering for speed and efficiency. The none keyword value disables containment, strict is equivalent to contain: size layout style paint, and content is equivalent to contain: layout style paint.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | strict | content | [ [size | inline-size] || layout || style || paint ]
// /// ```
// ///
// /// https://drafts.csswg.org/css-contain-4/#contain
// #[syntax(
//     " none | strict | content | [ [size | inline-size] || layout || style || paint ] "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     StyleValue,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash
// )]
// #[style_value(
//     initial = "none",
//     applies_to = "See below",
//     inherited = "no",
//     percentages = "n/a",
//     canonical_order = "per grammar",
//     animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.contain")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum ContainStyleValue {}

/// Represents the style value for `content-visibility` as defined in [css-contain-4](https://drafts.csswg.org/css-contain-4/#content-visibility).
///
/// The content-visibility CSS property delays rendering an element, including layout and painting, until it is needed.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// visible | auto | hidden
/// ```
///
/// https://drafts.csswg.org/css-contain-4/#content-visibility
#[syntax(" visible | auto | hidden ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "visible",
	applies_to = "elements for which size containment can apply",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see §\u{202f}4.1 animating and interpolating content-visibility"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.content-visibility"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum ContentVisibilityStyleValue {}
