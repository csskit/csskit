#![allow(warnings)]
//! https://drafts.csswg.org/css-logical-1/

mod impls;
use super::prelude::*;
use impls::*;
/// Represents the style value for `block-size` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#block-size).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'width'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#block-size
#[syntax(" <'width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Size,
    box_portion = Size,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.block-size"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct BlockSizeStyleValue;

/// Represents the style value for `inline-size` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#inline-size).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'width'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#inline-size
#[syntax(" <'width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Size,
    box_portion = Size,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.inline-size"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct InlineSizeStyleValue;

/// Represents the style value for `margin-block` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#margin-block).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>{1,2}
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#margin-block
#[syntax(" <'margin-top'>{1,2} ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    animation_type = Unknown,
    percentages = Unknown,
    longhands = MarginBlockEnd|MarginBlockStart,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = BlockStart|BlockEnd,
    box_portion = Margin,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.margin-block"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct MarginBlockStyleValue;

/// Represents the style value for `margin-block-end` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#margin-block-end).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#margin-block-end
#[syntax(" <'margin-top'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    shorthand_group = MarginBlock,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Margin,
    box_side = BlockEnd,
    box_portion = Margin,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.margin-block-end"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct MarginBlockEndStyleValue;

/// Represents the style value for `margin-block-start` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#margin-block-start).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#margin-block-start
#[syntax(" <'margin-top'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    shorthand_group = MarginBlock,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Margin,
    box_side = BlockStart,
    box_portion = Margin,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.margin-block-start"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct MarginBlockStartStyleValue;

/// Represents the style value for `margin-inline` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#margin-inline).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>{1,2}
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#margin-inline
#[syntax(" <'margin-top'>{1,2} ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    animation_type = Unknown,
    percentages = Unknown,
    longhands = MarginInlineEnd|MarginInlineStart,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = InlineStart|InlineEnd,
    box_portion = Margin,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.margin-inline"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct MarginInlineStyleValue;

/// Represents the style value for `margin-inline-end` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#margin-inline-end).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#margin-inline-end
#[syntax(" <'margin-top'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    shorthand_group = MarginInline,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Margin,
    box_side = InlineEnd,
    box_portion = Margin,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.margin-inline-end"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct MarginInlineEndStyleValue;

/// Represents the style value for `margin-inline-start` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#margin-inline-start).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#margin-inline-start
#[syntax(" <'margin-top'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    shorthand_group = MarginInline,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Margin,
    box_side = InlineStart,
    box_portion = Margin,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.margin-inline-start"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct MarginInlineStartStyleValue;

/// Represents the style value for `max-block-size` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#max-block-size).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'max-width'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#max-block-size
#[syntax(" <'max-width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = MaxSize,
    box_side = BlockStart|BlockEnd,
    box_portion = Size,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.max-block-size"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct MaxBlockSizeStyleValue;

/// Represents the style value for `max-inline-size` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#max-inline-size).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'max-width'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#max-inline-size
#[syntax(" <'max-width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = MaxSize,
    box_side = InlineStart|InlineEnd,
    box_portion = Size,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.max-inline-size"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct MaxInlineSizeStyleValue;

/// Represents the style value for `min-block-size` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#min-block-size).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'min-width'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#min-block-size
#[syntax(" <'min-width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = MinSize,
    box_side = BlockStart|BlockEnd,
    box_portion = Size,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.min-block-size"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct MinBlockSizeStyleValue;

/// Represents the style value for `min-inline-size` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#min-inline-size).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'min-width'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#min-inline-size
#[syntax(" <'min-width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = MinSize,
    box_side = InlineStart|InlineEnd,
    box_portion = Size,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.min-inline-size"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct MinInlineSizeStyleValue;

/// Represents the style value for `padding-block` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#padding-block).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>{1,2}
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#padding-block
#[syntax(" <'padding-top'>{1,2} ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    animation_type = Unknown,
    percentages = Unknown,
    longhands = PaddingBlockEnd|PaddingBlockStart,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = BlockStart|BlockEnd,
    box_portion = Padding,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.padding-block"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct PaddingBlockStyleValue;

/// Represents the style value for `padding-block-end` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#padding-block-end).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#padding-block-end
#[syntax(" <'padding-top'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    shorthand_group = PaddingBlock,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Padding,
    box_side = BlockEnd,
    box_portion = Padding,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.padding-block-end"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct PaddingBlockEndStyleValue;

/// Represents the style value for `padding-block-start` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#padding-block-start).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#padding-block-start
#[syntax(" <'padding-top'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    shorthand_group = PaddingBlock,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Padding,
    box_side = BlockStart,
    box_portion = Padding,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.padding-block-start"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct PaddingBlockStartStyleValue;

/// Represents the style value for `padding-inline` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#padding-inline).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>{1,2}
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#padding-inline
#[syntax(" <'padding-top'>{1,2} ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    animation_type = Unknown,
    percentages = Unknown,
    longhands = PaddingInlineEnd|PaddingInlineStart,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_side = InlineStart|InlineEnd,
    box_portion = Padding,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.padding-inline"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct PaddingInlineStyleValue;

/// Represents the style value for `padding-inline-end` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#padding-inline-end).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#padding-inline-end
#[syntax(" <'padding-top'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    shorthand_group = PaddingInline,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Padding,
    box_side = InlineEnd,
    box_portion = Padding,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.padding-inline-end"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct PaddingInlineEndStyleValue;

/// Represents the style value for `padding-inline-start` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#padding-inline-start).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>
/// ```
///
/// https://drafts.csswg.org/css-logical-1/#padding-inline-start
#[syntax(" <'padding-top'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    shorthand_group = PaddingInline,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Padding,
    box_side = InlineStart,
    box_portion = Padding,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.padding-inline-start"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct PaddingInlineStartStyleValue;
