#![allow(warnings)]
//! CSS Conditional Rules Module Level 5
//! https://drafts.csswg.org/css-conditional-5/

mod impls;
use impls::*;

// /// Represents the style value for `container-type` as defined in [css-conditional-5](https://drafts.csswg.org/css-conditional-5/#container-type).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | [ [ size | inline-size ] || scroll-state ]
// /// ```
// ///
// // https://drafts.csswg.org/css-conditional-5/#container-type
// #[value(" normal | [ [ size | inline-size ] || scroll-state ] ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum ContainerTypeStyleValue {}

/// Represents the style value for `container-name` as defined in [css-conditional-5](https://drafts.csswg.org/css-conditional-5/#container-name).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <custom-ident>+
/// ```
///
// https://drafts.csswg.org/css-conditional-5/#container-name
#[value(" none | <custom-ident>+ ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ContainerNameStyleValue<'a>;

// /// Represents the style value for `container` as defined in [css-conditional-5](https://drafts.csswg.org/css-conditional-5/#container).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'container-name'> [ / <'container-type'> ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-conditional-5/#container
// #[value(" <'container-name'> [ / <'container-type'> ]? ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct ContainerStyleValue;
