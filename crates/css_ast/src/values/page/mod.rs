#![allow(warnings)]
//! https://drafts.csswg.org/css-page-4/

mod impls;
use super::prelude::*;
use impls::*;
/// Represents the style value for `page` as defined in [css-page-4](https://drafts.csswg.org/css-page-4/#page).
///
/// The :first, :left, and :right pseudo-classes select pages based on their position in sequence after pagination. They're often used with the page CSS property, to choose a print layout defined by the @page rule.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <custom-ident>
/// ```
///
/// https://drafts.csswg.org/css-page-4/#page
#[syntax(" auto | <custom-ident> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "boxes that create class A break points",
	inherited = "no (but see prose)",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.page"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct PageStyleValue;
