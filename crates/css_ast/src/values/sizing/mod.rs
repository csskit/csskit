#![allow(warnings)]
//! https://drafts.csswg.org/css-sizing-4/

mod impls;
use super::prelude::*;
use impls::*;
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
/// https://drafts.csswg.org/css-sizing-4/#aspect-ratio
#[syntax(" auto || <ratio> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements except inline boxes and internal ruby or table boxes",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.aspect-ratio"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AspectRatioStyleValue;

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
/// https://drafts.csswg.org/css-sizing-4/#box-sizing
#[syntax(" content-box | border-box ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "content-box",
	applies_to = "all elements that accept width or height",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-sizing"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BoxSizingStyleValue {}

// /// Represents the style value for `contain-intrinsic-block-size` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-block-size).
// ///
// /// The contain-intrinsic-size CSS property sets the intrinsic size of an element. When using size containment, the browser will lay out the element as if it had a single child of this size.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto? [ none | <length [0,∞]> ]
// /// ```
// ///
// /// https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-block-size
// #[syntax(" auto? [ none | <length [0,∞]> ] ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     StyleValue,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash
// )]
// #[style_value(
//     initial = "none",
//     applies_to = "elements with size containment",
//     inherited = "no",
//     percentages = "n/a",
//     canonical_order = "per grammar",
//     animation_type = "by computed value type",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.contain-intrinsic-block-size")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct ContainIntrinsicBlockSizeStyleValue;

// /// Represents the style value for `contain-intrinsic-height` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-height).
// ///
// /// The contain-intrinsic-size CSS property sets the intrinsic size of an element. When using size containment, the browser will lay out the element as if it had a single child of this size.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto? [ none | <length [0,∞]> ]
// /// ```
// ///
// /// https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-height
// #[syntax(" auto? [ none | <length [0,∞]> ] ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     StyleValue,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash
// )]
// #[style_value(
//     initial = "none",
//     applies_to = "elements with size containment",
//     inherited = "no",
//     percentages = "n/a",
//     canonical_order = "per grammar",
//     animation_type = "by computed value type",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.contain-intrinsic-height")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct ContainIntrinsicHeightStyleValue;

// /// Represents the style value for `contain-intrinsic-inline-size` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-inline-size).
// ///
// /// The contain-intrinsic-size CSS property sets the intrinsic size of an element. When using size containment, the browser will lay out the element as if it had a single child of this size.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto? [ none | <length [0,∞]> ]
// /// ```
// ///
// /// https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-inline-size
// #[syntax(" auto? [ none | <length [0,∞]> ] ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     StyleValue,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash
// )]
// #[style_value(
//     initial = "none",
//     applies_to = "elements with size containment",
//     inherited = "no",
//     percentages = "n/a",
//     canonical_order = "per grammar",
//     animation_type = "by computed value type",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.contain-intrinsic-inline-size")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
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
// /// https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-size
// #[syntax(" [ auto? [ none | <length> ] ]{1,2} ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     StyleValue,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash
// )]
// #[style_value(
//     initial = "see individual properties",
//     applies_to = "see individual properties",
//     inherited = "see individual properties",
//     percentages = "see individual properties",
//     canonical_order = "per grammar",
//     animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.contain-intrinsic-size")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct ContainIntrinsicSizeStyleValue;

// /// Represents the style value for `contain-intrinsic-width` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-width).
// ///
// /// The contain-intrinsic-size CSS property sets the intrinsic size of an element. When using size containment, the browser will lay out the element as if it had a single child of this size.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto? [ none | <length [0,∞]> ]
// /// ```
// ///
// /// https://drafts.csswg.org/css-sizing-4/#contain-intrinsic-width
// #[syntax(" auto? [ none | <length [0,∞]> ] ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     StyleValue,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash
// )]
// #[style_value(
//     initial = "none",
//     applies_to = "elements with size containment",
//     inherited = "no",
//     percentages = "n/a",
//     canonical_order = "per grammar",
//     animation_type = "by computed value type",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.contain-intrinsic-width")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct ContainIntrinsicWidthStyleValue;

/// Represents the style value for `height` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#height).
///
/// The width and height CSS properties set the preferred physical size of an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain
/// ```
///
/// https://drafts.csswg.org/css-sizing-4/#height
#[syntax(
	" auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain "
)]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements except non-replaced inlines",
	inherited = "no",
	percentages = "relative to width/height of containing block",
	canonical_order = "per grammar",
	animation_type = "by computed value type, recursing into fit-content()"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.height"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum HeightStyleValue {}

/// Represents the style value for `max-height` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#max-height).
///
/// The min-width, min-height, max-width, and max-height CSS properties set the minimum and maximum size of an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain
/// ```
///
/// https://drafts.csswg.org/css-sizing-4/#max-height
#[syntax(
	" none | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain "
)]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements that accept width or height",
	inherited = "no",
	percentages = "relative to width/height of containing block",
	canonical_order = "per grammar",
	animation_type = "by computed value, recursing into fit-content()"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.max-height"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum MaxHeightStyleValue {}

/// Represents the style value for `max-width` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#max-width).
///
/// The min-width, min-height, max-width, and max-height CSS properties set the minimum and maximum size of an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain
/// ```
///
/// https://drafts.csswg.org/css-sizing-4/#max-width
#[syntax(
	" none | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain "
)]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements that accept width or height",
	inherited = "no",
	percentages = "relative to width/height of containing block",
	canonical_order = "per grammar",
	animation_type = "by computed value, recursing into fit-content()"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.max-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum MaxWidthStyleValue {}

/// Represents the style value for `min-height` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#min-height).
///
/// The min-width, min-height, max-width, and max-height CSS properties set the minimum and maximum size of an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain
/// ```
///
/// https://drafts.csswg.org/css-sizing-4/#min-height
#[syntax(
	" auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain "
)]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements that accept width or height",
	inherited = "no",
	percentages = "relative to width/height of containing block",
	canonical_order = "per grammar",
	animation_type = "by computed value, recursing into fit-content()"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.min-height"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum MinHeightStyleValue {}

// /// Represents the style value for `min-intrinsic-sizing` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#min-intrinsic-sizing).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// legacy | zero-if-scroll || zero-if-extrinsic
// /// ```
// ///
// /// https://drafts.csswg.org/css-sizing-4/#min-intrinsic-sizing
// #[syntax(" legacy | zero-if-scroll || zero-if-extrinsic ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     StyleValue,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash
// )]
// #[style_value(
//     initial = "legacy",
//     applies_to = "all elements except inline boxes",
//     inherited = "no",
//     percentages = "n/a",
//     canonical_order = "per grammar",
//     animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.min-intrinsic-sizing")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum MinIntrinsicSizingStyleValue {}

/// Represents the style value for `min-width` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#min-width).
///
/// The min-width, min-height, max-width, and max-height CSS properties set the minimum and maximum size of an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain
/// ```
///
/// https://drafts.csswg.org/css-sizing-4/#min-width
#[syntax(
	" auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain "
)]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements that accept width or height",
	inherited = "no",
	percentages = "relative to width/height of containing block",
	canonical_order = "per grammar",
	animation_type = "by computed value, recursing into fit-content()"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.min-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum MinWidthStyleValue {}

/// Represents the style value for `width` as defined in [css-sizing-4](https://drafts.csswg.org/css-sizing-4/#width).
///
/// The width and height CSS properties set the preferred physical size of an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain
/// ```
///
/// https://drafts.csswg.org/css-sizing-4/#width
#[syntax(
	" auto | <length-percentage [0,∞]> | min-content | max-content | fit-content(<length-percentage [0,∞]>) | <calc-size()> | stretch | fit-content | contain "
)]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements except non-replaced inlines",
	inherited = "no",
	percentages = "relative to width/height of containing block",
	canonical_order = "per grammar",
	animation_type = "by computed value type, recursing into fit-content()"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum WidthStyleValue {}
