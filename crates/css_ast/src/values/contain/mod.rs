#![allow(warnings)]
//! CSS Containment Module Level 2
//! https://drafts.csswg.org/css-contain-4/

mod impls;
use impls::*;

// /// Represents the style value for `contain` as defined in [css-contain-4](https://drafts.csswg.org/css-contain-4/#contain).
// ///
// /// The contain CSS property sets limits to the scope of styles, layout, and paint rendering for speed and efficiency. The none keyword value disables containment, strict is equivalent to contain: size layout style paint, and content is equivalent to contain: layout style paint.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | strict | content | [ [size | inline-size] || layout || style || paint ]
// /// ```
// ///
// // https://drafts.csswg.org/css-contain-4/#contain
// #[value(" none | strict | content | [ [size | inline-size] || layout || style || paint ] ")]
// #[initial("none")]
// #[applies_to("See below")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// #[popularity(18.392)]
// #[caniuse("https://caniuse.com/css-containment")]
// #[baseline(widely)]
// #[versions(chrome:52,chrome_android:52,edge:79,firefox:69,firefox_android:79,safari:15.4,safari_ios:15.4)]
// pub enum ContainStyleValue {}

/// Represents the style value for `content-visibility` as defined in [css-contain-4](https://drafts.csswg.org/css-contain-4/#content-visibility).
///
/// The content-visibility CSS property delays rendering an element, including layout and painting, until it is needed.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// visible | auto | hidden
/// ```
///
// https://drafts.csswg.org/css-contain-4/#content-visibility
#[value(" visible | auto | hidden ")]
#[initial("visible")]
#[applies_to("elements for which size containment can apply")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see § 4.1 animating and interpolating content-visibility")]
#[popularity(18.392)]
#[caniuse("https://caniuse.com/css-content-visibility")]
#[baseline(limited)]
#[versions(chrome:108,chrome_android:108,edge:108,firefox:130,firefox_android:130)]
pub enum ContentVisibilityStyleValue {}
