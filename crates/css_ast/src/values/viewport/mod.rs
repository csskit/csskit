#![allow(warnings)]
//! CSS Viewport Module Level 1
//! https://drafts.csswg.org/css-viewport-1/

mod impls;
use impls::*;

/// Represents the style value for `zoom` as defined in [css-viewport-1](https://drafts.csswg.org/css-viewport-1/#zoom).
///
/// The zoom CSS property scales the size of an element. Unlike the transform property, a zoomed element affects page layout.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <number [0,∞]> | <percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-viewport-1/#zoom
#[value(" <number [0,∞]> | <percentage [0,∞]> ")]
#[initial("1")]
#[applies_to("all <length> property values of all elements")]
#[inherited("no")]
#[percentages("converted to <number>")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-zoom")]
#[baseline(newly)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:126,firefox_android:126,safari:3.1,safari_ios:3)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct ZoomStyleValue;
