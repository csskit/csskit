#![allow(warnings)]
//! CSS Animations Level 2
//! https://drafts.csswg.org/css-animations-2/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `animation-name` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-name).
///
/// The animation CSS property animates an element's style over time, using keyframes described in @keyframes rules.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ none | <keyframes-name> ]#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-name
#[syntax(" [ none | <keyframes-name> ]# ")]
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
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-name"))]
#[visit]
pub struct AnimationNameStyleValue<'a>;

/// Represents the style value for `animation-duration` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-duration).
///
/// The animation CSS property animates an element's style over time, using keyframes described in @keyframes rules.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ auto | <time [0s,∞]> ]#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-duration
#[syntax(" [ auto | <time [0s,∞]> ]# ")]
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
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-duration"))]
#[visit]
pub struct AnimationDurationStyleValue<'a>;

/// Represents the style value for `animation-timing-function` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-timing-function).
///
/// The animation CSS property animates an element's style over time, using keyframes described in @keyframes rules.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <easing-function>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-timing-function
#[syntax(" <easing-function># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "ease",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-timing-function"))]
#[visit]
pub struct AnimationTimingFunctionStyleValue<'a>;

/// Represents the style value for `animation-iteration-count` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-iteration-count).
///
/// The animation CSS property animates an element's style over time, using keyframes described in @keyframes rules.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-iteration-count>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-iteration-count
#[syntax(" <single-animation-iteration-count># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "1",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-iteration-count"))]
#[visit]
pub struct AnimationIterationCountStyleValue<'a>;

/// Represents the style value for `animation-direction` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-direction).
///
/// The animation CSS property animates an element's style over time, using keyframes described in @keyframes rules.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-direction>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-direction
#[syntax(" <single-animation-direction># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-direction"))]
#[visit]
pub struct AnimationDirectionStyleValue<'a>;

/// Represents the style value for `animation-play-state` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-play-state).
///
/// The animation CSS property animates an element's style over time, using keyframes described in @keyframes rules.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-play-state>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-play-state
#[syntax(" <single-animation-play-state># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "running",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-play-state"))]
#[visit]
pub struct AnimationPlayStateStyleValue<'a>;

/// Represents the style value for `animation-delay` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-delay).
///
/// The animation CSS property animates an element's style over time, using keyframes described in @keyframes rules.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-delay
#[syntax(" <time># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0s",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-delay"))]
#[visit]
pub struct AnimationDelayStyleValue<'a>;

/// Represents the style value for `animation-fill-mode` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-fill-mode).
///
/// The animation CSS property animates an element's style over time, using keyframes described in @keyframes rules.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-fill-mode>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-fill-mode
#[syntax(" <single-animation-fill-mode># ")]
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
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-fill-mode"))]
#[visit]
pub struct AnimationFillModeStyleValue<'a>;

// /// Represents the style value for `animation` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation).
// ///
// /// The animation CSS property animates an element's style over time, using keyframes described in @keyframes rules.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <single-animation>#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation
// #[syntax(" <single-animation># ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "see individual properties",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation"))]
// #[visit]
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
#[syntax(" <single-animation-composition># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "replace",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-composition"))]
#[visit]
pub struct AnimationCompositionStyleValue<'a>;

/// Represents the style value for `animation-timeline` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-timeline).
///
/// The animation-timeline, scroll-timeline, and view-timeline CSS properties advance animations based on the user's scroll position.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-timeline>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-timeline
#[syntax(" <single-animation-timeline># ")]
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
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-timeline"))]
#[visit]
pub struct AnimationTimelineStyleValue<'a>;

/// Represents the style value for `animation-trigger-behavior` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-behavior).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-trigger-behavior>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-trigger-behavior
#[syntax(" <single-animation-trigger-behavior># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "once",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(
	feature = "css_feature_data",
	derive(ToCSSFeature),
	css_feature("css.properties.animation-trigger-behavior")
)]
#[visit]
pub struct AnimationTriggerBehaviorStyleValue<'a>;

/// Represents the style value for `animation-trigger-timeline` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-timeline).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-timeline>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-trigger-timeline
#[syntax(" <single-animation-timeline># ")]
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
#[cfg_attr(
	feature = "css_feature_data",
	derive(ToCSSFeature),
	css_feature("css.properties.animation-trigger-timeline")
)]
#[visit]
pub struct AnimationTriggerTimelineStyleValue<'a>;

// /// Represents the style value for `animation-trigger-range` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-range).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <'animation-trigger-range-start'> <'animation-trigger-range-end'>? ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-trigger-range
// #[syntax(" [ <'animation-trigger-range-start'> <'animation-trigger-range-end'>? ]# ")]
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
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-trigger-range"))]
// #[visit]
// pub struct AnimationTriggerRangeStyleValue<'a>;

// /// Represents the style value for `animation-trigger-range-start` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-range-start).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-trigger-range-start
// #[syntax(" [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "relative to the specified named timeline range if one was specified, else to the entire timeline",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-trigger-range-start"))]
// #[visit]
// pub struct AnimationTriggerRangeStartStyleValue<'a>;

// /// Represents the style value for `animation-trigger-range-end` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-range-end).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-trigger-range-end
// #[syntax(" [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "normal",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "relative to the specified named timeline range if one was specified, else to the entire timeline",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-trigger-range-end"))]
// #[visit]
// pub struct AnimationTriggerRangeEndStyleValue<'a>;

// /// Represents the style value for `animation-trigger-exit-range` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <'animation-trigger-exit-range-start'> <'animation-trigger-exit-range-end'>? ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range
// #[syntax(" [ <'animation-trigger-exit-range-start'> <'animation-trigger-exit-range-end'>? ]# ")]
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
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-trigger-exit-range"))]
// #[visit]
// pub struct AnimationTriggerExitRangeStyleValue<'a>;

// /// Represents the style value for `animation-trigger-exit-range-start` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range-start).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range-start
// #[syntax(" [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "auto",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "relative to the specified named timeline range if one was specified, else to the entire timeline",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-trigger-exit-range-start"))]
// #[visit]
// pub struct AnimationTriggerExitRangeStartStyleValue<'a>;

// /// Represents the style value for `animation-trigger-exit-range-end` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range-end).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range-end
// #[syntax(" [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "auto",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "relative to the specified named timeline range if one was specified, else to the entire timeline",
// 	canonical_order = "per grammar",
// 	animation_type = "not animatable",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-trigger-exit-range-end"))]
// #[visit]
// pub struct AnimationTriggerExitRangeEndStyleValue<'a>;

/// Represents the style value for `animation-trigger` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-trigger>#
/// ```
///
// https://drafts.csswg.org/css-animations-2/#animation-trigger
#[syntax(" <single-animation-trigger># ")]
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
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-trigger"))]
#[visit]
pub struct AnimationTriggerStyleValue<'a>;
