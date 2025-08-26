#![allow(warnings)]
//! CSS Rhythmic Sizing
//! https://drafts.csswg.org/css-rhythm-1/

mod impls;
use impls::*;

/// Represents the style value for `block-step-size` as defined in [css-rhythm-1](https://drafts.csswg.org/css-rhythm-1/#block-step-size).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-rhythm-1/#block-step-size
#[value(" none | <length [0,∞]> ")]
#[initial("none")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct BlockStepSizeStyleValue;

/// Represents the style value for `block-step-insert` as defined in [css-rhythm-1](https://drafts.csswg.org/css-rhythm-1/#block-step-insert).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// margin-box | padding-box | content-box
/// ```
///
// https://drafts.csswg.org/css-rhythm-1/#block-step-insert
#[value(" margin-box | padding-box | content-box ")]
#[initial("margin-box")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum BlockStepInsertStyleValue {}

/// Represents the style value for `block-step-align` as defined in [css-rhythm-1](https://drafts.csswg.org/css-rhythm-1/#block-step-align).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | center | start | end
/// ```
///
// https://drafts.csswg.org/css-rhythm-1/#block-step-align
#[value(" auto | center | start | end ")]
#[initial("auto")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum BlockStepAlignStyleValue {}

/// Represents the style value for `block-step-round` as defined in [css-rhythm-1](https://drafts.csswg.org/css-rhythm-1/#block-step-round).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// up | down | nearest
/// ```
///
// https://drafts.csswg.org/css-rhythm-1/#block-step-round
#[value(" up | down | nearest ")]
#[initial("up")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum BlockStepRoundStyleValue {}

/// Represents the style value for `block-step` as defined in [css-rhythm-1](https://drafts.csswg.org/css-rhythm-1/#block-step).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'block-step-size'> || <'block-step-insert'> || <'block-step-align'> || <'block-step-round'>
/// ```
///
// https://drafts.csswg.org/css-rhythm-1/#block-step
#[value(" <'block-step-size'> || <'block-step-insert'> || <'block-step-align'> || <'block-step-round'> ")]
#[initial("see individual properties")]
#[applies_to("block-level boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct BlockStepStyleValue;

/// Represents the style value for `line-height-step` as defined in [css-rhythm-1](https://drafts.csswg.org/css-rhythm-1/#line-height-step).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-rhythm-1/#line-height-step
#[value(" <length [0,∞]> ")]
#[initial("0")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct LineHeightStepStyleValue;
