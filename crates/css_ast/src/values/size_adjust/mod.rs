#![allow(warnings)]
//! CSS Mobile Text Size Adjustment Module Level 1
//! https://drafts.csswg.org/css-size-adjust-1/

mod impls;
use impls::*;

/// Represents the style value for `text-size-adjust` as defined in [css-size-adjust-1](https://drafts.csswg.org/css-size-adjust-1/#text-size-adjust).
///
/// The text-size-adjust CSS property disables or modifies the browser's default text size adjustment for small screen sizes.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none | <percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-size-adjust-1/#text-size-adjust
#[syntax(" auto | none | <percentage [0,∞]> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("see below")]
#[canonical_order("n/a")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/text-size-adjust")]
#[baseline(limited)]
#[versions(chrome:54,chrome_android:54,edge:79)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct TextSizeAdjustStyleValue;
