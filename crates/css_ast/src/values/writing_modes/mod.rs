#![allow(warnings)]
//! CSS Writing Modes Module Level 4
//! https://drafts.csswg.org/css-writing-modes-4/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `direction` as defined in [css-writing-modes-4](https://drafts.csswg.org/css-writing-modes-4/#direction).
///
/// The unicode-bidi and direction CSS properties override the Unicode layout algorithm. They are intended for Document Type Definition (DTD) designers. For HTML documents, you should use the dir global HTML attribute and <bdo> HTML element instead.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// ltr | rtl
/// ```
///
// https://drafts.csswg.org/css-writing-modes-4/#direction
#[syntax(" ltr | rtl ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "ltr",
	applies_to = "all elements",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "n/a",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.direction"))]
#[visit]
pub enum DirectionStyleValue {}

/// Represents the style value for `unicode-bidi` as defined in [css-writing-modes-4](https://drafts.csswg.org/css-writing-modes-4/#unicode-bidi).
///
/// The unicode-bidi and direction CSS properties override the Unicode layout algorithm. They are intended for Document Type Definition (DTD) designers. For HTML documents, you should use the dir global HTML attribute and <bdo> HTML element instead.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | embed | isolate | bidi-override | isolate-override | plaintext
/// ```
///
// https://drafts.csswg.org/css-writing-modes-4/#unicode-bidi
#[syntax(" normal | embed | isolate | bidi-override | isolate-override | plaintext ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "all elements, but see prose",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.unicode-bidi"))]
#[visit]
pub enum UnicodeBidiStyleValue {}

/// Represents the style value for `writing-mode` as defined in [css-writing-modes-4](https://drafts.csswg.org/css-writing-modes-4/#writing-mode).
///
/// The writing-mode CSS property sets whether text is laid out horizontally or vertically, and left to right, or right to left.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// horizontal-tb | vertical-rl | vertical-lr | sideways-rl | sideways-lr
/// ```
///
// https://drafts.csswg.org/css-writing-modes-4/#writing-mode
#[syntax(" horizontal-tb | vertical-rl | vertical-lr | sideways-rl | sideways-lr ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "horizontal-tb",
	applies_to = "All elements except table row groups, table column groups, table rows, table columns, ruby base containers, ruby annotation containers",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "n/a",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.writing-mode"))]
#[visit]
pub enum WritingModeStyleValue {}

/// Represents the style value for `text-orientation` as defined in [css-writing-modes-4](https://drafts.csswg.org/css-writing-modes-4/#text-orientation).
///
/// The text-orientation CSS property sets the how text is typeset within a line when the writing mode is vertical.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// mixed | upright | sideways
/// ```
///
// https://drafts.csswg.org/css-writing-modes-4/#text-orientation
#[syntax(" mixed | upright | sideways ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "mixed",
	applies_to = "all elements except table row groups, rows, column groups, and columns; and text",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "n/a",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-orientation"))]
#[visit]
pub enum TextOrientationStyleValue {}

/// Represents the style value for `glyph-orientation-vertical` as defined in [css-writing-modes-4](https://drafts.csswg.org/css-writing-modes-4/#glyph-orientation-vertical).
///
/// The glyph-orientation-vertical CSS property sets the orientation of glyphs in text rendered in a vertical writing mode.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | 0deg | 90deg | 0 | 90
/// ```
///
// https://drafts.csswg.org/css-writing-modes-4/#glyph-orientation-vertical
#[syntax(" auto | 0deg | 90deg | 0 | 90 ")]
#[derive(Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "n/a",
	applies_to = "n/a",
	inherited = "n/a",
	percentages = "n/a",
	canonical_order = "n/a",
	animation_type = "n/a"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(
	feature = "css_feature_data",
	derive(ToCSSFeature),
	css_feature("css.properties.glyph-orientation-vertical")
)]
#[visit]
pub enum GlyphOrientationVerticalStyleValue {}

/// Represents the style value for `text-combine-upright` as defined in [css-writing-modes-4](https://drafts.csswg.org/css-writing-modes-4/#text-combine-upright).
///
/// The text-combine-upright CSS property displays multiple characters in the space of a single character in vertical text. This is used in East Asian documents to display Latin-based strings such as components of a date or letters of an initialism.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | all | [ digits <integer [2,4]>? ]
/// ```
///
// https://drafts.csswg.org/css-writing-modes-4/#text-combine-upright
#[syntax(" none | all | [ digits <integer [2,4]>? ] ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "inline boxes and text",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "n/a",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-combine-upright"))]
#[visit]
pub enum TextCombineUprightStyleValue {}
