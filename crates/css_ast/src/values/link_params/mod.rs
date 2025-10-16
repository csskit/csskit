#![allow(warnings)]
//! CSS Linked Parameters Module Level 1
//! https://drafts.csswg.org/css-link-params-1/

mod impls;

use super::prelude::*;
use impls::*;

/// Represents the style value for `link-parameters` as defined in [css-link-params-1](https://drafts.csswg.org/css-link-params-1/#link-parameters).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <param()>#
/// ```
///
// https://drafts.csswg.org/css-link-params-1/#link-parameters
#[syntax(" none | <param()># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements and pseudo-elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.link-parameters"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct LinkParametersStyleValue<'a>;
