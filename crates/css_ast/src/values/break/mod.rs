#![allow(warnings)]
//! CSS Fragmentation Module Level 4  Breaking the Web, one fragment at a time
//! https://drafts.csswg.org/css-break-4/

mod impls;
use impls::*;

/// Represents the style value for `break-before` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#break-before).
///
/// In printed page layouts, the break-after, break-before, break-inside CSS properties control where printed pages start and end. Also known as pagination or page breaking.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | avoid | always | all | avoid-page | page | left | right | recto | verso | avoid-column | column | avoid-region | region
/// ```
///
// https://drafts.csswg.org/css-break-4/#break-before
#[syntax(
	" auto | avoid | always | all | avoid-page | page | left | right | recto | verso | avoid-column | column | avoid-region | region "
)]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "block-level boxes, grid items, flex items, table row groups, table rows (but see prose)",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.break-before"))]
#[visit]
pub enum BreakBeforeStyleValue {}

/// Represents the style value for `break-after` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#break-after).
///
/// In printed page layouts, the break-after, break-before, break-inside CSS properties control where printed pages start and end. Also known as pagination or page breaking.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | avoid | always | all | avoid-page | page | left | right | recto | verso | avoid-column | column | avoid-region | region
/// ```
///
// https://drafts.csswg.org/css-break-4/#break-after
#[syntax(
	" auto | avoid | always | all | avoid-page | page | left | right | recto | verso | avoid-column | column | avoid-region | region "
)]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "block-level boxes, grid items, flex items, table row groups, table rows (but see prose)",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.break-after"))]
#[visit]
pub enum BreakAfterStyleValue {}

/// Represents the style value for `break-inside` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#break-inside).
///
/// In printed page layouts, the break-after, break-before, break-inside CSS properties control where printed pages start and end. Also known as pagination or page breaking.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | avoid | avoid-page | avoid-column | avoid-region
/// ```
///
// https://drafts.csswg.org/css-break-4/#break-inside
#[syntax(" auto | avoid | avoid-page | avoid-column | avoid-region ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements except inline-level boxes, internal ruby boxes, table column boxes, table column group boxes, absolutely-positioned boxes",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.break-inside"))]
#[visit]
pub enum BreakInsideStyleValue {}

/// Represents the style value for `orphans` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#orphans).
///
/// The widows and orphans CSS properties set the minimum lines included in a text fragment created by page, column, or region breaks.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <integer [1,∞]>
/// ```
///
// https://drafts.csswg.org/css-break-4/#orphans
#[syntax(" <integer [1,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "2",
	applies_to = "block containers that establish an inline formatting context",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.orphans"))]
#[visit]
pub struct OrphansStyleValue;

/// Represents the style value for `widows` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#widows).
///
/// The widows and orphans CSS properties set the minimum lines included in a text fragment created by page, column, or region breaks.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <integer [1,∞]>
/// ```
///
// https://drafts.csswg.org/css-break-4/#widows
#[syntax(" <integer [1,∞]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "2",
	applies_to = "block containers that establish an inline formatting context",
	inherited = "yes",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.widows"))]
#[visit]
pub struct WidowsStyleValue;

/// Represents the style value for `box-decoration-break` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#box-decoration-break).
///
/// The box-decoration-break CSS property sets whether box decorations, such as borders or backgrounds, of an element divided across a page, column, or region wraps each fragment or splits across the break.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// slice | clone
/// ```
///
// https://drafts.csswg.org/css-break-4/#box-decoration-break
#[syntax(" slice | clone ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "slice",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.box-decoration-break"))]
#[visit]
pub enum BoxDecorationBreakStyleValue {}

/// Represents the style value for `margin-break` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#margin-break).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | keep | discard
/// ```
///
// https://drafts.csswg.org/css-break-4/#margin-break
#[syntax(" auto | keep | discard ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.margin-break"))]
#[visit]
pub enum MarginBreakStyleValue {}
