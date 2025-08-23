#![allow(warnings)]
//! CSS Page Floats
//! https://drafts.csswg.org/css-page-floats-3/

mod impls;
use impls::*;

/// Represents the style value for `float-reference` as defined in [css-page-floats-3](https://drafts.csswg.org/css-page-floats-3/#float-reference).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// inline | column | region | page
/// ```
///
// https://drafts.csswg.org/css-page-floats-3/#float-reference
#[value(" inline | column | region | page ")]
#[initial("inline")]
#[applies_to("all elements.")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum FloatReferenceStyleValue {}

/// Represents the style value for `float` as defined in [css-page-floats-3](https://drafts.csswg.org/css-page-floats-3/#float).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// block-start | block-end | inline-start | inline-end | snap-block | <snap-block()> | snap-inline | <snap-inline()> | left | right | top | bottom | none
/// ```
///
// https://drafts.csswg.org/css-page-floats-3/#float
#[value(
	" block-start | block-end | inline-start | inline-end | snap-block | <snap-block()> | snap-inline | <snap-inline()> | left | right | top | bottom | none "
)]
#[initial("none")]
#[applies_to("all elements.")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum FloatStyleValue {}

/// Represents the style value for `clear` as defined in [css-page-floats-3](https://drafts.csswg.org/css-page-floats-3/#clear).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// inline-start | inline-end | block-start | block-end | left | right | top | bottom | both-inline | both-block | both | none
/// ```
///
// https://drafts.csswg.org/css-page-floats-3/#clear
#[value(
	" inline-start | inline-end | block-start | block-end | left | right | top | bottom | both-inline | both-block | both | none "
)]
#[initial("none")]
#[applies_to("block-level elements, floats, regions, pages")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum ClearStyleValue {}

/// Represents the style value for `float-defer` as defined in [css-page-floats-3](https://drafts.csswg.org/css-page-floats-3/#float-defer).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <integer> | last | none
/// ```
///
// https://drafts.csswg.org/css-page-floats-3/#float-defer
#[value(" <integer> | last | none ")]
#[initial("none")]
#[applies_to("floats")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum FloatDeferStyleValue {}

/// Represents the style value for `float-offset` as defined in [css-page-floats-3](https://drafts.csswg.org/css-page-floats-3/#float-offset).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-page-floats-3/#float-offset
#[value(" <length-percentage> ")]
#[initial("0")]
#[applies_to("floats")]
#[inherited("no")]
#[percentages("see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct FloatOffsetStyleValue;
