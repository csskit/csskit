#![allow(warnings)]
//! CSS Spatial Navigation Level 1
//! https://drafts.csswg.org/css-nav-1/

mod impls;
use impls::*;

/// Represents the style value for `spatial-navigation-contain` as defined in [css-nav-1](https://drafts.csswg.org/css-nav-1/#spatial-navigation-contain).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | contain
/// ```
///
// https://drafts.csswg.org/css-nav-1/#spatial-navigation-contain
#[value(" auto | contain ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum SpatialNavigationContainStyleValue {}

/// Represents the style value for `spatial-navigation-action` as defined in [css-nav-1](https://drafts.csswg.org/css-nav-1/#spatial-navigation-action).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | focus | scroll
/// ```
///
// https://drafts.csswg.org/css-nav-1/#spatial-navigation-action
#[value(" auto | focus | scroll ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum SpatialNavigationActionStyleValue {}

/// Represents the style value for `spatial-navigation-function` as defined in [css-nav-1](https://drafts.csswg.org/css-nav-1/#spatial-navigation-function).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | grid
/// ```
///
// https://drafts.csswg.org/css-nav-1/#spatial-navigation-function
#[value(" normal | grid ")]
#[initial("normal")]
#[applies_to("spatial navigation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum SpatialNavigationFunctionStyleValue {}
