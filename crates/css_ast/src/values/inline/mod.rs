#![allow(warnings)]
//! CSS Inline Layout Module Level 3
//! https://drafts.csswg.org/css-inline-3/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `dominant-baseline` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#dominant-baseline).
///
/// The dominant-baseline CSS property sets the specific baseline used to align an elements's text and inline-level contents.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | text-bottom | alphabetic | ideographic | middle | central | mathematical | hanging | text-top
/// ```
///
// https://drafts.csswg.org/css-inline-3/#dominant-baseline
#[syntax(" auto | text-bottom | alphabetic | ideographic | middle | central | mathematical | hanging | text-top ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "block containers, inline boxes, table rows, grid containers, flex containers, and SVG text content elements",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.dominant-baseline"))]
#[visit]
pub enum DominantBaselineStyleValue {}

// /// Represents the style value for `vertical-align` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#vertical-align).
// ///
// /// The vertical-align CSS property sets the vertical alignment of inline, inline-block, and table cell elements. It has no effect on block-level elements.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ first | last] || <'alignment-baseline'> || <'baseline-shift'>
// /// ```
// ///
// // https://drafts.csswg.org/css-inline-3/#vertical-align
// #[syntax(" [ first | last] || <'alignment-baseline'> || <'baseline-shift'> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "baseline",
//   applies_to = "see individual properties",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.vertical-align"))]
// #[visit]
// pub struct VerticalAlignStyleValue;

/// Represents the style value for `baseline-source` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#baseline-source).
///
/// The baseline-source CSS property controls how inline-level boxes with multiple lines of text are aligned with the surrounding text. By default, which typographic baseline is used depends on the display property value.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | first | last
/// ```
///
// https://drafts.csswg.org/css-inline-3/#baseline-source
#[syntax(" auto | first | last ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "inline-level boxes",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.baseline-source"))]
#[visit]
pub enum BaselineSourceStyleValue {}

/// Represents the style value for `alignment-baseline` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#alignment-baseline).
///
/// The alignment-baseline CSS property sets which baseline of an element is aligned with the corresponding baseline of its parent.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// baseline | text-bottom | alphabetic | ideographic | middle | central | mathematical | text-top
/// ```
///
// https://drafts.csswg.org/css-inline-3/#alignment-baseline
#[syntax(" baseline | text-bottom | alphabetic | ideographic | middle | central | mathematical | text-top ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "baseline",
	applies_to = "inline-level boxes, flex items, grid items, table cells, and SVG text content elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.alignment-baseline"))]
#[visit]
pub enum AlignmentBaselineStyleValue {}

/// Represents the style value for `baseline-shift` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#baseline-shift).
///
/// The baseline-shift CSS property sets the position of an element relative to its dominant baseline.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage> | sub | super | top | center | bottom
/// ```
///
// https://drafts.csswg.org/css-inline-3/#baseline-shift
#[syntax(" <length-percentage> | sub | super | top | center | bottom ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "inline-level boxes and SVG text content elements",
	inherited = "no",
	percentages = "refer to the used value of line-height",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.baseline-shift"))]
#[visit]
pub enum BaselineShiftStyleValue {}

/// Represents the style value for `line-height` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#line-height).
///
/// The line-height CSS property sets the spacing between text baselines, oriented to the horizontal or vertical writing mode.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <number [0,∞]> | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-inline-3/#line-height
#[syntax(" normal | <number [0,∞]> | <length-percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "non-replaced inline boxes and SVG text content elements",
	inherited = "yes",
	percentages = "computed relative to 1em",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.line-height"))]
#[visit]
pub enum LineHeightStyleValue {}

/// Represents the style value for `line-fit-edge` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#line-fit-edge).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// leading | <text-edge>
/// ```
///
// https://drafts.csswg.org/css-inline-3/#line-fit-edge
#[syntax(" leading | <text-edge> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "leading",
	applies_to = "inline boxes",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.line-fit-edge"))]
#[visit]
pub enum LineFitEdgeStyleValue {}

// /// Represents the style value for `text-box` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#text-box).
// ///
// /// The text-box CSS property sets the spacing above and below text based on a font's typographic features. For example, text-box: trim-both ex alphabetic trims the top to the top of the letter x and the bottom to the bottom of most letters, without descenders.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | <'text-box-trim'> || <'text-box-edge'>
// /// ```
// ///
// // https://drafts.csswg.org/css-inline-3/#text-box
// #[syntax(" normal | <'text-box-trim'> || <'text-box-edge'> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "block containers, multi-column containers, and inline boxes",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-box"))]
// #[visit]
// pub enum TextBoxStyleValue {}

/// Represents the style value for `text-box-trim` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#text-box-trim).
///
/// The text-box CSS property sets the spacing above and below text based on a font's typographic features. For example, text-box: trim-both ex alphabetic trims the top to the top of the letter x and the bottom to the bottom of most letters, without descenders.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | trim-start | trim-end | trim-both
/// ```
///
// https://drafts.csswg.org/css-inline-3/#text-box-trim
#[syntax(" none | trim-start | trim-end | trim-both ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "block containers, multi-column containers, and inline boxes",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-box-trim"))]
#[visit]
pub enum TextBoxTrimStyleValue {}

/// Represents the style value for `text-box-edge` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#text-box-edge).
///
/// The text-box CSS property sets the spacing above and below text based on a font's typographic features. For example, text-box: trim-both ex alphabetic trims the top to the top of the letter x and the bottom to the bottom of most letters, without descenders.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <text-edge>
/// ```
///
// https://drafts.csswg.org/css-inline-3/#text-box-edge
#[syntax(" auto | <text-edge> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "block containers and inline boxes",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-box-edge"))]
#[visit]
pub struct TextBoxEdgeStyleValue;

/// Represents the style value for `inline-sizing` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#inline-sizing).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | stretch
/// ```
///
// https://drafts.csswg.org/css-inline-3/#inline-sizing
#[syntax(" normal | stretch ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "inline boxes, but not ruby container boxes nor internal ruby boxes",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.inline-sizing"))]
#[visit]
pub enum InlineSizingStyleValue {}

// /// Represents the style value for `initial-letter` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#initial-letter).
// ///
// /// The initial-letter CSS property sets the number of lines the first letter of an element occupies. You can use the property to make a raised capital or drop cap.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | <number [1,∞]> <integer [1,∞]> | <number [1,∞]> && [ drop | raise ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-inline-3/#initial-letter
// #[syntax(" normal | <number [1,∞]> <integer [1,∞]> | <number [1,∞]> && [ drop | raise ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "certain inline-level boxes and ::first-letter and inside ::marker boxes (see prose)",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "by computed value type",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.initial-letter"))]
// #[visit]
// pub enum InitialLetterStyleValue {}

// /// Represents the style value for `initial-letter-align` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#initial-letter-align).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ border-box? [ alphabetic | ideographic | hanging | leading ]? ]!
// /// ```
// ///
// // https://drafts.csswg.org/css-inline-3/#initial-letter-align
// #[syntax(" [ border-box? [ alphabetic | ideographic | hanging | leading ]? ]! ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "alphabetic",
//   applies_to = "certain inline-level boxes and ::first-letter and inside ::marker boxes (see prose)",
// 	inherited = "yes",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.initial-letter-align"))]
// #[visit]
// pub struct InitialLetterAlignStyleValue;

/// Represents the style value for `initial-letter-wrap` as defined in [css-inline-3](https://drafts.csswg.org/css-inline-3/#initial-letter-wrap).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | first | all | grid | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-inline-3/#initial-letter-wrap
#[syntax(" none | first | all | grid | <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "certain inline-level boxes and ::first-letter and inside ::marker boxes (see prose)",
	inherited = "yes",
	percentages = "relative to logical width of (last fragment of) initial letter",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.initial-letter-wrap"))]
#[visit]
pub enum InitialLetterWrapStyleValue {}
