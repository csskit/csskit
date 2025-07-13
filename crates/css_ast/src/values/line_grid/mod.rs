#![allow(warnings)]
//! CSS Line Grid Module Level 1
//! https://drafts.csswg.org/css-line-grid-1/

mod impls;
use impls::*;

/// Represents the style value for `line-grid` as defined in [css-line-grid-1](https://drafts.csswg.org/css-line-grid-1/#line-grid).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// match-parent | create
/// ```
///
// https://drafts.csswg.org/css-line-grid-1/#line-grid
#[value(" match-parent | create ")]
#[initial("match-parent")]
#[applies_to("block, flex and grid containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum LineGridStyleValue {}

/// Represents the style value for `line-snap` as defined in [css-line-grid-1](https://drafts.csswg.org/css-line-grid-1/#line-snap).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | baseline | contain
/// ```
///
// https://drafts.csswg.org/css-line-grid-1/#line-snap
#[value(" none | baseline | contain ")]
#[initial("none")]
#[applies_to("block container elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum LineSnapStyleValue {}

/// Represents the style value for `box-snap` as defined in [css-line-grid-1](https://drafts.csswg.org/css-line-grid-1/#box-snap).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | block-start | block-end | center | baseline | last-baseline
/// ```
///
// https://drafts.csswg.org/css-line-grid-1/#box-snap
#[value(" none | block-start | block-end | center | baseline | last-baseline ")]
#[initial("none")]
#[applies_to("block-level boxes and internal table elements except table cells")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum BoxSnapStyleValue {}
