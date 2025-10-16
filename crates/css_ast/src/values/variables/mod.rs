#![allow(warnings)]
//! CSS Custom Properties for Cascading Variables Module Level 2
//! https://drafts.csswg.org/css-variables-2/

mod impls;

use super::prelude::*;
use impls::*;

// /// Represents the style value for `--*` as defined in [css-variables-2](https://drafts.csswg.org/css-variables-2/#defining-variables).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <declaration-value>?
// /// ```
// ///
// // https://drafts.csswg.org/css-variables-2/#defining-variables
// #[syntax(" <declaration-value>? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "the guaranteed-invalid value",
//   applies_to = "all elements and all pseudo-elements (including those with restricted property lists)",
// 	inherited = "yes",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.--*"))]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CustomStyleValue;
