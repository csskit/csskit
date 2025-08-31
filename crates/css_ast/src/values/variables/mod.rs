#![allow(warnings)]
//! CSS Custom Properties for Cascading Variables Module Level 2
//! https://drafts.csswg.org/css-variables-2/

mod impls;
use impls::*;

// /// Represents the style value for `--*` as defined in [css-variables-2](https://drafts.csswg.org/css-variables-2/#defining-variables).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <declaration-value>?
// /// ```
// ///
// // https://drafts.csswg.org/css-variables-2/#defining-variables
// #[syntax(" <declaration-value>? ")]
// #[initial("the guaranteed-invalid value")]
// #[applies_to("all elements and all pseudo-elements (including those with restricted property lists)")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CustomStyleValue;
