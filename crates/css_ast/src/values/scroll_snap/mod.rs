#![allow(warnings)]
//! CSS Scroll Snap Module Level 2
//! https://drafts.csswg.org/css-scroll-snap-2/

mod impls;
use impls::*;

// /// Represents the style value for `scroll-snap-type` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-type).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ x | y | block | inline | both ] [ mandatory | proximity ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-type
// #[value(" none | [ x | y | block | inline | both ] [ mandatory | proximity ]? ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum ScrollSnapTypeStyleValue {}

/// Represents the style value for `scroll-padding` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ auto | <length-percentage [0,∞]> ]{1,4}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding
#[value(" [ auto | <length-percentage [0,∞]> ]{1,4} ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the corresponding dimension of the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollPaddingStyleValue;

/// Represents the style value for `scroll-margin` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>{1,4}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin
#[value(" <length>{1,4} ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollMarginStyleValue;

/// Represents the style value for `scroll-snap-align` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-align).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ none | start | end | center ]{1,2}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-align
#[value(" [ none | start | end | center ]{1,2} ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollSnapAlignStyleValue;

/// Represents the style value for `scroll-snap-stop` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-stop).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | always
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-snap-stop
#[value(" normal | always ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum ScrollSnapStopStyleValue {}

/// Represents the style value for `scroll-padding-top` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-top).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-top
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollPaddingTopStyleValue;

/// Represents the style value for `scroll-padding-right` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-right).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-right
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollPaddingRightStyleValue;

/// Represents the style value for `scroll-padding-bottom` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-bottom).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-bottom
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollPaddingBottomStyleValue;

/// Represents the style value for `scroll-padding-left` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-left).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-left
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollPaddingLeftStyleValue;

/// Represents the style value for `scroll-padding-inline-start` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline-start
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollPaddingInlineStartStyleValue;

/// Represents the style value for `scroll-padding-block-start` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block-start
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollPaddingBlockStartStyleValue;

/// Represents the style value for `scroll-padding-inline-end` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline-end
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollPaddingInlineEndStyleValue;

/// Represents the style value for `scroll-padding-block-end` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block-end
#[value(" auto | <length-percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollPaddingBlockEndStyleValue;

/// Represents the style value for `scroll-padding-block` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ auto | <length-percentage [0,∞]> ]{1,2}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-block
#[value(" [ auto | <length-percentage [0,∞]> ]{1,2} ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollPaddingBlockStyleValue;

/// Represents the style value for `scroll-padding-inline` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ auto | <length-percentage [0,∞]> ]{1,2}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-padding-inline
#[value(" [ auto | <length-percentage [0,∞]> ]{1,2} ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("relative to the scroll container’s scrollport")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollPaddingInlineStyleValue;

/// Represents the style value for `scroll-margin-top` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-top).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-top
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollMarginTopStyleValue;

/// Represents the style value for `scroll-margin-right` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-right).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-right
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollMarginRightStyleValue;

/// Represents the style value for `scroll-margin-bottom` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-bottom).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-bottom
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollMarginBottomStyleValue;

/// Represents the style value for `scroll-margin-left` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-left).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-left
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollMarginLeftStyleValue;

/// Represents the style value for `scroll-margin-block-start` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block-start
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollMarginBlockStartStyleValue;

/// Represents the style value for `scroll-margin-inline-start` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline-start
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollMarginInlineStartStyleValue;

/// Represents the style value for `scroll-margin-block-end` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block-end
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollMarginBlockEndStyleValue;

/// Represents the style value for `scroll-margin-inline-end` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline-end
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollMarginInlineEndStyleValue;

/// Represents the style value for `scroll-margin-block` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>{1,2}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-block
#[value(" <length>{1,2} ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollMarginBlockStyleValue;

/// Represents the style value for `scroll-margin-inline` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>{1,2}
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-margin-inline
#[value(" <length>{1,2} ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ScrollMarginInlineStyleValue;

/// Represents the style value for `scroll-initial-target` as defined in [css-scroll-snap-2](https://drafts.csswg.org/css-scroll-snap-2/#scroll-initial-target).
///
/// The scroll-initial-target: nearest CSS declaration sets the initial scroll position of its scroll container to the top of the element, much like scrolling to a URL fragment.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | nearest
/// ```
///
// https://drafts.csswg.org/css-scroll-snap-2/#scroll-initial-target
#[value(" none | nearest ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("none")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:133,chrome_android:133,edge:133)]
pub enum ScrollInitialTargetStyleValue {}
