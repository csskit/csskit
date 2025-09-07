#![allow(warnings)]
//! CSS Positioned Layout Module Level 4
//! https://drafts.csswg.org/css-position-4/

mod impls;
use impls::*;

/// Represents the style value for `position` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#position).
///
/// The position CSS property sets the origin position of an element to an element, the element's scrollport, or the viewport.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// static | relative | absolute | sticky | fixed
/// ```
///
// https://drafts.csswg.org/css-position-4/#position
#[syntax(" static | relative | absolute | sticky | fixed ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "static",
	applies_to = "all elements except table-column-group and table-column",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.position"))]
#[visit]
pub enum PositionStyleValue {}

/// Represents the style value for `top` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#top).
///
/// The physical CSS properties, top, right, bottom, and left, set the inset position of an element relative to the corresponding side of a container determined by the element's position property.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#top
#[syntax(" auto | <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "positioned elements",
	inherited = "no",
	percentages = "refer to size of containing block; see prose",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.top"))]
#[visit]
pub struct TopStyleValue;

/// Represents the style value for `right` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#right).
///
/// The physical CSS properties, top, right, bottom, and left, set the inset position of an element relative to the corresponding side of a container determined by the element's position property.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#right
#[syntax(" auto | <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "positioned elements",
	inherited = "no",
	percentages = "refer to size of containing block; see prose",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.right"))]
#[visit]
pub struct RightStyleValue;

/// Represents the style value for `bottom` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#bottom).
///
/// The physical CSS properties, top, right, bottom, and left, set the inset position of an element relative to the corresponding side of a container determined by the element's position property.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#bottom
#[syntax(" auto | <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "positioned elements",
	inherited = "no",
	percentages = "refer to size of containing block; see prose",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.bottom"))]
#[visit]
pub struct BottomStyleValue;

/// Represents the style value for `left` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#left).
///
/// The physical CSS properties, top, right, bottom, and left, set the inset position of an element relative to the corresponding side of a container determined by the element's position property.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#left
#[syntax(" auto | <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "positioned elements",
	inherited = "no",
	percentages = "refer to size of containing block; see prose",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.left"))]
#[visit]
pub struct LeftStyleValue;

/// Represents the style value for `inset-block-start` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset-block-start).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset-block-start
#[syntax(" auto | <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "positioned elements",
	inherited = "no",
	percentages = "refer to size of containing block; see prose",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.inset-block-start"))]
#[visit]
pub struct InsetBlockStartStyleValue;

/// Represents the style value for `inset-inline-start` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset-inline-start).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset-inline-start
#[syntax(" auto | <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "positioned elements",
	inherited = "no",
	percentages = "refer to size of containing block; see prose",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.inset-inline-start"))]
#[visit]
pub struct InsetInlineStartStyleValue;

/// Represents the style value for `inset-block-end` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset-block-end).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset-block-end
#[syntax(" auto | <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "positioned elements",
	inherited = "no",
	percentages = "refer to size of containing block; see prose",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.inset-block-end"))]
#[visit]
pub struct InsetBlockEndStyleValue;

/// Represents the style value for `inset-inline-end` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset-inline-end).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset-inline-end
#[syntax(" auto | <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "positioned elements",
	inherited = "no",
	percentages = "refer to size of containing block; see prose",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.inset-inline-end"))]
#[visit]
pub struct InsetInlineEndStyleValue;

/// Represents the style value for `inset-block` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset-block).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'top'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset-block
#[syntax(" <'top'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "positioned elements",
	inherited = "no",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.inset-block"))]
#[visit]
pub struct InsetBlockStyleValue;

/// Represents the style value for `inset-inline` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset-inline).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'top'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset-inline
#[syntax(" <'top'>{1,2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "positioned elements",
	inherited = "no",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.inset-inline"))]
#[visit]
pub struct InsetInlineStyleValue;

/// Represents the style value for `inset` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset).
///
/// CSS logical properties control borders, size, margin, and padding with directions and dimensions relative to the writing mode. For example, in a left to right, top to bottom writing mode, block-end refers to the bottom. Also known as flow relative.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'top'>{1,4}
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset
#[syntax(" <'top'>{1,4} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "positioned elements",
	inherited = "no",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.inset"))]
#[visit]
pub struct InsetStyleValue;

/// Represents the style value for `overlay` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#overlay).
///
/// The overlay CSS property, used as an allow-discrete CSS transition, prevents a top layer element, such as a popover or a <dialog>, from being removed from the top layer before it has finished animating. You can't set the value of the overlay property; only use it as transition property.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | auto
/// ```
///
// https://drafts.csswg.org/css-position-4/#overlay
#[syntax(" none | auto ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "see prose"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.overlay"))]
#[visit]
pub enum OverlayStyleValue {}
