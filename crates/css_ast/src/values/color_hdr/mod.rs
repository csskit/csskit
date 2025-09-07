#![allow(warnings)]
//! CSS Color HDR Module Level 1
//! https://drafts.csswg.org/css-color-hdr-1/

mod impls;
use impls::*;

/// Represents the style value for `dynamic-range-limit` as defined in [css-color-hdr-1](https://drafts.csswg.org/css-color-hdr-1/#dynamic-range-limit).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// standard | no-limit | constrained | <dynamic-range-limit-mix()>
/// ```
///
// https://drafts.csswg.org/css-color-hdr-1/#dynamic-range-limit
#[syntax(" standard | no-limit | constrained | <dynamic-range-limit-mix()> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "no-limit",
	applies_to = "all elements",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by dynamic-range-limit-mix()"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.dynamic-range-limit"))]
#[visit]
pub enum DynamicRangeLimitStyleValue<'a> {}
