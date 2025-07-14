#![allow(warnings)]
//! CSS Generated Content for Paged Media Module Level 4
//! https://drafts.csswg.org/css-gcpm-4/

mod impls;
use impls::*;

// /// Represents the style value for `string-set` as defined in [css-gcpm-4](https://drafts.csswg.org/css-gcpm-4/#string-set).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <custom-ident> <content-list> ]# | none
// /// ```
// ///
// // https://drafts.csswg.org/css-gcpm-4/#string-set
// #[value(" [ <custom-ident> <content-list> ]# | none ")]
// #[initial("none")]
// #[applies_to("all elements, but not pseudo-elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum StringSetStyleValue<'a> {}

/// Represents the style value for `running` as defined in [css-gcpm-4](https://drafts.csswg.org/css-gcpm-4/#running).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <custom-ident>
/// ```
///
// https://drafts.csswg.org/css-gcpm-4/#running
#[value(" <custom-ident> ")]
#[initial("none")]
#[applies_to("elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct RunningStyleValue;

/// Represents the style value for `footnote-display` as defined in [css-gcpm-4](https://drafts.csswg.org/css-gcpm-4/#footnote-display).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// block | inline | compact
/// ```
///
// https://drafts.csswg.org/css-gcpm-4/#footnote-display
#[value(" block | inline | compact ")]
#[initial("block")]
#[applies_to("elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum FootnoteDisplayStyleValue {}

/// Represents the style value for `footnote-policy` as defined in [css-gcpm-4](https://drafts.csswg.org/css-gcpm-4/#footnote-policy).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | line | block
/// ```
///
// https://drafts.csswg.org/css-gcpm-4/#footnote-policy
#[value(" auto | line | block ")]
#[initial("auto")]
#[applies_to("elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum FootnotePolicyStyleValue {}

// /// Represents the style value for `copy-into` as defined in [css-gcpm-4](https://drafts.csswg.org/css-gcpm-4/#copy-into).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none |  [ [ <custom-ident>  <content-level>] [,  <custom-ident>  <content-level>]*  ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-gcpm-4/#copy-into
// #[value(" none |  [ [ <custom-ident>  <content-level>] [,  <custom-ident>  <content-level>]*  ]? ")]
// #[initial("none")]
// #[applies_to("all elements and pseudo-elements, but not ::first-line or ::first-letter.")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum CopyIntoStyleValue {}
