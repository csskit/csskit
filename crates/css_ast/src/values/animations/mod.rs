#![allow(warnings)]
//! https://drafts.csswg.org/css-animations-2/

mod impls;
use super::prelude::*;
use impls::*;
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
// /// https://drafts.csswg.org/css-animations-2/#animation
// #[syntax(" <single-animation># ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     SemanticEq,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "see individual properties",
//     applies_to = Elements,
//     property_group = Animations,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.animation")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
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
/// https://drafts.csswg.org/css-animations-2/#animation-composition
#[syntax(" <single-animation-composition># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "replace",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-composition"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AnimationCompositionStyleValue<'a>;

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
/// https://drafts.csswg.org/css-animations-2/#animation-delay
#[syntax(" <time># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0s",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-delay"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AnimationDelayStyleValue<'a>;

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
/// https://drafts.csswg.org/css-animations-2/#animation-direction
#[syntax(" <single-animation-direction># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-direction"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AnimationDirectionStyleValue<'a>;

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
/// https://drafts.csswg.org/css-animations-2/#animation-duration
#[syntax(" [ auto | <time [0s,∞]> ]# ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-duration"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AnimationDurationStyleValue<'a>;

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
/// https://drafts.csswg.org/css-animations-2/#animation-fill-mode
#[syntax(" <single-animation-fill-mode># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-fill-mode"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AnimationFillModeStyleValue<'a>;

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
/// https://drafts.csswg.org/css-animations-2/#animation-iteration-count
#[syntax(" <single-animation-iteration-count># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "1",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-iteration-count"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AnimationIterationCountStyleValue<'a>;

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
/// https://drafts.csswg.org/css-animations-2/#animation-name
#[syntax(" [ none | <keyframes-name> ]# ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-name"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AnimationNameStyleValue<'a>;

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
/// https://drafts.csswg.org/css-animations-2/#animation-play-state
#[syntax(" <single-animation-play-state># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "running",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-play-state"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AnimationPlayStateStyleValue<'a>;

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
/// https://drafts.csswg.org/css-animations-2/#animation-timeline
#[syntax(" <single-animation-timeline># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-timeline"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AnimationTimelineStyleValue<'a>;

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
/// https://drafts.csswg.org/css-animations-2/#animation-timing-function
#[syntax(" <easing-function># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "ease",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-timing-function"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AnimationTimingFunctionStyleValue<'a>;

/// Represents the style value for `animation-trigger` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#animation-trigger).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ none | [ <dashed-ident> <animation-action>+ ]+ ]#
/// ```
///
/// https://drafts.csswg.org/css-animations-2/#animation-trigger
#[syntax(" [ none | [ <dashed-ident> <animation-action>+ ]+ ]# ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.animation-trigger"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AnimationTriggerStyleValue<'a>;

/// Represents the style value for `event-trigger` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#event-trigger).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | [ <'event-trigger-name'> <'event-trigger-source'> ]#
/// ```
///
/// https://drafts.csswg.org/css-animations-2/#event-trigger
#[syntax(" none | [ <'event-trigger-name'> <'event-trigger-source'> ]# ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.event-trigger"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct EventTriggerStyleValue<'a>;

/// Represents the style value for `event-trigger-name` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#event-trigger-name).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <dashed-ident>#
/// ```
///
/// https://drafts.csswg.org/css-animations-2/#event-trigger-name
#[syntax(" none | <dashed-ident># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.event-trigger-name"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct EventTriggerNameStyleValue<'a>;

/// Represents the style value for `event-trigger-source` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#event-trigger-source).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ none | <event-trigger-event>+ [ / <event-trigger-event>+ ]? ]#
/// ```
///
/// https://drafts.csswg.org/css-animations-2/#event-trigger-source
#[syntax(" [ none | <event-trigger-event>+ [ / <event-trigger-event>+ ]? ]# ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.event-trigger-source"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct EventTriggerSourceStyleValue<'a>;

// /// Represents the style value for `timeline-trigger` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#timeline-trigger).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ <'timeline-trigger-name'> <'timeline-trigger-source'> <'timeline-trigger-range'> [ '/' <'timeline-trigger-exit-range'> ]? ]#
// /// ```
// ///
// /// https://drafts.csswg.org/css-animations-2/#timeline-trigger
// #[syntax(
//     " none | [ <'timeline-trigger-name'> <'timeline-trigger-source'> <'timeline-trigger-range'> [ '/' <'timeline-trigger-exit-range'> ]? ]# "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     SemanticEq,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "see individual properties",
//     inherits = Unknown,
//     applies_to = Unknown,
//     percentages = Unknown,
//     animation_type = Unknown,
//     property_group = Animations,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.timeline-trigger")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct TimelineTriggerStyleValue<'a>;

// /// Represents the style value for `timeline-trigger-exit-range` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#timeline-trigger-exit-range).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <'timeline-trigger-exit-range-start'> <'timeline-trigger-exit-range-end'>? ]#
// /// ```
// ///
// /// https://drafts.csswg.org/css-animations-2/#timeline-trigger-exit-range
// #[syntax(
//     " [ <'timeline-trigger-exit-range-start'> <'timeline-trigger-exit-range-end'>? ]# "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     SemanticEq,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "see individual properties",
//     inherits = Unknown,
//     applies_to = Unknown,
//     percentages = Unknown,
//     animation_type = Unknown,
//     property_group = Animations,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.timeline-trigger-exit-range")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct TimelineTriggerExitRangeStyleValue<'a>;

// /// Represents the style value for `timeline-trigger-exit-range-end` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#timeline-trigger-exit-range-end).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
// /// ```
// ///
// /// https://drafts.csswg.org/css-animations-2/#timeline-trigger-exit-range-end
// #[syntax(
//     " [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     SemanticEq,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "auto",
//     applies_to = Elements,
//     percentages = Unknown,
//     property_group = Animations,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.timeline-trigger-exit-range-end")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct TimelineTriggerExitRangeEndStyleValue<'a>;

// /// Represents the style value for `timeline-trigger-exit-range-start` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#timeline-trigger-exit-range-start).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
// /// ```
// ///
// /// https://drafts.csswg.org/css-animations-2/#timeline-trigger-exit-range-start
// #[syntax(
//     " [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     SemanticEq,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "auto",
//     applies_to = Elements,
//     percentages = Unknown,
//     property_group = Animations,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.timeline-trigger-exit-range-start")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct TimelineTriggerExitRangeStartStyleValue<'a>;

/// Represents the style value for `timeline-trigger-name` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#timeline-trigger-name).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <dashed-ident>#
/// ```
///
/// https://drafts.csswg.org/css-animations-2/#timeline-trigger-name
#[syntax(" none | <dashed-ident># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.timeline-trigger-name"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct TimelineTriggerNameStyleValue<'a>;

// /// Represents the style value for `timeline-trigger-range` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#timeline-trigger-range).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <'timeline-trigger-range-start'> <'timeline-trigger-range-end'>? ]#
// /// ```
// ///
// /// https://drafts.csswg.org/css-animations-2/#timeline-trigger-range
// #[syntax(" [ <'timeline-trigger-range-start'> <'timeline-trigger-range-end'>? ]# ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     SemanticEq,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "see individual properties",
//     inherits = Unknown,
//     applies_to = Unknown,
//     percentages = Unknown,
//     animation_type = Unknown,
//     property_group = Animations,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.timeline-trigger-range")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct TimelineTriggerRangeStyleValue<'a>;

// /// Represents the style value for `timeline-trigger-range-end` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#timeline-trigger-range-end).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
// /// ```
// ///
// /// https://drafts.csswg.org/css-animations-2/#timeline-trigger-range-end
// #[syntax(
//     " [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     SemanticEq,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "normal",
//     applies_to = Elements,
//     percentages = Unknown,
//     property_group = Animations,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.timeline-trigger-range-end")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct TimelineTriggerRangeEndStyleValue<'a>;

// /// Represents the style value for `timeline-trigger-range-start` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#timeline-trigger-range-start).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
// /// ```
// ///
// /// https://drafts.csswg.org/css-animations-2/#timeline-trigger-range-start
// #[syntax(
//     " [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     SemanticEq,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "normal",
//     applies_to = Elements,
//     percentages = Unknown,
//     property_group = Animations,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.timeline-trigger-range-start")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct TimelineTriggerRangeStartStyleValue<'a>;

/// Represents the style value for `timeline-trigger-source` as defined in [css-animations-2](https://drafts.csswg.org/css-animations-2/#timeline-trigger-source).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-timeline>#
/// ```
///
/// https://drafts.csswg.org/css-animations-2/#timeline-trigger-source
#[syntax(" <single-animation-timeline># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.timeline-trigger-source"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct TimelineTriggerSourceStyleValue<'a>;
