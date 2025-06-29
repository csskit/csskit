mod impls;
use impls::*;

/*
 * https://drafts.csswg.org/css-animations-2/
 * CSS Animations Level 2
 */

// // https://drafts.csswg.org/css-animations-2/#animation-name
// #[value(" [ none | <keyframes-name> ]# ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum AnimationNameStyleValue<'a> {}

// // https://drafts.csswg.org/css-animations-2/#animation-duration
// #[value(" [ auto | <time [0s,∞]> ]# ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum AnimationDurationStyleValue<'a> {}

// // https://drafts.csswg.org/css-animations-2/#animation-timing-function
// #[value(" <easing-function># ")]
// #[initial("ease")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct AnimationTimingFunctionStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-iteration-count
#[value(" <single-animation-iteration-count># ")]
#[initial("1")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationIterationCountStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-direction
#[value(" <single-animation-direction># ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationDirectionStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-play-state
#[value(" <single-animation-play-state># ")]
#[initial("running")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationPlayStateStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-delay
#[value(" <time># ")]
#[initial("0s")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationDelayStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-fill-mode
#[value(" <single-animation-fill-mode># ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationFillModeStyleValue<'a>;

// // https://drafts.csswg.org/css-animations-2/#animation
// #[value(" <single-animation># ")]
// #[initial("see individual properties")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct AnimationStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-composition
#[value(" <single-animation-composition># ")]
#[initial("replace")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationCompositionStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-timeline
#[value(" <single-animation-timeline># ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationTimelineStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-trigger-behavior
#[value(" <single-animation-trigger-behavior># ")]
#[initial("once")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationTriggerBehaviorStyleValue<'a>;

// https://drafts.csswg.org/css-animations-2/#animation-trigger-timeline
#[value(" <single-animation-timeline># ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationTriggerTimelineStyleValue<'a>;

// // https://drafts.csswg.org/css-animations-2/#animation-trigger-range
// #[value(" [ <'animation-trigger-range-start'> <'animation-trigger-range-end'>? ]# ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct AnimationTriggerRangeStyleValue<'a>;

// // https://drafts.csswg.org/css-animations-2/#animation-trigger-range-start
// #[value(" [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("relative to the specified named timeline range if one was specified, else to the entire timeline")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum AnimationTriggerRangeStartStyleValue<'a> {}

// // https://drafts.csswg.org/css-animations-2/#animation-trigger-range-end
// #[value(" [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("relative to the specified named timeline range if one was specified, else to the entire timeline")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum AnimationTriggerRangeEndStyleValue<'a> {}

// // https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range
// #[value(" [ <'animation-trigger-exit-range-start'> <'animation-trigger-exit-range-end'>? ]# ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct AnimationTriggerExitRangeStyleValue<'a>;

// // https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range-start
// #[value(" [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("relative to the specified named timeline range if one was specified, else to the entire timeline")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum AnimationTriggerExitRangeStartStyleValue<'a> {}

// // https://drafts.csswg.org/css-animations-2/#animation-trigger-exit-range-end
// #[value(" [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]# ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("relative to the specified named timeline range if one was specified, else to the entire timeline")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum AnimationTriggerExitRangeEndStyleValue<'a> {}

// https://drafts.csswg.org/css-animations-2/#animation-trigger
#[value(" <single-animation-trigger># ")]
#[initial("see individual properties")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct AnimationTriggerStyleValue<'a>;
