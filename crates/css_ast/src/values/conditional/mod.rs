#![allow(warnings)]
//! CSS Conditional Rules Module Level 5
//! https://drafts.csswg.org/css-conditional-5/

mod impls;
use impls::*;

// /// Represents the style value for `container-type` as defined in [css-conditional-5](https://drafts.csswg.org/css-conditional-5/#container-type).
// ///
// /// Container size queries with the @container at-rule apply styles to an element based on the dimensions of its container.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | [ [ size | inline-size ] || scroll-state ]
// /// ```
// ///
// // https://drafts.csswg.org/css-conditional-5/#container-type
// #[syntax(" normal | [ [ size | inline-size ] || scroll-state ] ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.container-type"))]
// #[visit]
// pub enum ContainerTypeStyleValue {}

/// Represents the style value for `container-name` as defined in [css-conditional-5](https://drafts.csswg.org/css-conditional-5/#container-name).
///
/// Container size queries with the @container at-rule apply styles to an element based on the dimensions of its container.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <custom-ident>+
/// ```
///
// https://drafts.csswg.org/css-conditional-5/#container-name
#[syntax(" none | <custom-ident>+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.container-name"))]
#[visit]
pub struct ContainerNameStyleValue<'a>;

// /// Represents the style value for `container` as defined in [css-conditional-5](https://drafts.csswg.org/css-conditional-5/#container).
// ///
// /// Container size queries with the @container at-rule apply styles to an element based on the dimensions of its container.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'container-name'> [ / <'container-type'> ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-conditional-5/#container
// #[syntax(" <'container-name'> [ / <'container-type'> ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "see individual properties",
//   applies_to = "see individual properties",
// 	inherited = "see individual properties",
// 	percentages = "see individual properties",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.container"))]
// #[visit]
// pub struct ContainerStyleValue;
