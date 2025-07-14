#![allow(warnings)]
//! CSS Transitions Level 2
//! https://drafts.csswg.org/css-transitions-2/

mod impls;
use impls::*;

/// Represents the style value for `transition-property` as defined in [css-transitions-2](https://drafts.csswg.org/css-transitions-2/#transition-property).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <single-transition-property>#
/// ```
///
// https://drafts.csswg.org/css-transitions-2/#transition-property
#[value(" none | <single-transition-property># ")]
#[initial("all")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum TransitionPropertyStyleValue<'a> {}

/// Represents the style value for `transition-duration` as defined in [css-transitions-2](https://drafts.csswg.org/css-transitions-2/#transition-duration).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time [0s,∞]>#
/// ```
///
// https://drafts.csswg.org/css-transitions-2/#transition-duration
#[value(" <time [0s,∞]># ")]
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
pub struct TransitionDurationStyleValue<'a>;

/// Represents the style value for `transition-timing-function` as defined in [css-transitions-2](https://drafts.csswg.org/css-transitions-2/#transition-timing-function).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <easing-function>#
/// ```
///
// https://drafts.csswg.org/css-transitions-2/#transition-timing-function
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
pub struct TransitionTimingFunctionStyleValue<'a>;

/// Represents the style value for `transition-delay` as defined in [css-transitions-2](https://drafts.csswg.org/css-transitions-2/#transition-delay).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time>#
/// ```
///
// https://drafts.csswg.org/css-transitions-2/#transition-delay
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
pub struct TransitionDelayStyleValue<'a>;

/// Represents the style value for `transition` as defined in [css-transitions-2](https://drafts.csswg.org/css-transitions-2/#transition).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-transition>#
/// ```
///
// https://drafts.csswg.org/css-transitions-2/#transition
#[value(" <single-transition># ")]
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
pub struct TransitionStyleValue<'a>;

/// Represents the style value for `transition-behavior` as defined in [css-transitions-2](https://drafts.csswg.org/css-transitions-2/#transition-behavior).
///
/// The transition-behavior: allow-discrete CSS declaration allows transitions for properties whose animation behavior is discrete. Such properties can't be interpolated and swap from their start value to the end value at 50%.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <transition-behavior-value>#
/// ```
///
// https://drafts.csswg.org/css-transitions-2/#transition-behavior
#[value(" <transition-behavior-value># ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:117,chrome_android:117,edge:117,firefox:129,firefox_android:129,safari:17.4,safari_ios:17.4)]
pub struct TransitionBehaviorStyleValue<'a>;
