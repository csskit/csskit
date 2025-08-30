#![allow(warnings)]
//! CSS Scroll Anchoring Module Level 1
//! https://drafts.csswg.org/css-scroll-anchoring-1/

mod impls;
use impls::*;

/// Represents the style value for `overflow-anchor` as defined in [css-scroll-anchoring-1](https://drafts.csswg.org/css-scroll-anchoring-1/#overflow-anchor).
///
/// The overflow-anchor CSS property sets an element as a possible scroll anchor, reducing unintended scrolling when document changes occur above the current scrollport. This is enabled by default where supported.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none
/// ```
///
// https://drafts.csswg.org/css-scroll-anchoring-1/#overflow-anchor
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-overflow-anchor")]
#[baseline(limited)]
#[versions(chrome:56,chrome_android:56,edge:79,firefox:66,firefox_android:66)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum OverflowAnchorStyleValue {}
