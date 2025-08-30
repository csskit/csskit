#![allow(warnings)]
//! CSS Regions Module Level 1
//! https://drafts.csswg.org/css-regions-1/

mod impls;
use impls::*;

// /// Represents the style value for `flow-into` as defined in [css-regions-1](https://drafts.csswg.org/css-regions-1/#flow-into).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | <custom-ident> [element | content]?
// /// ```
// ///
// // https://drafts.csswg.org/css-regions-1/#flow-into
// #[value(" none | <custom-ident> [element | content]? ")]
// #[initial("none")]
// #[applies_to("All elements, but not pseudo-elements such as ::first-line, ::first-letter, ::before or ::after.")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum FlowIntoStyleValue {}

/// Represents the style value for `flow-from` as defined in [css-regions-1](https://drafts.csswg.org/css-regions-1/#flow-from).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <custom-ident> | none
/// ```
///
// https://drafts.csswg.org/css-regions-1/#flow-from
#[value(" <custom-ident> | none ")]
#[initial("none")]
#[applies_to(
	"Non-replaced block containers.  This might be expanded in future versions of the specification to allow other types of containers to receive flow content."
)]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct FlowFromStyleValue;

/// Represents the style value for `region-fragment` as defined in [css-regions-1](https://drafts.csswg.org/css-regions-1/#region-fragment).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | break
/// ```
///
// https://drafts.csswg.org/css-regions-1/#region-fragment
#[value(" auto | break ")]
#[initial("auto")]
#[applies_to("CSS Regions")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum RegionFragmentStyleValue {}
