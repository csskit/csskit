#![allow(warnings)]
//! CSS Animations Level 2
//! https://drafts.csswg.org/css-animations-2/

mod impls;
use impls::*;

// /// Represents the style value for `animation-name` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-name).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ none | <keyframes-name> ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-name
// #[value(" [ none | <keyframes-name> ]# ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct AnimationNameStyleValue<'a>;

/// Represents the style value for `animation-duration` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-duration).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ auto | <time [0s,∞]> ]#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-duration
#[value(" [ auto | <time [0s,∞]> ]# ")]
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
pub struct AnimationDurationStyleValue<'a>;

/// Represents the style value for `animation-timing-function` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-timing-function).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <easing-function>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-timing-function
#[value(" <easing-function># ")]
#[initial("ease")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct AnimationTimingFunctionStyleValue<'a>;

/// Represents the style value for `animation-iteration-count` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-iteration-count).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-iteration-count>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-iteration-count
#[value(" <single-animation-iteration-count># ")]
#[initial("1")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct AnimationIterationCountStyleValue<'a>;

/// Represents the style value for `animation-direction` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-direction).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-direction>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-direction
#[value(" <single-animation-direction># ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct AnimationDirectionStyleValue<'a>;

/// Represents the style value for `animation-play-state` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-play-state).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-play-state>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-play-state
#[value(" <single-animation-play-state># ")]
#[initial("running")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct AnimationPlayStateStyleValue<'a>;

/// Represents the style value for `animation-delay` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-delay).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-delay
#[value(" <time># ")]
#[initial("0s")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct AnimationDelayStyleValue<'a>;

/// Represents the style value for `animation-fill-mode` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-fill-mode).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-fill-mode>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-fill-mode
#[value(" <single-animation-fill-mode># ")]
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
pub struct AnimationFillModeStyleValue<'a>;

// /// Represents the style value for `animation` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <single-animation>#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation
// #[value(" <single-animation># ")]
// #[initial("see individual properties")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct AnimationStyleValue<'a>;

/// Represents the style value for `animation-composition` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-composition).
///
/// The animation-composition CSS property chooses how to combine animations that affect the same property.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-composition>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-composition
#[value(" <single-animation-composition># ")]
#[initial("replace")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:112,chrome_android:112,edge:112,firefox:115,firefox_android:115,safari:16,safari_ios:16)]
pub struct AnimationCompositionStyleValue<'a>;

/// Represents the style value for `animation-timeline` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-timeline).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-timeline>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-timeline
#[value(" <single-animation-timeline># ")]
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
pub struct AnimationTimelineStyleValue<'a>;

/// Represents the style value for `animation-trigger-behavior` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-behavior).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-trigger-behavior>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-trigger-behavior
#[value(" <single-animation-trigger-behavior># ")]
#[initial("once")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct AnimationTriggerBehaviorStyleValue<'a>;

/// Represents the style value for `animation-trigger-timeline` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-timeline).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-timeline>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-trigger-timeline
#[value(" <single-animation-timeline># ")]
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
pub struct AnimationTriggerTimelineStyleValue<'a>;

// /// Represents the style value for `animation-trigger-range` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-range).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <'animation-trigger-range-start'> <'animation-trigger-range-end'>? ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-trigger-range
// #[value(" [ <'animation-trigger-range-start'> <'animation-trigger-range-end'>? ]# ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct AnimationTriggerRangeStyleValue<'a>;

// /// Represents the style value for `animation-trigger-range-start` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-range-start).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-trigger-range-start
// #[value(" [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("relative to the specified named timeline range if one was specified, else to the entire timeline")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct AnimationTriggerRangeStartStyleValue<'a>;

// /// Represents the style value for `animation-trigger-range-end` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-range-end).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-trigger-range-end
// #[value(" [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("relative to the specified named timeline range if one was specified, else to the entire timeline")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct AnimationTriggerRangeEndStyleValue<'a>;

// /// Represents the style value for `animation-trigger-exit-range` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <'animation-trigger-exit-range-start'> <'animation-trigger-exit-range-end'>? ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range
// #[value(" [ <'animation-trigger-exit-range-start'> <'animation-trigger-exit-range-end'>? ]# ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct AnimationTriggerExitRangeStyleValue<'a>;

// /// Represents the style value for `animation-trigger-exit-range-start` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range-start).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range-start
// #[value(" [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("relative to the specified named timeline range if one was specified, else to the entire timeline")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct AnimationTriggerExitRangeStartStyleValue<'a>;

// /// Represents the style value for `animation-trigger-exit-range-end` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range-end).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range-end
// #[value(" [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("relative to the specified named timeline range if one was specified, else to the entire timeline")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct AnimationTriggerExitRangeEndStyleValue<'a>;

/// Represents the style value for `animation-trigger` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-trigger>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-trigger
#[value(" <single-animation-trigger># ")]
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
pub struct AnimationTriggerStyleValue<'a>;
