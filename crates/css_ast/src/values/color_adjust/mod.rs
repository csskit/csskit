#![allow(warnings)]
//! CSS Color Adjustment Module Level 1
//! https://drafts.csswg.org/css-color-adjust-1/

mod impls;
use impls::*;

// /// Represents the style value for `color-scheme` as defined in [css-color-adjust-1](https://drafts.csswg.org/css-color-adjust-1/#color-scheme).
// ///
// /// The color-scheme CSS property sets which color schemes (light or dark) an element uses and may prevent automatic dark mode adjustments by the browser.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | [ light | dark | <custom-ident> ]+ && only?
// /// ```
// ///
// // https://drafts.csswg.org/css-color-adjust-1/#color-scheme
// #[syntax(" normal | [ light | dark | <custom-ident> ]+ && only? ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:98,chrome_android:98,edge:98,firefox:96,firefox_android:96,safari:13,safari_ios:13)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum ColorSchemeStyleValue<'a> {}

/// Represents the style value for `forced-color-adjust` as defined in [css-color-adjust-1](https://drafts.csswg.org/css-color-adjust-1/#forced-color-adjust).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none | preserve-parent-color
/// ```
///
// https://drafts.csswg.org/css-color-adjust-1/#forced-color-adjust
#[syntax(" auto | none | preserve-parent-color ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum ForcedColorAdjustStyleValue {}

/// Represents the style value for `print-color-adjust` as defined in [css-color-adjust-1](https://drafts.csswg.org/css-color-adjust-1/#print-color-adjust).
///
/// The print-color-adjust CSS property sets whether styles of printed pages should be adjusted to use less ink, in cases such as light text on a dark background.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// economy | exact
/// ```
///
// https://drafts.csswg.org/css-color-adjust-1/#print-color-adjust
#[syntax(" economy | exact ")]
#[initial("economy")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:136,chrome_android:136,edge:136,firefox:97,firefox_android:97,safari:15.4,safari_ios:15.4)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum PrintColorAdjustStyleValue {}

/// Represents the style value for `color-adjust` as defined in [css-color-adjust-1](https://drafts.csswg.org/css-color-adjust-1/#color-adjust).
///
/// The color-adjust shorthand CSS property allows multiple performance related color adjustments to be set at once. Setting the print-color-adjust CSS property directly is preferred, as it is the only such adjustment so far defined.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'print-color-adjust'>
/// ```
///
// https://drafts.csswg.org/css-color-adjust-1/#color-adjust
#[syntax(" <'print-color-adjust'> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(firefox:48,firefox_android:48,safari:15.4,safari_ios:15.4)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct ColorAdjustStyleValue;
