#![allow(warnings)]
//! CSS Paged Media Module Level 3
//! https://drafts.csswg.org/css-page-4/

mod impls;
use impls::*;

/// Represents the style value for `page` as defined in [css-page-4](https://drafts.csswg.org/css-page-4/#page).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <custom-ident>
/// ```
///
// https://drafts.csswg.org/css-page-4/#page
#[value(" auto | <custom-ident> ")]
#[initial("auto")]
#[applies_to("boxes that create class A break points")]
#[inherited("no (but see prose)")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(0.328)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct PageStyleValue;
