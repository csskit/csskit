#![allow(warnings)]
//! https://drafts.csswg.org/css-gaps-1/

mod impls;
use super::prelude::*;
use impls::*;
/// Represents the style value for `column-rule` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#column-rule).
///
/// Multi-column layout flows an element's content across one or more columns in a single row, without affecting the display property of its children.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <gap-rule-list> | <gap-auto-rule-list>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#column-rule
#[syntax(" <gap-rule-list> | <gap-auto-rule-list> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.column-rule"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum ColumnRuleStyleValue {}

/// Represents the style value for `column-rule-break` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#column-rule-break).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | spanning-item | intersection
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#column-rule-break
#[syntax(" none | spanning-item | intersection ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "spanning-item",
	applies_to = "grid containers, flex containers, multicol containers, and masonry containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.column-rule-break"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum ColumnRuleBreakStyleValue {}

/// Represents the style value for `column-rule-color` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#column-rule-color).
///
/// Multi-column layout flows an element's content across one or more columns in a single row, without affecting the display property of its children.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-color-list> | <auto-line-color-list>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#column-rule-color
#[syntax(" <line-color-list> | <auto-line-color-list> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "currentcolor",
	applies_to = "grid containers, flex containers, multicol containers, and masonry containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "repeatable list, see §\u{202f}3.4.1 interpolation behavior."
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.column-rule-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum ColumnRuleColorStyleValue {}

/// Represents the style value for `column-rule-outset` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#column-rule-outset).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#column-rule-outset
#[syntax(" <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "50%",
	applies_to = "grid containers, flex containers, multicol containers, and masonry containers",
	inherited = "no",
	percentages = "refer to the crossing gap width",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.column-rule-outset"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct ColumnRuleOutsetStyleValue;

/// Represents the style value for `column-rule-style` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#column-rule-style).
///
/// Multi-column layout flows an element's content across one or more columns in a single row, without affecting the display property of its children.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style-list> | <auto-line-style-list>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#column-rule-style
#[syntax(" <line-style-list> | <auto-line-style-list> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "grid containers, flex containers, multicol containers, and masonry containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.column-rule-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum ColumnRuleStyleStyleValue {}

/// Represents the style value for `column-rule-width` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#column-rule-width).
///
/// Multi-column layout flows an element's content across one or more columns in a single row, without affecting the display property of its children.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width-list> | <auto-line-width-list>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#column-rule-width
#[syntax(" <line-width-list> | <auto-line-width-list> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "medium",
	applies_to = "grid containers, flex containers, multicol containers, and masonry containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "repeatable list, see §\u{202f}3.4.1 interpolation behavior."
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.column-rule-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum ColumnRuleWidthStyleValue<'a> {}

/// Represents the style value for `row-rule` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#row-rule).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <gap-rule-list> | <gap-auto-rule-list>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#row-rule
#[syntax(" <gap-rule-list> | <gap-auto-rule-list> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "see individual properties",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.row-rule"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum RowRuleStyleValue {}

/// Represents the style value for `row-rule-break` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#row-rule-break).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | spanning-item | intersection
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#row-rule-break
#[syntax(" none | spanning-item | intersection ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "spanning-item",
	applies_to = "grid containers, flex containers, multicol containers, and masonry containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.row-rule-break"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum RowRuleBreakStyleValue {}

/// Represents the style value for `row-rule-color` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#row-rule-color).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-color-list> | <auto-line-color-list>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#row-rule-color
#[syntax(" <line-color-list> | <auto-line-color-list> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "currentcolor",
	applies_to = "grid containers, flex containers, multicol containers, and masonry containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "repeatable list, see §\u{202f}3.4.1 interpolation behavior."
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.row-rule-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum RowRuleColorStyleValue {}

/// Represents the style value for `row-rule-outset` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#row-rule-outset).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#row-rule-outset
#[syntax(" <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "50%",
	applies_to = "grid containers, flex containers, multicol containers, and masonry containers",
	inherited = "no",
	percentages = "refer to the crossing gap width",
	canonical_order = "per grammar",
	animation_type = "by computed value type"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.row-rule-outset"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct RowRuleOutsetStyleValue;

/// Represents the style value for `row-rule-style` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#row-rule-style).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style-list> | <auto-line-style-list>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#row-rule-style
#[syntax(" <line-style-list> | <auto-line-style-list> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "grid containers, flex containers, multicol containers, and masonry containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.row-rule-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum RowRuleStyleStyleValue {}

/// Represents the style value for `row-rule-width` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#row-rule-width).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width-list> | <auto-line-width-list>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#row-rule-width
#[syntax(" <line-width-list> | <auto-line-width-list> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "medium",
	applies_to = "grid containers, flex containers, multicol containers, and masonry containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "repeatable list, see §\u{202f}3.4.1 interpolation behavior."
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.row-rule-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum RowRuleWidthStyleValue<'a> {}

/// Represents the style value for `rule` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'column-rule'>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#rule
#[syntax(" <'column-rule'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "Same as column-rule and row-rule",
	inherited = "no",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.rule"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct RuleStyleValue;

/// Represents the style value for `rule-break` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule-break).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'column-rule-break'>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#rule-break
#[syntax(" <'column-rule-break'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "Same as column-rule-break and row-rule-break",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.rule-break"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct RuleBreakStyleValue;

/// Represents the style value for `rule-color` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule-color).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'column-rule-color'>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#rule-color
#[syntax(" <'column-rule-color'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "Same as column-rule-color and row-rule-color",
	inherited = "no",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.rule-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct RuleColorStyleValue;

/// Represents the style value for `rule-outset` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule-outset).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'column-rule-outset'>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#rule-outset
#[syntax(" <'column-rule-outset'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "Same as column-rule-outset and row-rule-outset",
	inherited = "see individual properties",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.rule-outset"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct RuleOutsetStyleValue;

/// Represents the style value for `rule-overlap` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule-overlap).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// row-over-column | column-over-row
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#rule-overlap
#[syntax(" row-over-column | column-over-row ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "row-over-column",
	applies_to = "grid containers, flex containers, and masonry containers",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.rule-overlap"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum RuleOverlapStyleValue {}

/// Represents the style value for `rule-style` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule-style).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'column-rule-style'>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#rule-style
#[syntax(" <'column-rule-style'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "Same as column-rule-style and row-rule-style",
	inherited = "no",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.rule-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct RuleStyleStyleValue;

/// Represents the style value for `rule-width` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule-width).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'column-rule-width'>
/// ```
///
/// https://drafts.csswg.org/css-gaps-1/#rule-width
#[syntax(" <'column-rule-width'> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "see individual properties",
	applies_to = "Same as column-rule-width and row-rule-width",
	inherited = "no",
	percentages = "see individual properties",
	canonical_order = "per grammar",
	animation_type = "see individual properties"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.rule-width"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct RuleWidthStyleValue<'a>;
