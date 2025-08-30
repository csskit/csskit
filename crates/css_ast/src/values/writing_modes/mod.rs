#![allow(warnings)]
//! CSS Writing Modes Level 4
//! https://drafts.csswg.org/css-writing-modes-4/

mod impls;
use impls::*;

/// Represents the style value for `direction` as defined in [css-writing-modes-4](https://drafts.csswg.org/css-writing-modes-4/#direction).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// ltr | rtl
/// ```
///
// https://drafts.csswg.org/css-writing-modes-4/#direction
#[value(" ltr | rtl ")]
#[initial("ltr")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum DirectionStyleValue {}

/// Represents the style value for `unicode-bidi` as defined in [css-writing-modes-4](https://drafts.csswg.org/css-writing-modes-4/#unicode-bidi).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | embed | isolate | bidi-override | isolate-override | plaintext
/// ```
///
// https://drafts.csswg.org/css-writing-modes-4/#unicode-bidi
#[value(" normal | embed | isolate | bidi-override | isolate-override | plaintext ")]
#[initial("normal")]
#[applies_to("all elements, but see prose")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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
#[value(" horizontal-tb | vertical-rl | vertical-lr | sideways-rl | sideways-lr ")]
#[initial("horizontal-tb")]
#[applies_to(
	"All elements except table row groups, table column groups, table rows, table columns, ruby base containers, ruby annotation containers"
)]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-writing-mode")]
#[baseline(widely)]
#[versions(chrome:48,chrome_android:48,edge:12,firefox:41,firefox_android:41,safari:10.1,safari_ios:10.3)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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
#[value(" mixed | upright | sideways ")]
#[initial("mixed")]
#[applies_to("all elements except table row groups, rows, column groups, and columns; and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-text-orientation")]
#[baseline(widely)]
#[versions(chrome:48,chrome_android:48,edge:79,firefox:41,firefox_android:41,safari:14,safari_ios:14)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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
#[value(" auto | 0deg | 90deg | 0 | 90 ")]
#[initial("n/a")]
#[applies_to("n/a")]
#[inherited("n/a")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("n/a")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(safari:4,safari_ios:3.2)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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
#[value(" none | all | [ digits <integer [2,4]>? ] ")]
#[initial("none")]
#[applies_to("inline boxes and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:48,chrome_android:48,edge:79,firefox:48,firefox_android:48,safari:15.4,safari_ios:15.4)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum TextCombineUprightStyleValue {}
