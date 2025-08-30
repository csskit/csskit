#![allow(warnings)]
//! CSS Speech Module Level 1
//! https://drafts.csswg.org/css-speech-1/

mod impls;
use impls::*;

// /// Represents the style value for `voice-volume` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-volume).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// silent | [[x-soft | soft | medium | loud | x-loud] || <decibel>]
// /// ```
// ///
// // https://drafts.csswg.org/css-speech-1/#voice-volume
// #[value(" silent | [[x-soft | soft | medium | loud | x-loud] || <decibel>] ")]
// #[initial("medium")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum VoiceVolumeStyleValue {}

/// Represents the style value for `voice-balance` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-balance).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <number> | left | center | right | leftwards | rightwards
/// ```
///
// https://drafts.csswg.org/css-speech-1/#voice-balance
#[value(" <number> | left | center | right | leftwards | rightwards ")]
#[initial("center")]
#[applies_to("all elements")]
#[inherited("yes")]
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
#[value(" auto | never | always ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(edge:80)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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
// #[value(" normal | spell-out || digits || [ literal-punctuation | no-punctuation ] ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(limited)]
// #[versions(safari:11.1,safari_ios:11.3)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum SpeakAsStyleValue {}

/// Represents the style value for `pause-before` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#pause-before).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong
/// ```
///
// https://drafts.csswg.org/css-speech-1/#pause-before
#[value(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
#[initial("none")]
#[applies_to("all elements")]
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
pub enum PauseBeforeStyleValue {}

/// Represents the style value for `pause-after` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#pause-after).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong
/// ```
///
// https://drafts.csswg.org/css-speech-1/#pause-after
#[value(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
#[initial("none")]
#[applies_to("all elements")]
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
pub enum PauseAfterStyleValue {}

/// Represents the style value for `pause` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#pause).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'pause-before'> <'pause-after'>?
/// ```
///
// https://drafts.csswg.org/css-speech-1/#pause
#[value(" <'pause-before'> <'pause-after'>? ")]
#[initial("see individual properties")]
#[applies_to("all elements")]
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
pub struct PauseStyleValue;

/// Represents the style value for `rest-before` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#rest-before).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong
/// ```
///
// https://drafts.csswg.org/css-speech-1/#rest-before
#[value(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
#[initial("none")]
#[applies_to("all elements")]
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
pub enum RestBeforeStyleValue {}

/// Represents the style value for `rest-after` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#rest-after).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong
/// ```
///
// https://drafts.csswg.org/css-speech-1/#rest-after
#[value(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
#[initial("none")]
#[applies_to("all elements")]
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
pub enum RestAfterStyleValue {}

/// Represents the style value for `rest` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#rest).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'rest-before'> <'rest-after'>?
/// ```
///
// https://drafts.csswg.org/css-speech-1/#rest
#[value(" <'rest-before'> <'rest-after'>? ")]
#[initial("see individual properties")]
#[applies_to("all elements")]
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
pub struct RestStyleValue;

/// Represents the style value for `cue-before` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#cue-before).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <uri> <decibel>? | none
/// ```
///
// https://drafts.csswg.org/css-speech-1/#cue-before
#[value(" <uri> <decibel>? | none ")]
#[initial("none")]
#[applies_to("all elements")]
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
pub struct CueBeforeStyleValue;

/// Represents the style value for `cue-after` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#cue-after).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <uri> <decibel>? | none
/// ```
///
// https://drafts.csswg.org/css-speech-1/#cue-after
#[value(" <uri> <decibel>? | none ")]
#[initial("none")]
#[applies_to("all elements")]
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
pub struct CueAfterStyleValue;

/// Represents the style value for `cue` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#cue).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'cue-before'> <'cue-after'>?
/// ```
///
// https://drafts.csswg.org/css-speech-1/#cue
#[value(" <'cue-before'> <'cue-after'>? ")]
#[initial("see individual properties")]
#[applies_to("all elements")]
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
pub struct CueStyleValue;

// /// Represents the style value for `voice-family` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-family).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [[<family-name> | <generic-voice>],]* [<family-name> | <generic-voice>] | preserve
// /// ```
// ///
// // https://drafts.csswg.org/css-speech-1/#voice-family
// #[value(" [[<family-name> | <generic-voice>],]* [<family-name> | <generic-voice>] | preserve ")]
// #[initial("implementation-dependent")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum VoiceFamilyStyleValue {}

// /// Represents the style value for `voice-rate` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-rate).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [normal | x-slow | slow | medium | fast | x-fast] || <percentage [0,∞]>
// /// ```
// ///
// // https://drafts.csswg.org/css-speech-1/#voice-rate
// #[value(" [normal | x-slow | slow | medium | fast | x-fast] || <percentage [0,∞]> ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("refer to default value")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct VoiceRateStyleValue;

// /// Represents the style value for `voice-pitch` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-pitch).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]]
// /// ```
// ///
// // https://drafts.csswg.org/css-speech-1/#voice-pitch
// #[value(
// 	" <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]] "
// )]
// #[initial("medium")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("refer to inherited value")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum VoicePitchStyleValue {}

// /// Represents the style value for `voice-range` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-range).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]]
// /// ```
// ///
// // https://drafts.csswg.org/css-speech-1/#voice-range
// #[value(
// 	" <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]] "
// )]
// #[initial("medium")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("refer to inherited value")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum VoiceRangeStyleValue {}

/// Represents the style value for `voice-stress` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-stress).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | strong | moderate | none | reduced
/// ```
///
// https://drafts.csswg.org/css-speech-1/#voice-stress
#[value(" normal | strong | moderate | none | reduced ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("yes")]
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
pub enum VoiceStressStyleValue {}

/// Represents the style value for `voice-duration` as defined in [css-speech-1](https://drafts.csswg.org/css-speech-1/#voice-duration).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <time [0s,∞]>
/// ```
///
// https://drafts.csswg.org/css-speech-1/#voice-duration
#[value(" auto | <time [0s,∞]> ")]
#[initial("auto")]
#[applies_to("all elements")]
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
pub struct VoiceDurationStyleValue;
