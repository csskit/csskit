mod impls;
use impls::*;

/*
 * https://drafts.csswg.org/css-color-hdr-1/
 * CSS Color HDR Module Level 1
 */

// https://drafts.csswg.org/css-color-hdr-1/#dynamic-range-limit
#[value(" standard | no-limit | constrained | <dynamic-range-limit-mix()> ")]
#[initial("no-limit")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by dynamic-range-limit-mix()")]
pub enum DynamicRangeLimitStyleValue<'a> {}
