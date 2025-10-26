#![allow(warnings)]
//! https://drafts.csswg.org/css-viewport-1/

mod impls;
use super::prelude::*;
use impls::*;
/// Represents the style value for `zoom` as defined in [css-viewport-1](https://drafts.csswg.org/css-viewport-1/#zoom).
///
/// The zoom CSS property scales the size of an element. Unlike the transform property, a zoomed element affects page layout.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <number [0,∞]> | <percentage [0,∞]>
/// ```
///
/// https://drafts.csswg.org/css-viewport-1/#zoom
#[syntax(" <number [0,∞]> | <percentage [0,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "1",
	applies_to = "all <length> property values of all elements",
	inherited = "no",
	percentages = "converted to <number>",
	canonical_order = "per grammar",
	animation_type = "not animatable"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.zoom"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct ZoomStyleValue;
