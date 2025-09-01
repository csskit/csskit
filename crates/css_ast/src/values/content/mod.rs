#![allow(warnings)]
//! CSS Generated Content Module Level 3
//! https://drafts.csswg.org/css-content-3/

mod impls;
use impls::*;

// /// Represents the style value for `content` as defined in [css-content-3](https://drafts.csswg.org/css-content-3/#content).
// ///
// /// The content CSS property sets the content inside of an element or pseudo-element, replacing the current value. It's often used with the ::before and ::after pseudo-elements to generate cosmetic content.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | none | [ <content-replacement> | <content-list> ] [/ [ <string> | <counter> | <attr()> ]+ ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-content-3/#content
// #[syntax(" normal | none | [ <content-replacement> | <content-list> ] [/ [ <string> | <counter> | <attr()> ]+ ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "all elements, tree-abiding pseudo-elements, and page margin boxes",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.content"))]
// #[visit]
// pub enum ContentStyleValue<'a> {}

// /// Represents the style value for `quotes` as defined in [css-content-3](https://drafts.csswg.org/css-content-3/#quotes).
// ///
// /// The quotes CSS property sets the quotation marks inserted via the content CSS property or <q> element.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | none | match-parent | [ <string> <string> ]+
// /// ```
// ///
// // https://drafts.csswg.org/css-content-3/#quotes
// #[syntax(" auto | none | match-parent | [ <string> <string> ]+ ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "auto",
//   applies_to = "all elements",
// 	inherited = "yes",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.quotes"))]
// #[visit]
// pub enum QuotesStyleValue<'a> {}

/// Represents the style value for `bookmark-level` as defined in [css-content-3](https://drafts.csswg.org/css-content-3/#bookmark-level).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <integer [1,∞]>
/// ```
///
// https://drafts.csswg.org/css-content-3/#bookmark-level
#[syntax(" none | <integer [1,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.bookmark-level"))]
#[visit]
pub struct BookmarkLevelStyleValue;

/// Represents the style value for `bookmark-label` as defined in [css-content-3](https://drafts.csswg.org/css-content-3/#bookmark-label).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <content-list>
/// ```
///
// https://drafts.csswg.org/css-content-3/#bookmark-label
#[syntax(" <content-list> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "content(text)",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.bookmark-label"))]
#[visit]
pub struct BookmarkLabelStyleValue<'a>;

/// Represents the style value for `bookmark-state` as defined in [css-content-3](https://drafts.csswg.org/css-content-3/#bookmark-state).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// open | closed
/// ```
///
// https://drafts.csswg.org/css-content-3/#bookmark-state
#[syntax(" open | closed ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "open",
	applies_to = "block-level elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.bookmark-state"))]
#[visit]
pub enum BookmarkStateStyleValue {}
