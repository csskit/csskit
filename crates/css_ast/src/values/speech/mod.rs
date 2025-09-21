#![allow(warnings)]
//! CSS Speech Module Level 1
//! https://drafts.csswg.org/css-speech-1/

mod impls;

use super::prelude::*;
use impls::*;

// /// Represents the style value for `voice-volume` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-volume).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// silent | [[x-soft | soft | medium | loud | x-loud] || <decibel>]
// /// ```
// ///
// // https://drafts.csswg.org/css-speech-1/#voice-volume
// #[syntax(" silent | [[x-soft | soft | medium | loud | x-loud] || <decibel>] ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "medium",
//   applies_to = "all elements",
// 	inherited = "yes",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.voice-volume"))]
// #[visit]
// pub enum VoiceVolumeStyleValue {}

/// Represents the style value for `voice-balance` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-balance).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <number> | left | center | right | leftwards | rightwards
/// ```
///
// https://drafts.csswg.org/css-speech-1/#voice-balance
#[syntax(" <number> | left | center | right | leftwards | rightwards ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "center",
	applies_to = "all elements",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.voice-balance"))]
#[visit]
pub enum VoiceBalanceStyleValue {}

/// Represents the style value for `speak` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#speak).
///
/// The speak CSS property sets whether or not text should be spoken.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | never | always
/// ```
///
// https://drafts.csswg.org/css-speech-1/#speak
#[syntax(" auto | never | always ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.speak"))]
#[visit]
pub enum SpeakStyleValue {}

// /// Represents the style value for `speak-as` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#speak-as).
// ///
// /// The speak-as CSS property sets how any element's content is spoken. Not to be confused with the speak-as descriptor of @counter-style at-rules.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | spell-out || digits || [ literal-punctuation | no-punctuation ]
// /// ```
// ///
// // https://drafts.csswg.org/css-speech-1/#speak-as
// #[syntax(" normal | spell-out || digits || [ literal-punctuation | no-punctuation ] ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "all elements",
// 	inherited = "yes",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.speak-as"))]
// #[visit]
// pub enum SpeakAsStyleValue {}

/// Represents the style value for `pause-before` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#pause-before).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong
/// ```
///
// https://drafts.csswg.org/css-speech-1/#pause-before
#[syntax(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
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
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.pause-before"))]
#[visit]
pub enum PauseBeforeStyleValue {}

/// Represents the style value for `pause-after` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#pause-after).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong
/// ```
///
// https://drafts.csswg.org/css-speech-1/#pause-after
#[syntax(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
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
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.pause-after"))]
#[visit]
pub enum PauseAfterStyleValue {}

/// Represents the style value for `pause` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#pause).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'pause-before'> <'pause-after'>?
/// ```
///
// https://drafts.csswg.org/css-speech-1/#pause
#[syntax(" <'pause-before'> <'pause-after'>? ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.pause"))]
#[visit]
pub struct PauseStyleValue;

/// Represents the style value for `rest-before` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#rest-before).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong
/// ```
///
// https://drafts.csswg.org/css-speech-1/#rest-before
#[syntax(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
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
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.rest-before"))]
#[visit]
pub enum RestBeforeStyleValue {}

/// Represents the style value for `rest-after` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#rest-after).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong
/// ```
///
// https://drafts.csswg.org/css-speech-1/#rest-after
#[syntax(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
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
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.rest-after"))]
#[visit]
pub enum RestAfterStyleValue {}

/// Represents the style value for `rest` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#rest).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'rest-before'> <'rest-after'>?
/// ```
///
// https://drafts.csswg.org/css-speech-1/#rest
#[syntax(" <'rest-before'> <'rest-after'>? ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.rest"))]
#[visit]
pub struct RestStyleValue;

/// Represents the style value for `cue-before` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#cue-before).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <uri> <decibel>? | none
/// ```
///
// https://drafts.csswg.org/css-speech-1/#cue-before
#[syntax(" <uri> <decibel>? | none ")]
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
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.cue-before"))]
#[visit]
pub struct CueBeforeStyleValue;

/// Represents the style value for `cue-after` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#cue-after).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <uri> <decibel>? | none
/// ```
///
// https://drafts.csswg.org/css-speech-1/#cue-after
#[syntax(" <uri> <decibel>? | none ")]
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
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.cue-after"))]
#[visit]
pub struct CueAfterStyleValue;

/// Represents the style value for `cue` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#cue).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'cue-before'> <'cue-after'>?
/// ```
///
// https://drafts.csswg.org/css-speech-1/#cue
#[syntax(" <'cue-before'> <'cue-after'>? ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.cue"))]
#[visit]
pub struct CueStyleValue;

// /// Represents the style value for `voice-family` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-family).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [[<family-name> | <generic-voice>],]* [<family-name> | <generic-voice>] | preserve
// /// ```
// ///
// // https://drafts.csswg.org/css-speech-1/#voice-family
// #[syntax(" [[<family-name> | <generic-voice>],]* [<family-name> | <generic-voice>] | preserve ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "implementation-dependent",
//   applies_to = "all elements",
// 	inherited = "yes",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.voice-family"))]
// #[visit]
// pub enum VoiceFamilyStyleValue {}

// /// Represents the style value for `voice-rate` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-rate).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [normal | x-slow | slow | medium | fast | x-fast] || <percentage [0,∞]>
// /// ```
// ///
// // https://drafts.csswg.org/css-speech-1/#voice-rate
// #[syntax(" [normal | x-slow | slow | medium | fast | x-fast] || <percentage [0,∞]> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "all elements",
// 	inherited = "yes",
// 	percentages = "refer to default value",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.voice-rate"))]
// #[visit]
// pub struct VoiceRateStyleValue;

// /// Represents the style value for `voice-pitch` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-pitch).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]]
// /// ```
// ///
// // https://drafts.csswg.org/css-speech-1/#voice-pitch
// #[syntax(
// 	" <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]] "
// )]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "medium",
//   applies_to = "all elements",
// 	inherited = "yes",
// 	percentages = "refer to inherited value",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.voice-pitch"))]
// #[visit]
// pub enum VoicePitchStyleValue {}

// /// Represents the style value for `voice-range` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-range).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]]
// /// ```
// ///
// // https://drafts.csswg.org/css-speech-1/#voice-range
// #[syntax(
// 	" <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]] "
// )]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "medium",
//   applies_to = "all elements",
// 	inherited = "yes",
// 	percentages = "refer to inherited value",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.voice-range"))]
// #[visit]
// pub enum VoiceRangeStyleValue {}

/// Represents the style value for `voice-stress` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-stress).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | strong | moderate | none | reduced
/// ```
///
// https://drafts.csswg.org/css-speech-1/#voice-stress
#[syntax(" normal | strong | moderate | none | reduced ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "all elements",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.voice-stress"))]
#[visit]
pub enum VoiceStressStyleValue {}

/// Represents the style value for `voice-duration` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-duration).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <time [0s,∞]>
/// ```
///
// https://drafts.csswg.org/css-speech-1/#voice-duration
#[syntax(" auto | <time [0s,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.voice-duration"))]
#[visit]
pub struct VoiceDurationStyleValue;
