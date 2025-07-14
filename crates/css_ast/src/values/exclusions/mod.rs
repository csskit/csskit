#![allow(warnings)]
//! CSS Exclusions Module Level 1
//! https://drafts.csswg.org/css-exclusions-1/

mod impls;
use impls::*;

/// Represents the style value for `wrap-flow` as defined in [css-exclusions-1](https://drafts.csswg.org/css-exclusions-1/#wrap-flow).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | both | start | end | minimum | maximum | clear
/// ```
///
// https://drafts.csswg.org/css-exclusions-1/#wrap-flow
#[value(" auto | both | start | end | minimum | maximum | clear ")]
#[initial("auto")]
#[applies_to("block-level elements.")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum WrapFlowStyleValue {}

/// Represents the style value for `wrap-through` as defined in [css-exclusions-1](https://drafts.csswg.org/css-exclusions-1/#wrap-through).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// wrap | none
/// ```
///
// https://drafts.csswg.org/css-exclusions-1/#wrap-through
#[value(" wrap | none ")]
#[initial("wrap")]
#[applies_to("block-level elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum WrapThroughStyleValue {}
