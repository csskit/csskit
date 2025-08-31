#![allow(warnings)]
//! CSS Shapes Module Level 2
//! https://drafts.csswg.org/css-shapes-2/

mod impls;
use impls::*;

// /// Represents the style value for `shape-outside` as defined in [css-shapes-2](https://drafts.csswg.org/css-shapes-2/#shape-outside).
// ///
// /// The shape-outside CSS property, along with shape-margin and shape-image-threshold, sets the shape around which adjacent content will wrap.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ <basic-shape> || <shape-box> ] | <image>
// /// ```
// ///
// // https://drafts.csswg.org/css-shapes-2/#shape-outside
// #[syntax(" none | [ <basic-shape> || <shape-box> ] | <image> ")]
// #[initial("none")]
// #[applies_to("floats and initial letter boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("as defined for <basic-shape>, otherwise discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:37,chrome_android:37,edge:79,firefox:62,firefox_android:62,safari:10.1,safari_ios:10.3)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum ShapeOutsideStyleValue<'a> {}

/// Represents the style value for `shape-image-threshold` as defined in [css-shapes-2](https://drafts.csswg.org/css-shapes-2/#shape-image-threshold).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <opacity-value>
/// ```
///
// https://drafts.csswg.org/css-shapes-2/#shape-image-threshold
#[syntax(" <opacity-value> ")]
#[initial("0")]
#[applies_to("floats")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct ShapeImageThresholdStyleValue;

/// Represents the style value for `shape-margin` as defined in [css-shapes-2](https://drafts.csswg.org/css-shapes-2/#shape-margin).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-shapes-2/#shape-margin
#[syntax(" <length-percentage [0,∞]> ")]
#[initial("0")]
#[applies_to("floats and initial letter boxes")]
#[inherited("no")]
#[percentages("refer to the inline size of the containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct ShapeMarginStyleValue;

// /// Represents the style value for `shape-inside` as defined in [css-shapes-2](https://drafts.csswg.org/css-shapes-2/#shape-inside).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | outside-shape | [ <basic-shape> || shape-box ] | <image> | display
// /// ```
// ///
// // https://drafts.csswg.org/css-shapes-2/#shape-inside
// #[syntax(" auto | outside-shape | [ <basic-shape> || shape-box ] | <image> | display ")]
// #[initial("auto")]
// #[applies_to("block-level elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("as defined for <basic-shape>, otherwise discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum ShapeInsideStyleValue<'a> {}

/// Represents the style value for `shape-padding` as defined in [css-shapes-2](https://drafts.csswg.org/css-shapes-2/#shape-padding).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-shapes-2/#shape-padding
#[syntax(" <length-percentage [0,∞]> ")]
#[initial("0")]
#[applies_to("block-level elements")]
#[inherited("no")]
#[percentages("refer to the inline size of the containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct ShapePaddingStyleValue;
