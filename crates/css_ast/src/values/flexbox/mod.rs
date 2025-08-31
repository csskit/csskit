#![allow(warnings)]
//! CSS Flexible Box Layout Module Level 1
//! https://drafts.csswg.org/css-flexbox-1/

mod impls;
use impls::*;

/// Represents the style value for `flex-direction` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex-direction).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// row | row-reverse | column | column-reverse
/// ```
///
// https://drafts.csswg.org/css-flexbox-1/#flex-direction
#[syntax(" row | row-reverse | column | column-reverse ")]
#[initial("row")]
#[applies_to("flex containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum FlexDirectionStyleValue {}

/// Represents the style value for `flex-wrap` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex-wrap).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// nowrap | wrap | wrap-reverse
/// ```
///
// https://drafts.csswg.org/css-flexbox-1/#flex-wrap
#[syntax(" nowrap | wrap | wrap-reverse ")]
#[initial("nowrap")]
#[applies_to("flex containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum FlexWrapStyleValue {}

/// Represents the style value for `flex-flow` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex-flow).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'flex-direction'> || <'flex-wrap'>
/// ```
///
// https://drafts.csswg.org/css-flexbox-1/#flex-flow
#[syntax(" <'flex-direction'> || <'flex-wrap'> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct FlexFlowStyleValue;

// /// Represents the style value for `flex` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ <'flex-grow'> <'flex-shrink'>? || <'flex-basis'> ]
// /// ```
// ///
// // https://drafts.csswg.org/css-flexbox-1/#flex
// #[syntax(" none | [ <'flex-grow'> <'flex-shrink'>? || <'flex-basis'> ] ")]
// #[initial("0 1 auto")]
// #[applies_to("flex items")]
// #[inherited("no")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum FlexStyleValue {}

/// Represents the style value for `flex-grow` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex-grow).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <number [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-flexbox-1/#flex-grow
#[syntax(" <number [0,∞]> ")]
#[initial("0")]
#[applies_to("flex items")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct FlexGrowStyleValue;

/// Represents the style value for `flex-shrink` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex-shrink).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <number [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-flexbox-1/#flex-shrink
#[syntax(" <number [0,∞]> ")]
#[initial("1")]
#[applies_to("flex items")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("number")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct FlexShrinkStyleValue;

/// Represents the style value for `flex-basis` as defined in [css-flexbox-1](https://drafts.csswg.org/css-flexbox-1/#flex-basis).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// content | <'width'>
/// ```
///
// https://drafts.csswg.org/css-flexbox-1/#flex-basis
#[syntax(" content | <'width'> ")]
#[initial("auto")]
#[applies_to("flex items")]
#[inherited("no")]
#[percentages("relative to the flex container’s inner main size")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum FlexBasisStyleValue {}
