#![allow(warnings)]
//! https://drafts.csswg.org/css-borders-4/

mod impls;
use super::prelude::*;
use impls::*;
/// Represents the style value for `border` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#border
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = Top|Bottom|Left|Right,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-block
#[syntax(" <'border-block-start'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = BlockStart|BlockEnd,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBlockStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-block-color
#[syntax(" <'border-top-color'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = BlockStart|BlockEnd,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBlockColorStyleValue<'a>;

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
/// https://drafts.csswg.org/css-borders-4/#border-block-end
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "See individual properties",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = BlockEnd,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-end"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBlockEndStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-block-end-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "currentcolor",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderColor,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-end-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderBlockEndColorStyleValue<'a> {}

// /// Represents the style value for `border-block-end-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-end-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-block-end-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-block-end-radius")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderBlockEndRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-block-end-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderStyle,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-end-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBlockEndStyleStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-block-end-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "medium",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = AbsoluteLengthOrNone,
    canonical_order = "per grammar",
    logical_property_group = BorderWidth,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-end-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBlockEndWidthStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-block-start
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "See individual properties",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = BlockStart,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-start"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBlockStartStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-block-start-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "currentcolor",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderColor,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-start-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderBlockStartColorStyleValue<'a> {}

// /// Represents the style value for `border-block-start-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-start-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-block-start-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-block-start-radius")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderBlockStartRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-block-start-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderStyle,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-start-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBlockStartStyleStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-block-start-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "medium",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = AbsoluteLengthOrNone,
    canonical_order = "per grammar",
    logical_property_group = BorderWidth,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-start-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBlockStartWidthStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-block-style
#[syntax(" <'border-top-style'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = BlockStart|BlockEnd,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBlockStyleStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-block-width
#[syntax(" <'border-top-width'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = BlockStart|BlockEnd,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-block-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBlockWidthStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-bottom
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "See individual properties",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = Bottom,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-bottom"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBottomStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-bottom-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "currentcolor",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderColor,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-bottom-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderBottomColorStyleValue<'a> {}

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
/// https://drafts.csswg.org/css-borders-4/#border-bottom-left-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    percentages = BorderBox,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = Bottom|Left,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-bottom-left-radius"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBottomLeftRadiusStyleValue;

// /// Represents the style value for `border-bottom-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-bottom-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-bottom-radius")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderBottomRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-bottom-right-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    percentages = BorderBox,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = Bottom|Right,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(
	feature = "css_feature_data",
	derive(ToCSSFeature),
	css_feature("css.properties.border-bottom-right-radius")
)]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBottomRightRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-bottom-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderStyle,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-bottom-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBottomStyleStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-bottom-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "medium",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = AbsoluteLengthOrNone,
    canonical_order = "per grammar",
    logical_property_group = BorderWidth,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-bottom-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderBottomWidthStyleValue;

/// Represents the style value for `border-clip` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#border-clip
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    percentages = BorderEdge,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = SpecifiedWithAbsoluteLengths,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-clip"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderClipStyleValue<'a> {}

/// Represents the style value for `border-clip-bottom` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip-bottom).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#border-clip-bottom
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    percentages = BorderEdge,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = SpecifiedWithAbsoluteLengths,
    canonical_order = "per grammar",
    box_side = Bottom,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-clip-bottom"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderClipBottomStyleValue<'a> {}

/// Represents the style value for `border-clip-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip-left).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#border-clip-left
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    percentages = BorderEdge,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = SpecifiedWithAbsoluteLengths,
    canonical_order = "per grammar",
    box_side = Left,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-clip-left"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderClipLeftStyleValue<'a> {}

/// Represents the style value for `border-clip-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip-right).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#border-clip-right
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    percentages = BorderEdge,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = SpecifiedWithAbsoluteLengths,
    canonical_order = "per grammar",
    box_side = Right,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-clip-right"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderClipRightStyleValue<'a> {}

/// Represents the style value for `border-clip-top` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip-top).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#border-clip-top
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    percentages = BorderEdge,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = SpecifiedWithAbsoluteLengths,
    canonical_order = "per grammar",
    box_side = Top,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-clip-top"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderClipTopStyleValue<'a> {}

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
// /// https://drafts.csswg.org/css-borders-4/#border-color
// #[syntax(" [ <color> | <image-1D> ]{1,4} ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "see individual properties",
//     inherits = Unknown,
//     applies_to = Unknown,
//     percentages = Unknown,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = Top|Bottom|Left|Right,
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-color")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderColorStyleValue<'a>;

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
/// https://drafts.csswg.org/css-borders-4/#border-end-end-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    percentages = BorderBox,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = BlockEnd|InlineEnd,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-end-end-radius"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderEndEndRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-end-start-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    percentages = BorderBox,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = BlockEnd|InlineStart,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-end-start-radius"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderEndStartRadiusStyleValue;

// /// Represents the style value for `border-image` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-image).
// ///
// /// The border-image CSS property draws an image around an element.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-image-source'> || <'border-image-slice'> [ / <'border-image-width'> | / <'border-image-width'>? / <'border-image-outset'> ]? || <'border-image-repeat'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-image
// #[syntax(
//     " <'border-image-source'> || <'border-image-slice'> [ / <'border-image-width'> | / <'border-image-width'>? / <'border-image-outset'> ]? || <'border-image-repeat'> "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "See individual properties",
//     applies_to = Unknown,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-image")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderImageStyleValue;

/// Represents the style value for `border-image-outset` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-image-outset).
///
/// The border-image CSS property draws an image around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ <length [0,∞]> | <number [0,∞]> ]{1,4}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#border-image-outset
#[syntax(" [ <length [0,∞]> | <number [0,∞]> ]{1,4} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = AbsoluteLength,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-image-outset"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderImageOutsetStyleValue;

/// Represents the style value for `border-image-repeat` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-image-repeat).
///
/// The border-image CSS property draws an image around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ stretch | repeat | round | space ]{1,2}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#border-image-repeat
#[syntax(" [ stretch | repeat | round | space ]{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "stretch",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-image-repeat"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderImageRepeatStyleValue;

// /// Represents the style value for `border-image-slice` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-image-slice).
// ///
// /// The border-image CSS property draws an image around an element.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [<number [0,∞]> | <percentage [0,∞]>]{1,4} && fill?
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-image-slice
// #[syntax(" [<number [0,∞]> | <percentage [0,∞]>]{1,4} && fill? ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "100%",
//     applies_to = Unknown,
//     percentages = BorderImageArea,
//     animation_type = ByComputedValue,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-image-slice")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderImageSliceStyleValue;

/// Represents the style value for `border-image-source` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-image-source).
///
/// The border-image CSS property draws an image around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <image>
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#border-image-source
#[syntax(" none | <image> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-image-source"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderImageSourceStyleValue<'a>;

// /// Represents the style value for `border-image-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-image-width).
// ///
// /// The border-image CSS property draws an image around an element.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]> | <number [0,∞]> | auto ]{1,4}
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-image-width
// #[syntax(" [ <length-percentage [0,∞]> | <number [0,∞]> | auto ]{1,4} ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "1",
//     applies_to = Unknown,
//     percentages = BorderImageArea,
//     animation_type = ByComputedValue,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-image-width")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderImageWidthStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-inline
#[syntax(" <'border-block-start'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = InlineStart|InlineEnd,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderInlineStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-inline-color
#[syntax(" <'border-top-color'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = InlineStart|InlineEnd,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderInlineColorStyleValue<'a>;

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
/// https://drafts.csswg.org/css-borders-4/#border-inline-end
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "See individual properties",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = InlineEnd,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-end"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderInlineEndStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-inline-end-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "currentcolor",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderColor,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-end-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderInlineEndColorStyleValue<'a> {}

// /// Represents the style value for `border-inline-end-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-end-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-inline-end-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-inline-end-radius")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderInlineEndRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-inline-end-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderStyle,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-end-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderInlineEndStyleStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-inline-end-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "medium",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = AbsoluteLengthOrNone,
    canonical_order = "per grammar",
    logical_property_group = BorderWidth,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-end-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderInlineEndWidthStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-inline-start
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "See individual properties",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = InlineStart,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-start"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderInlineStartStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-inline-start-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "currentcolor",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderColor,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-start-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderInlineStartColorStyleValue<'a> {}

// /// Represents the style value for `border-inline-start-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-start-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-inline-start-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-inline-start-radius")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderInlineStartRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-inline-start-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderStyle,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-start-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderInlineStartStyleStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-inline-start-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "medium",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = AbsoluteLengthOrNone,
    canonical_order = "per grammar",
    logical_property_group = BorderWidth,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-start-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderInlineStartWidthStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-inline-style
#[syntax(" <'border-top-style'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = InlineStart|InlineEnd,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderInlineStyleStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-inline-width
#[syntax(" <'border-top-width'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = InlineStart|InlineEnd,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-inline-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderInlineWidthStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-left
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "See individual properties",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = Left,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-left"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderLeftStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-left-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "currentcolor",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderColor,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-left-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderLeftColorStyleValue<'a> {}

// /// Represents the style value for `border-left-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-left-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-left-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-left-radius")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderLeftRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-left-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderStyle,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-left-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderLeftStyleStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-left-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "medium",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = AbsoluteLengthOrNone,
    canonical_order = "per grammar",
    logical_property_group = BorderWidth,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-left-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderLeftWidthStyleValue;

// /// Represents the style value for `border-limit` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-limit).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /**all | [ sides | corners ] <length-percentage [0,∞]>?
// | [ top | right | bottom | left ] <length-percentage [0,∞]>*/
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-limit
// #[syntax(
//     " all | [ sides | corners ] <length-percentage [0,∞]>? | [ top | right | bottom | left ] <length-percentage [0,∞]> "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "all",
//     applies_to = Unknown,
//     percentages = BorderBox,
//     animation_type = Discrete,
//     property_group = Borders,
//     computed_value_type = AsSpecified,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-limit")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum BorderLimitStyleValue {}

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
// /// https://drafts.csswg.org/css-borders-4/#border-radius
// #[syntax(" <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]? ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "see individual properties",
//     inherits = Unknown,
//     applies_to = Unknown,
//     percentages = Unknown,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-radius")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-right
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "See individual properties",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = Right,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-right"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderRightStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-right-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "currentcolor",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderColor,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-right-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderRightColorStyleValue<'a> {}

// /// Represents the style value for `border-right-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-right-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-right-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-right-radius")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderRightRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-right-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderStyle,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-right-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderRightStyleStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-right-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "medium",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = AbsoluteLengthOrNone,
    canonical_order = "per grammar",
    logical_property_group = BorderWidth,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-right-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderRightWidthStyleValue;

// /// Represents the style value for `border-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-shape).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ <basic-shape> <geometry-box>?]{1,2}
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-shape
// #[syntax(" none | [ <basic-shape> <geometry-box>?]{1,2} ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "none",
//     applies_to = Elements,
//     percentages = Unknown,
//     animation_type = ByComputedValue,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-shape")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderShapeStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-start-end-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    percentages = BorderBox,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = BlockStart|InlineEnd,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-start-end-radius"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderStartEndRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-start-start-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    percentages = BorderBox,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = BlockStart|InlineStart,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-start-start-radius"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderStartStartRadiusStyleValue;

/// Represents the style value for `border-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-style).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-style'>{1,4}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#border-style
#[syntax(" <'border-top-style'>{1,4} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = Top|Bottom|Left|Right,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderStyleStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-top
#[syntax(" <line-width> || <line-style> || <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "See individual properties",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = Top,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderTopStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-top-color
#[syntax(" <color> | <image-1D> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "currentcolor",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderColor,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum BorderTopColorStyleValue<'a> {}

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
/// https://drafts.csswg.org/css-borders-4/#border-top-left-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    percentages = BorderBox,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = Top|Left,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top-left-radius"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderTopLeftRadiusStyleValue;

// /// Represents the style value for `border-top-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-radius).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#border-top-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_portion = Border,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.border-top-radius")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BorderTopRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-top-right-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    percentages = BorderBox,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = Top|Right,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top-right-radius"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderTopRightRadiusStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-top-style
#[syntax(" <line-style> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderStyle,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderTopStyleStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#border-top-width
#[syntax(" <line-width> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "medium",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = AbsoluteLengthOrNone,
    canonical_order = "per grammar",
    logical_property_group = BorderWidth,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-top-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderTopWidthStyleValue;

/// Represents the style value for `border-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-width).
///
/// The border CSS property sets the color, style, and width of the line around an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-width'>{1,4}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#border-width
#[syntax(" <'border-top-width'>{1,4} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = Top|Bottom|Left|Right,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.border-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BorderWidthStyleValue;

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
/// https://drafts.csswg.org/css-borders-4/#box-shadow
#[syntax(" <spread-shadow># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-shadow"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BoxShadowStyleValue<'a>;

/// Represents the style value for `box-shadow-blur` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-blur).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length [0,∞]>#
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#box-shadow-blur
#[syntax(" <length [0,∞]># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-shadow-blur"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BoxShadowBlurStyleValue<'a>;

/// Represents the style value for `box-shadow-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-color).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color>#
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#box-shadow-color
#[syntax(" <color># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "currentcolor",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-shadow-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BoxShadowColorStyleValue<'a>;

// /// Represents the style value for `box-shadow-offset` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-offset).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ none | <length>{2} ]#
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#box-shadow-offset
// #[syntax(" [ none | <length>{2} ]# ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "none",
//     applies_to = Elements,
//     animation_type = ByComputedValue,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.box-shadow-offset")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct BoxShadowOffsetStyleValue<'a>;

/// Represents the style value for `box-shadow-position` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-position).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ outset | inset ]#
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#box-shadow-position
#[syntax(" [ outset | inset ]# ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "outset",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-shadow-position"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BoxShadowPositionStyleValue<'a>;

/// Represents the style value for `box-shadow-spread` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-spread).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>#
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#box-shadow-spread
#[syntax(" <length># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-shadow-spread"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BoxShadowSpreadStyleValue<'a>;

// /// Represents the style value for `corner` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-radius'> || <'corner-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner
// #[syntax(" <'border-radius'> || <'corner-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerStyleValue;

// /// Represents the style value for `corner-block-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-block-end).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-radius'> || <'corner-top-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-block-end
// #[syntax(" <'border-top-radius'> || <'corner-top-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = BlockEnd,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-block-end")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerBlockEndStyleValue;

/// Represents the style value for `corner-block-end-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-block-end-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'corner-top-left-shape'>{1,2}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-block-end-shape
#[syntax(" <'corner-top-left-shape'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-block-end-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerBlockEndShapeStyleValue;

// /// Represents the style value for `corner-block-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-block-start).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-radius'> || <'corner-top-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-block-start
// #[syntax(" <'border-top-radius'> || <'corner-top-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = BlockStart,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-block-start")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerBlockStartStyleValue;

/// Represents the style value for `corner-block-start-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-block-start-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'corner-top-left-shape'>{1,2}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-block-start-shape
#[syntax(" <'corner-top-left-shape'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-block-start-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerBlockStartShapeStyleValue;

// /// Represents the style value for `corner-bottom` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-radius'> || <'corner-top-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-bottom
// #[syntax(" <'border-top-radius'> || <'corner-top-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = Bottom,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-bottom")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerBottomStyleValue;

// /// Represents the style value for `corner-bottom-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-left).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-left-radius'> || <'corner-top-left-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-bottom-left
// #[syntax(" <'border-top-left-radius'> || <'corner-top-left-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = Bottom|Left,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-bottom-left")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerBottomLeftStyleValue;

/// Represents the style value for `corner-bottom-left-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-left-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-bottom-left-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "round",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = CornerShape,
    box_side = Bottom|Left,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-bottom-left-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerBottomLeftShapeStyleValue;

// /// Represents the style value for `corner-bottom-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-right).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-left-radius'> || <'corner-top-left-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-bottom-right
// #[syntax(" <'border-top-left-radius'> || <'corner-top-left-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = Bottom|Right,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-bottom-right")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerBottomRightStyleValue;

/// Represents the style value for `corner-bottom-right-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-right-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-bottom-right-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "round",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = CornerShape,
    box_side = Bottom|Right,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-bottom-right-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerBottomRightShapeStyleValue;

/// Represents the style value for `corner-bottom-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'corner-top-left-shape'>{1,2}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-bottom-shape
#[syntax(" <'corner-top-left-shape'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-bottom-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerBottomShapeStyleValue;

// /// Represents the style value for `corner-end-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-end-end).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-left-radius'> || <'corner-top-left-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-end-end
// #[syntax(" <'border-top-left-radius'> || <'corner-top-left-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = BlockEnd|InlineEnd,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-end-end")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerEndEndStyleValue;

/// Represents the style value for `corner-end-end-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-end-end-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-end-end-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "round",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = CornerShape,
    box_side = BlockEnd|InlineEnd,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-end-end-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerEndEndShapeStyleValue;

// /// Represents the style value for `corner-end-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-end-start).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-left-radius'> || <'corner-top-left-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-end-start
// #[syntax(" <'border-top-left-radius'> || <'corner-top-left-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = BlockEnd|InlineStart,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-end-start")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerEndStartStyleValue;

/// Represents the style value for `corner-end-start-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-end-start-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-end-start-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "round",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = CornerShape,
    box_side = BlockEnd|InlineStart,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-end-start-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerEndStartShapeStyleValue;

// /// Represents the style value for `corner-inline-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-inline-end).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-radius'> || <'corner-top-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-inline-end
// #[syntax(" <'border-top-radius'> || <'corner-top-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = InlineEnd,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-inline-end")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerInlineEndStyleValue;

/// Represents the style value for `corner-inline-end-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-inline-end-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'corner-top-left-shape'>{1,2}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-inline-end-shape
#[syntax(" <'corner-top-left-shape'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-inline-end-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerInlineEndShapeStyleValue;

// /// Represents the style value for `corner-inline-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-inline-start).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-radius'> || <'corner-top-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-inline-start
// #[syntax(" <'border-top-radius'> || <'corner-top-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = InlineStart,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-inline-start")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerInlineStartStyleValue;

/// Represents the style value for `corner-inline-start-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-inline-start-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'corner-top-left-shape'>{1,2}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-inline-start-shape
#[syntax(" <'corner-top-left-shape'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-inline-start-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerInlineStartShapeStyleValue;

// /// Represents the style value for `corner-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-left).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-radius'> || <'corner-top-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-left
// #[syntax(" <'border-top-radius'> || <'corner-top-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = Left,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-left")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerLeftStyleValue;

/// Represents the style value for `corner-left-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-left-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'corner-top-left-shape'>{1,2}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-left-shape
#[syntax(" <'corner-top-left-shape'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-left-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerLeftShapeStyleValue;

// /// Represents the style value for `corner-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-right).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-radius'> || <'corner-top-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-right
// #[syntax(" <'border-top-radius'> || <'corner-top-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = Right,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-right")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerRightStyleValue;

/// Represents the style value for `corner-right-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-right-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'corner-top-left-shape'>{1,2}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-right-shape
#[syntax(" <'corner-top-left-shape'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-right-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerRightShapeStyleValue;

/// Represents the style value for `corner-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'corner-top-left-shape'>{1,4}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-shape
#[syntax(" <'corner-top-left-shape'>{1,4} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "round",
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerShapeStyleValue;

// /// Represents the style value for `corner-start-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-start-end).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-left-radius'> || <'corner-top-left-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-start-end
// #[syntax(" <'border-top-left-radius'> || <'corner-top-left-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = BlockStart|InlineEnd,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-start-end")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerStartEndStyleValue;

/// Represents the style value for `corner-start-end-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-start-end-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-start-end-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "round",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = CornerShape,
    box_side = BlockStart|InlineEnd,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-start-end-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerStartEndShapeStyleValue;

// /// Represents the style value for `corner-start-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-start-start).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-left-radius'> || <'corner-top-left-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-start-start
// #[syntax(" <'border-top-left-radius'> || <'corner-top-left-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = BlockStart|InlineStart,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-start-start")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerStartStartStyleValue;

/// Represents the style value for `corner-start-start-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-start-start-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-start-start-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "round",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = CornerShape,
    box_side = BlockStart|InlineStart,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-start-start-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerStartStartShapeStyleValue;

// /// Represents the style value for `corner-top` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-radius'> || <'corner-top-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-top
// #[syntax(" <'border-top-radius'> || <'corner-top-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = Top,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-top")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerTopStyleValue;

// /// Represents the style value for `corner-top-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-left).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-left-radius'> || <'corner-top-left-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-top-left
// #[syntax(" <'border-top-left-radius'> || <'corner-top-left-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = Top|Left,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-top-left")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerTopLeftStyleValue;

/// Represents the style value for `corner-top-left-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-left-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-top-left-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "round",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = CornerShape,
    box_side = Top|Left,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-top-left-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerTopLeftShapeStyleValue;

// /// Represents the style value for `corner-top-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-right).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-top-left-radius'> || <'corner-top-left-shape'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-borders-4/#corner-top-right
// #[syntax(" <'border-top-left-radius'> || <'corner-top-left-shape'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "0",
//     applies_to = Elements,
//     percentages = BorderBox,
//     animation_type = Unknown,
//     property_group = Borders,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
//     box_side = Top|Right,
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.corner-top-right")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct CornerTopRightStyleValue;

/// Represents the style value for `corner-top-right-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-right-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-top-right-shape
#[syntax(" <corner-shape-value> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "round",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = CornerShape,
    box_side = Top|Right,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-top-right-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerTopRightShapeStyleValue;

/// Represents the style value for `corner-top-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-shape).
///
/// The corner-shape CSS property sets the shape of an element's corners when using border-radius, allowing for shapes other than rounded corners. For example, corner-shape: squircle is a shape in between a square and rounded corner.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'corner-top-left-shape'>{1,2}
/// ```
///
/// https://drafts.csswg.org/css-borders-4/#corner-top-shape
#[syntax(" <'corner-top-left-shape'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.corner-top-shape"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct CornerTopShapeStyleValue;
