#![allow(warnings)]
//! CSS Round Display Level 1
//! https://drafts.csswg.org/css-round-display-1/

mod impls;
use impls::*;

/// Represents the style value for `border-boundary` as defined in [css-round-display-1](https://drafts.csswg.org/css-round-display-1/#border-boundary).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | parent | display
/// ```
///
// https://drafts.csswg.org/css-round-display-1/#border-boundary
#[value(" none | parent | display ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum BorderBoundaryStyleValue {}
