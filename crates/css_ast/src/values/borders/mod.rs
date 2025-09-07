#![allow(warnings)]
//! CSS Borders and Box Decorations Module Level 4
//! https://drafts.csswg.org/css-borders-4/

mod impls;
use impls::*;

/// Represents the style value for `border-top-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-color).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-top-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "currentcolor",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see prose"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top-color"))]
#[visit]
pub enum BorderTopColorStyleValue<'a> {}

/// Represents the style value for `border-right-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-right-color).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-right-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "currentcolor",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see prose"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-right-color"))]
#[visit]
pub enum BorderRightColorStyleValue<'a> {}

/// Represents the style value for `border-bottom-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-color).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-bottom-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "currentcolor",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see prose"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-bottom-color"))]
#[visit]
pub enum BorderBottomColorStyleValue<'a> {}

/// Represents the style value for `border-left-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-left-color).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-left-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "currentcolor",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see prose"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-left-color"))]
#[visit]
pub enum BorderLeftColorStyleValue<'a> {}

/// Represents the style value for `border-block-start-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-start-color).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-start-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "currentcolor",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see prose"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-start-color"))]
#[visit]
pub enum BorderBlockStartColorStyleValue<'a> {}

/// Represents the style value for `border-block-end-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-end-color).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-end-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "currentcolor",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see prose"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-end-color"))]
#[visit]
pub enum BorderBlockEndColorStyleValue<'a> {}

/// Represents the style value for `border-inline-start-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-start-color).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-start-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "currentcolor",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see prose"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-start-color"))]
#[visit]
pub enum BorderInlineStartColorStyleValue<'a> {}

/// Represents the style value for `border-inline-end-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-end-color).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-end-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "currentcolor",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see prose"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-end-color"))]
#[visit]
pub enum BorderInlineEndColorStyleValue<'a> {}

// /// Represents the style value for `border-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-color).
// ///
// /// The border CSS property sets the color, style, and width of the line around an element.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <color> | <image-1D> ]{1,4}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-color
// #[syntax(" [ <color> | <image-1D> ]{1,4} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "see individual properties",
//   applies_to = "see individual properties",
// 	inherited = "see individual properties",
// 	percentages = "see individual properties",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-color"))]
// #[visit]
// pub struct BorderColorStyleValue<'a>;

/// Represents the style value for `border-block-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-color).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-color'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-color
#[syntax(" <'border-top-color'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-color"))]
#[visit]
pub struct BorderBlockColorStyleValue<'a>;

/// Represents the style value for `border-inline-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-color).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-color'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-color
#[syntax(" <'border-top-color'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-color"))]
#[visit]
pub struct BorderInlineColorStyleValue<'a>;

/// Represents the style value for `border-top-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-style).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-top-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top-style"))]
#[visit]
pub struct BorderTopStyleStyleValue;

/// Represents the style value for `border-right-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-right-style).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-right-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-right-style"))]
#[visit]
pub struct BorderRightStyleStyleValue;

/// Represents the style value for `border-bottom-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-style).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-bottom-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-bottom-style"))]
#[visit]
pub struct BorderBottomStyleStyleValue;

/// Represents the style value for `border-left-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-left-style).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-left-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-left-style"))]
#[visit]
pub struct BorderLeftStyleStyleValue;

/// Represents the style value for `border-block-start-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-start-style).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-start-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-start-style"))]
#[visit]
pub struct BorderBlockStartStyleStyleValue;

/// Represents the style value for `border-block-end-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-end-style).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-end-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-end-style"))]
#[visit]
pub struct BorderBlockEndStyleStyleValue;

/// Represents the style value for `border-inline-start-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-start-style).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-start-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-start-style"))]
#[visit]
pub struct BorderInlineStartStyleStyleValue;

/// Represents the style value for `border-inline-end-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-end-style).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-end-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-end-style"))]
#[visit]
pub struct BorderInlineEndStyleStyleValue;

/// Represents the style value for `border-block-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-style).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-style'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-style
#[syntax(" <'border-top-style'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-style"))]
#[visit]
pub struct BorderBlockStyleStyleValue;

/// Represents the style value for `border-inline-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-style).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-style'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-style
#[syntax(" <'border-top-style'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-style"))]
#[visit]
pub struct BorderInlineStyleStyleValue;

/// Represents the style value for `border-top-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-width).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-top-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "medium",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top-width"))]
#[visit]
pub struct BorderTopWidthStyleValue;

/// Represents the style value for `border-right-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-right-width).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-right-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "medium",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-right-width"))]
#[visit]
pub struct BorderRightWidthStyleValue;

/// Represents the style value for `border-bottom-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-width).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-bottom-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "medium",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-bottom-width"))]
#[visit]
pub struct BorderBottomWidthStyleValue;

/// Represents the style value for `border-left-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-left-width).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-left-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "medium",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-left-width"))]
#[visit]
pub struct BorderLeftWidthStyleValue;

/// Represents the style value for `border-block-start-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-start-width).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-start-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "medium",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-start-width"))]
#[visit]
pub struct BorderBlockStartWidthStyleValue;

/// Represents the style value for `border-block-end-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-end-width).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-end-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "medium",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-end-width"))]
#[visit]
pub struct BorderBlockEndWidthStyleValue;

/// Represents the style value for `border-inline-start-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-start-width).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-start-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "medium",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-start-width"))]
#[visit]
pub struct BorderInlineStartWidthStyleValue;

/// Represents the style value for `border-inline-end-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-end-width).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-end-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "medium",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-end-width"))]
#[visit]
pub struct BorderInlineEndWidthStyleValue;

/// Represents the style value for `border-block-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-width).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-width'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-width
#[syntax(" <'border-top-width'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-width"))]
#[visit]
pub struct BorderBlockWidthStyleValue;

/// Represents the style value for `border-inline-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-width).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-width'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-width
#[syntax(" <'border-top-width'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-width"))]
#[visit]
pub struct BorderInlineWidthStyleValue;

/// Represents the style value for `border-top` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-top
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "See individual properties",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top"))]
#[visit]
pub struct BorderTopStyleValue;

/// Represents the style value for `border-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-right).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-right
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "See individual properties",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-right"))]
#[visit]
pub struct BorderRightStyleValue;

/// Represents the style value for `border-bottom` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-bottom
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "See individual properties",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-bottom"))]
#[visit]
pub struct BorderBottomStyleValue;

/// Represents the style value for `border-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-left).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-left
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "See individual properties",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-left"))]
#[visit]
pub struct BorderLeftStyleValue;

/// Represents the style value for `border-block-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-start).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-start
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "See individual properties",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-start"))]
#[visit]
pub struct BorderBlockStartStyleValue;

/// Represents the style value for `border-block-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-end).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-end
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "See individual properties",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-end"))]
#[visit]
pub struct BorderBlockEndStyleValue;

/// Represents the style value for `border-inline-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-start).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-start
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "See individual properties",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-start"))]
#[visit]
pub struct BorderInlineStartStyleValue;

/// Represents the style value for `border-inline-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-end).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-end
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "See individual properties",
	applies_to = "all elements except ruby base containers and ruby annotation containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-end"))]
#[visit]
pub struct BorderInlineEndStyleValue;

/// Represents the style value for `border-block` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-block-start'>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block
#[syntax(" <'border-block-start'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block"))]
#[visit]
pub struct BorderBlockStyleValue;

/// Represents the style value for `border-inline` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-block-start'>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline
#[syntax(" <'border-block-start'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline"))]
#[visit]
pub struct BorderInlineStyleValue;

/// Represents the style value for `border-top-left-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-left-radius).
///
/// The border-radius CSS property rounds the corners of the border drawn around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-top-left-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements (but see prose)",
	inherited = "no",
	percentages = "refer to corresponding dimension of the border box.",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top-left-radius"))]
#[visit]
pub struct BorderTopLeftRadiusStyleValue;

/// Represents the style value for `border-top-right-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-right-radius).
///
/// The border-radius CSS property rounds the corners of the border drawn around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-top-right-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements (but see prose)",
	inherited = "no",
	percentages = "refer to corresponding dimension of the border box.",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top-right-radius"))]
#[visit]
pub struct BorderTopRightRadiusStyleValue;

/// Represents the style value for `border-bottom-right-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-right-radius).
///
/// The border-radius CSS property rounds the corners of the border drawn around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-bottom-right-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements (but see prose)",
	inherited = "no",
	percentages = "refer to corresponding dimension of the border box.",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(
	feature = "css_feature_data",
	derive(ToCSSFeature),
	css_feature("css.properties.border-bottom-right-radius")
)]
#[visit]
pub struct BorderBottomRightRadiusStyleValue;

/// Represents the style value for `border-bottom-left-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-left-radius).
///
/// The border-radius CSS property rounds the corners of the border drawn around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-bottom-left-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements (but see prose)",
	inherited = "no",
	percentages = "refer to corresponding dimension of the border box.",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-bottom-left-radius"))]
#[visit]
pub struct BorderBottomLeftRadiusStyleValue;

/// Represents the style value for `border-start-start-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-start-start-radius).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-start-start-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements (but see prose)",
	inherited = "no",
	percentages = "refer to corresponding dimension of the border box.",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-start-start-radius"))]
#[visit]
pub struct BorderStartStartRadiusStyleValue;

/// Represents the style value for `border-start-end-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-start-end-radius).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-start-end-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements (but see prose)",
	inherited = "no",
	percentages = "refer to corresponding dimension of the border box.",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-start-end-radius"))]
#[visit]
pub struct BorderStartEndRadiusStyleValue;

/// Represents the style value for `border-end-start-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-end-start-radius).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-end-start-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements (but see prose)",
	inherited = "no",
	percentages = "refer to corresponding dimension of the border box.",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-end-start-radius"))]
#[visit]
pub struct BorderEndStartRadiusStyleValue;

/// Represents the style value for `border-end-end-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-end-end-radius).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-end-end-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements (but see prose)",
	inherited = "no",
	percentages = "refer to corresponding dimension of the border box.",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-end-end-radius"))]
#[visit]
pub struct BorderEndEndRadiusStyleValue;

// /// Represents the style value for `border-top-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-top-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top-radius"))]
// #[visit]
// pub struct BorderTopRadiusStyleValue;

// /// Represents the style value for `border-right-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-right-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-right-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-right-radius"))]
// #[visit]
// pub struct BorderRightRadiusStyleValue;

// /// Represents the style value for `border-bottom-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-bottom-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-bottom-radius"))]
// #[visit]
// pub struct BorderBottomRadiusStyleValue;

// /// Represents the style value for `border-left-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-left-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-left-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-left-radius"))]
// #[visit]
// pub struct BorderLeftRadiusStyleValue;

// /// Represents the style value for `border-block-start-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-start-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-block-start-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-start-radius"))]
// #[visit]
// pub struct BorderBlockStartRadiusStyleValue;

// /// Represents the style value for `border-block-end-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-end-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-block-end-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-end-radius"))]
// #[visit]
// pub struct BorderBlockEndRadiusStyleValue;

// /// Represents the style value for `border-inline-start-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-start-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-inline-start-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-start-radius"))]
// #[visit]
// pub struct BorderInlineStartRadiusStyleValue;

// /// Represents the style value for `border-inline-end-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-end-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-inline-end-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-end-radius"))]
// #[visit]
// pub struct BorderInlineEndRadiusStyleValue;

// /// Represents the style value for `border-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-radius).
// ///
// /// The border-radius CSS property rounds the corners of the border drawn around an element.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-radius
// #[syntax(" <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]? ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements, except table element when border-collapse is collapse",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-radius"))]
// #[visit]
// pub struct BorderRadiusStyleValue;

/// Represents the style value for `corner-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,4}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-shape
#[syntax(" <corner-shape-value>{1,4} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "round",
	applies_to = "all elements where border-radius can apply",
	inherited = "no",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-shape"))]
#[visit]
pub struct CornerShapeStyleValue;

/// Represents the style value for `corner-top-left-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-left-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-top-left-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "round",
	applies_to = "all elements where border-radius can apply",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see superellipse interpolation"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-top-left-shape"))]
#[visit]
pub struct CornerTopLeftShapeStyleValue;

/// Represents the style value for `corner-top-right-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-right-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-top-right-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "round",
	applies_to = "all elements where border-radius can apply",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see superellipse interpolation"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-top-right-shape"))]
#[visit]
pub struct CornerTopRightShapeStyleValue;

/// Represents the style value for `corner-bottom-right-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-right-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-bottom-right-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "round",
	applies_to = "all elements where border-radius can apply",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see superellipse interpolation"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-bottom-right-shape"))]
#[visit]
pub struct CornerBottomRightShapeStyleValue;

/// Represents the style value for `corner-bottom-left-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-left-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-bottom-left-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "round",
	applies_to = "all elements where border-radius can apply",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see superellipse interpolation"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-bottom-left-shape"))]
#[visit]
pub struct CornerBottomLeftShapeStyleValue;

/// Represents the style value for `corner-start-start-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-start-start-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-start-start-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "round",
	applies_to = "all elements where border-radius can apply",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see superellipse interpolation"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-start-start-shape"))]
#[visit]
pub struct CornerStartStartShapeStyleValue;

/// Represents the style value for `corner-start-end-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-start-end-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-start-end-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "round",
	applies_to = "all elements where border-radius can apply",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see superellipse interpolation"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-start-end-shape"))]
#[visit]
pub struct CornerStartEndShapeStyleValue;

/// Represents the style value for `corner-end-start-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-end-start-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-end-start-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "round",
	applies_to = "all elements where border-radius can apply",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see superellipse interpolation"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-end-start-shape"))]
#[visit]
pub struct CornerEndStartShapeStyleValue;

/// Represents the style value for `corner-end-end-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-end-end-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-end-end-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "round",
	applies_to = "all elements where border-radius can apply",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see superellipse interpolation"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-end-end-shape"))]
#[visit]
pub struct CornerEndEndShapeStyleValue;

/// Represents the style value for `corner-top-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-top-shape
#[syntax(" <corner-shape-value>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-top-shape"))]
#[visit]
pub struct CornerTopShapeStyleValue;

/// Represents the style value for `corner-right-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-right-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-right-shape
#[syntax(" <corner-shape-value>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-right-shape"))]
#[visit]
pub struct CornerRightShapeStyleValue;

/// Represents the style value for `corner-bottom-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-bottom-shape
#[syntax(" <corner-shape-value>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-bottom-shape"))]
#[visit]
pub struct CornerBottomShapeStyleValue;

/// Represents the style value for `corner-left-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-left-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-left-shape
#[syntax(" <corner-shape-value>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-left-shape"))]
#[visit]
pub struct CornerLeftShapeStyleValue;

/// Represents the style value for `corner-block-start-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-block-start-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-block-start-shape
#[syntax(" <corner-shape-value>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-block-start-shape"))]
#[visit]
pub struct CornerBlockStartShapeStyleValue;

/// Represents the style value for `corner-block-end-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-block-end-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-block-end-shape
#[syntax(" <corner-shape-value>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-block-end-shape"))]
#[visit]
pub struct CornerBlockEndShapeStyleValue;

/// Represents the style value for `corner-inline-start-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-inline-start-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-inline-start-shape
#[syntax(" <corner-shape-value>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-inline-start-shape"))]
#[visit]
pub struct CornerInlineStartShapeStyleValue;

/// Represents the style value for `corner-inline-end-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-inline-end-shape).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-inline-end-shape
#[syntax(" <corner-shape-value>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-inline-end-shape"))]
#[visit]
pub struct CornerInlineEndShapeStyleValue;

// /// Represents the style value for `corner-top-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-left).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-top-left
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-top-left"))]
// #[visit]
// pub struct CornerTopLeftStyleValue;

// /// Represents the style value for `corner-top-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-right).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-top-right
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-top-right"))]
// #[visit]
// pub struct CornerTopRightStyleValue;

// /// Represents the style value for `corner-bottom-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-left).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-bottom-left
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-bottom-left"))]
// #[visit]
// pub struct CornerBottomLeftStyleValue;

// /// Represents the style value for `corner-bottom-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-right).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-bottom-right
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-bottom-right"))]
// #[visit]
// pub struct CornerBottomRightStyleValue;

// /// Represents the style value for `corner-start-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-start-start).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-start-start
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-start-start"))]
// #[visit]
// pub struct CornerStartStartStyleValue;

// /// Represents the style value for `corner-start-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-start-end).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-start-end
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-start-end"))]
// #[visit]
// pub struct CornerStartEndStyleValue;

// /// Represents the style value for `corner-end-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-end-start).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-end-start
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-end-start"))]
// #[visit]
// pub struct CornerEndStartStyleValue;

// /// Represents the style value for `corner-end-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-end-end).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-end-end
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-end-end"))]
// #[visit]
// pub struct CornerEndEndStyleValue;

// /// Represents the style value for `corner-top` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-top
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-top"))]
// #[visit]
// pub struct CornerTopStyleValue;

// /// Represents the style value for `corner-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-right).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-right
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-right"))]
// #[visit]
// pub struct CornerRightStyleValue;

// /// Represents the style value for `corner-bottom` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-bottom
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-bottom"))]
// #[visit]
// pub struct CornerBottomStyleValue;

// /// Represents the style value for `corner-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-left).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-left
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-left"))]
// #[visit]
// pub struct CornerLeftStyleValue;

// /// Represents the style value for `corner-block-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-block-start).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-block-start
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-block-start"))]
// #[visit]
// pub struct CornerBlockStartStyleValue;

// /// Represents the style value for `corner-block-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-block-end).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-block-end
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-block-end"))]
// #[visit]
// pub struct CornerBlockEndStyleValue;

// /// Represents the style value for `corner-inline-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-inline-start).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-inline-start
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-inline-start"))]
// #[visit]
// pub struct CornerInlineStartStyleValue;

// /// Represents the style value for `corner-inline-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-inline-end).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-inline-end
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-inline-end"))]
// #[visit]
// pub struct CornerInlineEndStyleValue;

// /// Represents the style value for `corner` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]? ] || <corner-shape-value>{1,4}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner
// #[syntax(" [ <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]? ] || <corner-shape-value>{1,4} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "0",
//   applies_to = "all elements (but see prose)",
// 	inherited = "no",
// 	percentages = "refer to corresponding dimension of the border box.",
// 	canonical_order = "per grammar",
// 	animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner"))]
// #[visit]
// pub struct CornerStyleValue;

// /// Represents the style value for `border-limit` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-limit).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// all | [ sides | corners ] <length-percentage [0,∞]>? | [ top | right | bottom | left ] <length-percentage [0,∞]>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-limit
// #[syntax(
// 	" all | [ sides | corners ] <length-percentage [0,∞]>? | [ top | right | bottom | left ] <length-percentage [0,∞]> "
// )]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "all",
//   applies_to = "all elements, except table element when border-collapse is collapse",
// 	inherited = "no",
// 	percentages = "relative to border-box",
// 	canonical_order = "per grammar",
// 	animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-limit"))]
// #[visit]
// pub enum BorderLimitStyleValue {}

/// Represents the style value for `border-clip` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-clip
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "all elements",
	inherited = "no",
	percentages = "refer to length of border-edge side",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-clip"))]
#[visit]
pub enum BorderClipStyleValue<'a> {}

/// Represents the style value for `border-clip-top` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip-top).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-clip-top
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "all elements",
	inherited = "no",
	percentages = "refer to length of border-edge side",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-clip-top"))]
#[visit]
pub enum BorderClipTopStyleValue<'a> {}

/// Represents the style value for `border-clip-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip-right).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-clip-right
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "all elements",
	inherited = "no",
	percentages = "refer to length of border-edge side",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-clip-right"))]
#[visit]
pub enum BorderClipRightStyleValue<'a> {}

/// Represents the style value for `border-clip-bottom` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip-bottom).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-clip-bottom
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "all elements",
	inherited = "no",
	percentages = "refer to length of border-edge side",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-clip-bottom"))]
#[visit]
pub enum BorderClipBottomStyleValue<'a> {}

/// Represents the style value for `border-clip-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip-left).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-clip-left
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "all elements",
	inherited = "no",
	percentages = "refer to length of border-edge side",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-clip-left"))]
#[visit]
pub enum BorderClipLeftStyleValue<'a> {}

/// Represents the style value for `box-shadow-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-color).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color>#
/// ```
///
// https://drafts.csswg.org/css-borders-4/#box-shadow-color
#[syntax(" <color># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "currentcolor",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-shadow-color"))]
#[visit]
pub struct BoxShadowColorStyleValue<'a>;

// /// Represents the style value for `box-shadow-offset` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-offset).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ none | <length>{2} ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#box-shadow-offset
// #[syntax(" [ none | <length>{2} ]# ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "n/a",
// 	canonical_order = "per grammar",
// 	animation_type = "by computed value, treating none as 0 0 when interpolated with non-none values.",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-shadow-offset"))]
// #[visit]
// pub struct BoxShadowOffsetStyleValue<'a>;

/// Represents the style value for `box-shadow-blur` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-blur).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length [0,∞]>#
/// ```
///
// https://drafts.csswg.org/css-borders-4/#box-shadow-blur
#[syntax(" <length [0,∞]># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-shadow-blur"))]
#[visit]
pub struct BoxShadowBlurStyleValue<'a>;

/// Represents the style value for `box-shadow-spread` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-spread).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>#
/// ```
///
// https://drafts.csswg.org/css-borders-4/#box-shadow-spread
#[syntax(" <length># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "0",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-shadow-spread"))]
#[visit]
pub struct BoxShadowSpreadStyleValue<'a>;

/// Represents the style value for `box-shadow-position` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-position).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ outset | inset ]#
/// ```
///
// https://drafts.csswg.org/css-borders-4/#box-shadow-position
#[syntax(" [ outset | inset ]# ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "outset",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-shadow-position"))]
#[visit]
pub struct BoxShadowPositionStyleValue<'a>;

/// Represents the style value for `box-shadow` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow).
///
/// The box-shadow CSS property applies shadow effects around an element's frame. This can create drop shadow and inner shadow effects.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <spread-shadow>#
/// ```
///
// https://drafts.csswg.org/css-borders-4/#box-shadow
#[syntax(" <spread-shadow># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-shadow"))]
#[visit]
pub struct BoxShadowStyleValue<'a>;

// /// Represents the style value for `border-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-shape).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ <basic-shape> <geometry-box>?]{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-shape
// #[syntax(" none | [ <basic-shape> <geometry-box>?]{1,2} ")]
// #[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[style_value(
// 	initial = "none",
//   applies_to = "all elements",
// 	inherited = "no",
// 	percentages = "relative to the given <geometry-box>, or to border box if not given.",
// 	canonical_order = "per grammar",
// 	animation_type = "by computed value",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-shape"))]
// #[visit]
// pub enum BorderShapeStyleValue {}
