#![allow(warnings)]
//! CSS View Transitions Module Level 2
//! https://drafts.csswg.org/css-view-transitions-2/

mod impls;
use impls::*;

/// Represents the style value for `view-transition-name` as defined in [css-view-transitions-2](https://drafts.csswg.org/css-view-transitions-2/#view-transition-name).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <custom-ident>
/// ```
///
// https://drafts.csswg.org/css-view-transitions-2/#view-transition-name
#[value(" none | <custom-ident> ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum ViewTransitionNameStyleValue {}

/// Represents the style value for `view-transition-class` as defined in [css-view-transitions-2](https://drafts.csswg.org/css-view-transitions-2/#view-transition-class).
///
/// The view-transition-class CSS property sets a name that can be used to apply styles to multiple named view transition pseudo-elements.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <custom-ident>+
/// ```
///
// https://drafts.csswg.org/css-view-transitions-2/#view-transition-class
#[value(" none | <custom-ident>+ ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:125,chrome_android:125,edge:125,safari:18.2,safari_ios:18.2)]
pub enum ViewTransitionClassStyleValue<'a> {}

/// Represents the style value for `view-transition-group` as defined in [css-view-transitions-2](https://drafts.csswg.org/css-view-transitions-2/#view-transition-group).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | contain | nearest | <custom-ident>
/// ```
///
// https://drafts.csswg.org/css-view-transitions-2/#view-transition-group
#[value(" normal | contain | nearest | <custom-ident> ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum ViewTransitionGroupStyleValue {}
