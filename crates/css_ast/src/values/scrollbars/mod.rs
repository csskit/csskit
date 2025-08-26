#![allow(warnings)]
//! CSS Scrollbars Styling Module Level 1
//! https://drafts.csswg.org/css-scrollbars-1/

mod impls;
use impls::*;

/// Represents the style value for `scrollbar-color` as defined in [css-scrollbars-1](https://drafts.csswg.org/css-scrollbars-1/#scrollbar-color).
///
/// The scrollbar-color CSS property sets the color of the scrollbar track and thumb.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <color>{2}
/// ```
///
// https://drafts.csswg.org/css-scrollbars-1/#scrollbar-color
#[value(" auto | <color>{2} ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:121,chrome_android:121,edge:121,firefox:64,firefox_android:64)]
pub struct ScrollbarColorStyleValue;

/// Represents the style value for `scrollbar-width` as defined in [css-scrollbars-1](https://drafts.csswg.org/css-scrollbars-1/#scrollbar-width).
///
/// The scrollbar-width CSS property sets the width of the scrollbar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | thin | none
/// ```
///
// https://drafts.csswg.org/css-scrollbars-1/#scrollbar-width
#[value(" auto | thin | none ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:121,chrome_android:121,edge:121,firefox:64,firefox_android:64,safari:18.2,safari_ios:18.2)]
pub enum ScrollbarWidthStyleValue {}
