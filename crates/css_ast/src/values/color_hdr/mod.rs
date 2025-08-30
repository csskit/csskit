#![allow(warnings)]
//! CSS Color HDR Module Level 1
//! https://drafts.csswg.org/css-color-hdr-1/

mod impls;
use impls::*;

/// Represents the style value for `dynamic-range-limit` as defined in [css-color-hdr-1](https://drafts.csswg.org/css-color-hdr-1/#dynamic-range-limit).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// standard | no-limit | constrained | <dynamic-range-limit-mix()>
/// ```
///
// https://drafts.csswg.org/css-color-hdr-1/#dynamic-range-limit
#[value(" standard | no-limit | constrained | <dynamic-range-limit-mix()> ")]
#[initial("no-limit")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by dynamic-range-limit-mix()")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum DynamicRangeLimitStyleValue<'a> {}
