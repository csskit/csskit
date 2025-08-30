#![allow(warnings)]
//! CSS Box Sizing Module Level 4
//! https://drafts.csswg.org/css-sizing-4/

mod impls;
use impls::*;

/// Represents the style value for `width` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain
/// ```
///
// https://drafts.csswg.org/css-sizing-4/#width
#[syntax(
	" auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain "
)]
#[initial("auto")]
#[applies_to("all elements except non-replaced inlines")]
#[inherited("no")]
#[percentages("relative to width/height of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type, recursing into fit-content()")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum WidthStyleValue {}

/// Represents the style value for `height` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#height).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain
/// ```
///
// https://drafts.csswg.org/css-sizing-4/#height
#[syntax(
	" auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain "
)]
#[initial("auto")]
#[applies_to("all elements except non-replaced inlines")]
#[inherited("no")]
#[percentages("relative to width/height of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type, recursing into fit-content()")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum HeightStyleValue {}

/// Represents the style value for `min-width` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#min-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain
/// ```
///
// https://drafts.csswg.org/css-sizing-4/#min-width
#[syntax(
	" auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain "
)]
#[initial("auto")]
#[applies_to("all elements that accept width or height")]
#[inherited("no")]
#[percentages("relative to width/height of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value, recursing into fit-content()")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum MinWidthStyleValue {}

/// Represents the style value for `min-height` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#min-height).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain
/// ```
///
// https://drafts.csswg.org/css-sizing-4/#min-height
#[syntax(
	" auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain "
)]
#[initial("auto")]
#[applies_to("all elements that accept width or height")]
#[inherited("no")]
#[percentages("relative to width/height of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value, recursing into fit-content()")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum MinHeightStyleValue {}

/// Represents the style value for `max-width` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#max-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain
/// ```
///
// https://drafts.csswg.org/css-sizing-4/#max-width
#[syntax(
	" none | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain "
)]
#[initial("none")]
#[applies_to("all elements that accept width or height")]
#[inherited("no")]
#[percentages("relative to width/height of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value, recursing into fit-content()")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum MaxWidthStyleValue {}

/// Represents the style value for `max-height` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#max-height).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain
/// ```
///
// https://drafts.csswg.org/css-sizing-4/#max-height
#[syntax(
	" none | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain "
)]
#[initial("none")]
#[applies_to("all elements that accept width or height")]
#[inherited("no")]
#[percentages("relative to width/height of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value, recursing into fit-content()")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum MaxHeightStyleValue {}

/// Represents the style value for `box-sizing` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#box-sizing).
///
/// The box-sizing CSS property sets whether an element's width and height are calculated based on the content-box, which does not count the size of borders or padding, or border-box, which does count them.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// content-box | border-box
/// ```
///
// https://drafts.csswg.org/css-sizing-4/#box-sizing
#[syntax(" content-box | border-box ")]
#[initial("content-box")]
#[applies_to("all elements that accept width or height")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css3-boxsizing")]
#[baseline(widely)]
#[versions(chrome:10,chrome_android:18,edge:12,firefox:29,firefox_android:29,safari:5.1,safari_ios:6)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BoxSizingStyleValue {}

/// Represents the style value for `aspect-ratio` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#aspect-ratio).
///
/// The aspect-ratio CSS property controls the width-to-height ratio of elements. For <img> and <video> elements, the width and height attributes used together with height: auto control the aspect ratio while the image/video is loading.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto || <ratio>
/// ```
///
// https://drafts.csswg.org/css-sizing-4/#aspect-ratio
#[syntax(" auto || <ratio> ")]
#[initial("auto")]
#[applies_to("all elements except inline boxes and internal ruby or table boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:88,chrome_android:88,edge:88,firefox:89,firefox_android:89,safari:15,safari_ios:15)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct AspectRatioStyleValue;

// /// Represents the style value for `contain-intrinsic-width` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-width).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto? [ none | <length [0,∞]> ]
// /// ```
// ///
// // https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-width
// #[syntax(" auto? [ none | <length [0,∞]> ] ")]
// #[initial("none")]
// #[applies_to("elements with size containment")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct ContainIntrinsicWidthStyleValue;

// /// Represents the style value for `contain-intrinsic-height` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-height).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto? [ none | <length [0,∞]> ]
// /// ```
// ///
// // https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-height
// #[syntax(" auto? [ none | <length [0,∞]> ] ")]
// #[initial("none")]
// #[applies_to("elements with size containment")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct ContainIntrinsicHeightStyleValue;

// /// Represents the style value for `contain-intrinsic-block-size` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-block-size).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto? [ none | <length [0,∞]> ]
// /// ```
// ///
// // https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-block-size
// #[syntax(" auto? [ none | <length [0,∞]> ] ")]
// #[initial("none")]
// #[applies_to("elements with size containment")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct ContainIntrinsicBlockSizeStyleValue;

// /// Represents the style value for `contain-intrinsic-inline-size` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-inline-size).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto? [ none | <length [0,∞]> ]
// /// ```
// ///
// // https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-inline-size
// #[syntax(" auto? [ none | <length [0,∞]> ] ")]
// #[initial("none")]
// #[applies_to("elements with size containment")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct ContainIntrinsicInlineSizeStyleValue;

// /// Represents the style value for `contain-intrinsic-size` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-size).
// ///
// /// The contain-intrinsic-size CSS property sets the intrinsic size of an element. When using size containment, the browser will lay out the element as if it had a single child of this size.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ auto? [ none | <length> ] ]{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-size
// #[syntax(" [ auto? [ none | <length> ] ]{1,2} ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(newly)]
// #[versions(chrome:83,chrome_android:83,edge:83,firefox:107,firefox_android:107,safari:17,safari_ios:17)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct ContainIntrinsicSizeStyleValue;

// /// Represents the style value for `min-intrinsic-sizing` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#min-intrinsic-sizing).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// legacy | zero-if-scroll || zero-if-extrinsic
// /// ```
// ///
// // https://drafts.csswg.org/css-sizing-4/#min-intrinsic-sizing
// #[syntax(" legacy | zero-if-scroll || zero-if-extrinsic ")]
// #[initial("legacy")]
// #[applies_to("all elements except inline boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum MinIntrinsicSizingStyleValue {}
