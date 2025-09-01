#![allow(warnings)]
//! CSS Color Module Level 4
//! https://drafts.csswg.org/css-color-6/

mod impls;
use impls::*;

/// Represents the style value for `color` as defined in [css-color-6](https://drafts.csswg.org/css-color-6/#color).
///
/// The color CSS property sets the primary foreground color of an element, which is used for text, the default border color, and text decorations.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color>
/// ```
///
// https://drafts.csswg.org/css-color-6/#color
#[syntax(" <color> ")]
#[initial("CanvasText")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(89.836)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct ColorStyleValue;

/// Represents the style value for `opacity` as defined in [css-color-6](https://drafts.csswg.org/css-color-6/#opacity).
///
/// The opacity CSS property sets the transparency of an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <opacity-value>
/// ```
///
// https://drafts.csswg.org/css-color-6/#opacity
#[syntax(" <opacity-value> ")]
#[initial("1")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("map to the range [0,1]")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(89.836)]
#[caniuse("https://caniuse.com/css-opacity")]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:2,safari_ios:1)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct OpacityStyleValue;
