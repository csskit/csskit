#![allow(warnings)]
//! CSS Linked Parameters
//! https://drafts.csswg.org/css-link-params-1/

mod impls;
use impls::*;

// /// Represents the style value for `link-parameters` as defined in [css-link-params-1](https://drafts.csswg.org/css-link-params-1/#link-parameters).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | <param()>#
// /// ```
// ///
// // https://drafts.csswg.org/css-link-params-1/#link-parameters
// #[value(" none | <param()># ")]
// #[initial("none")]
// #[applies_to("all elements and pseudo-elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum LinkParametersStyleValue<'a> {}
