#![allow(warnings)]
//! CSS Overscroll Behavior Module Level 1
//! https://drafts.csswg.org/css-overscroll-1/

mod impls;
use impls::*;

/// Represents the style value for `overscroll-behavior` as defined in [css-overscroll-1](https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior).
///
/// The overscroll-behavior CSS property disables default scrolling behaviors when the edges of a scrolling area are reached.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ contain | none | auto ]{1,2}
/// ```
///
// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior
#[value(" [ contain | none | auto ]{1,2} ")]
#[initial("auto auto")]
#[applies_to("scroll container elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-overscroll-behavior")]
#[baseline(widely)]
#[versions(chrome:63,chrome_android:63,edge:18,firefox:59,firefox_android:59,safari:16,safari_ios:16)]
pub struct OverscrollBehaviorStyleValue;

/// Represents the style value for `overscroll-behavior-x` as defined in [css-overscroll-1](https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-x).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// contain | none | auto
/// ```
///
// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-x
#[value(" contain | none | auto ")]
#[initial("auto")]
#[applies_to("scroll container elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum OverscrollBehaviorXStyleValue {}

/// Represents the style value for `overscroll-behavior-y` as defined in [css-overscroll-1](https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-y).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// contain | none | auto
/// ```
///
// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-y
#[value(" contain | none | auto ")]
#[initial("auto")]
#[applies_to("scroll container elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum OverscrollBehaviorYStyleValue {}

/// Represents the style value for `overscroll-behavior-inline` as defined in [css-overscroll-1](https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-inline).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// contain | none | auto
/// ```
///
// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-inline
#[value(" contain | none | auto ")]
#[initial("auto")]
#[applies_to("scroll container elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum OverscrollBehaviorInlineStyleValue {}

/// Represents the style value for `overscroll-behavior-block` as defined in [css-overscroll-1](https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-block).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// contain | none | auto
/// ```
///
// https://drafts.csswg.org/css-overscroll-1/#overscroll-behavior-block
#[value(" contain | none | auto ")]
#[initial("auto")]
#[applies_to("scroll container elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum OverscrollBehaviorBlockStyleValue {}
