#![allow(warnings)]
//! https://drafts.csswg.org/css-will-change-1/

mod impls;
use super::prelude::*;
use impls::*;
/// Represents the style value for `will-change` as defined in [css-will-change-1](https://drafts.csswg.org/css-will-change-1/#will-change).
///
/// The will-change CSS property gives hints to the browser about expected changes to an element's scroll position, contents, or style. These hints allow browsers to optimize for upcoming style changes.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <animateable-feature>#
/// ```
///
/// https://drafts.csswg.org/css-will-change-1/#will-change
#[syntax(" auto | <animateable-feature># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.will-change"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct WillChangeStyleValue<'a>;
