#![allow(warnings)]
//! https://drafts.csswg.org/css-cascade-6/

mod impls;
use super::prelude::*;
use impls::*;
/// Represents the style value for `all` as defined in [css-cascade-6](https://drafts.csswg.org/css-cascade-6/#all).
///
/// The all CSS property is a shorthand for all CSS properties, except for direction and unicode-bidi. It accepts only the keywords for explicit defaulting (such as initial and inherit), since they are the only values supported on all CSS properties.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// initial | inherit | unset | revert | revert-layer
/// ```
///
/// https://drafts.csswg.org/css-cascade-6/#all
#[syntax(" initial | inherit | unset | revert | revert-layer ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.all"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum AllStyleValue {}
