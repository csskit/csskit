#![allow(warnings)]
//! CSS Lists and Counters Module Level 3
//! https://drafts.csswg.org/css-lists-3/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `list-style-image` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#list-style-image).
///
/// The list-style shorthand CSS property and the list-style-image, list-style-position, and list-style-type longhand properties set the position and appearance of a list item's marker.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <image> | none
/// ```
///
// https://drafts.csswg.org/css-lists-3/#list-style-image
#[syntax(" <image> | none ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "list items",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.list-style-image"))]
#[visit]
pub struct ListStyleImageStyleValue<'a>;

/// Represents the style value for `list-style-type` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#list-style-type).
///
/// The list-style shorthand CSS property and the list-style-image, list-style-position, and list-style-type longhand properties set the position and appearance of a list item's marker.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <counter-style> | <string> | none
/// ```
///
// https://drafts.csswg.org/css-lists-3/#list-style-type
#[syntax(" <counter-style> | <string> | none ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "disc",
	applies_to = "list items",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.list-style-type"))]
#[visit]
pub enum ListStyleTypeStyleValue<'a> {}

/// Represents the style value for `list-style-position` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#list-style-position).
///
/// The list-style shorthand CSS property and the list-style-image, list-style-position, and list-style-type longhand properties set the position and appearance of a list item's marker.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// inside | outside
/// ```
///
// https://drafts.csswg.org/css-lists-3/#list-style-position
#[syntax(" inside | outside ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "outside",
	applies_to = "list items",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.list-style-position"))]
#[visit]
pub enum ListStylePositionStyleValue {}

// /// Represents the style value for `list-style` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#list-style).
// ///
// /// The list-style shorthand CSS property and the list-style-image, list-style-position, and list-style-type longhand properties set the position and appearance of a list item's marker.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'list-style-position'> || <'list-style-image'> || <'list-style-type'>
// /// ```
// ///
// // https://drafts.csswg.org/css-lists-3/#list-style
// #[syntax(" <'list-style-position'> || <'list-style-image'> || <'list-style-type'> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "see individual properties",
//   applies_to = "list items",
// 	inherited = "see individual properties",
// 	percentages = "see individual properties",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.list-style"))]
// #[visit]
// pub struct ListStyleStyleValue;

/// Represents the style value for `marker-side` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#marker-side).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// match-self | match-parent
/// ```
///
// https://drafts.csswg.org/css-lists-3/#marker-side
#[syntax(" match-self | match-parent ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "match-self",
	applies_to = "list items",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.marker-side"))]
#[visit]
pub enum MarkerSideStyleValue {}

// /// Represents the style value for `counter-reset` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#counter-reset).
// ///
// /// The counter-reset and counter-increment CSS properties and the counter() and counters() functions automatically number headings or ordered list items.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <counter-name> <integer>? | <reversed-counter-name> <integer>? ]+ | none
// /// ```
// ///
// // https://drafts.csswg.org/css-lists-3/#counter-reset
// #[syntax(" [ <counter-name> <integer>? | <reversed-counter-name> <integer>? ]+ | none ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "by computed value type",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.counter-reset"))]
// #[visit]
// pub enum CounterResetStyleValue<'a> {}

// /// Represents the style value for `counter-increment` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#counter-increment).
// ///
// /// The counter-reset and counter-increment CSS properties and the counter() and counters() functions automatically number headings or ordered list items.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <counter-name> <integer>? ]+ | none
// /// ```
// ///
// // https://drafts.csswg.org/css-lists-3/#counter-increment
// #[syntax(" [ <counter-name> <integer>? ]+ | none ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "by computed value type",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.counter-increment"))]
// #[visit]
// pub enum CounterIncrementStyleValue<'a> {}

// /// Represents the style value for `counter-set` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#counter-set).
// ///
// /// The counter-set CSS property creates (and optionally sets a value for) a counter, the numbers for a series of headings or ordered list items.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <counter-name> <integer>? ]+ | none
// /// ```
// ///
// // https://drafts.csswg.org/css-lists-3/#counter-set
// #[syntax(" [ <counter-name> <integer>? ]+ | none ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "by computed value type",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.counter-set"))]
// #[visit]
// pub enum CounterSetStyleValue<'a> {}
