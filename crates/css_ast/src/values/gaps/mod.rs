#![allow(warnings)]
//! CSS Gap Decorations Module Level 1
//! https://drafts.csswg.org/css-gaps-1/

mod impls;
use impls::*;

/// Represents the style value for `column-rule-break` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#column-rule-break).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | spanning-item | intersection
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#column-rule-break
#[syntax(" none | spanning-item | intersection ")]
#[initial("spanning-item")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum ColumnRuleBreakStyleValue {}

/// Represents the style value for `row-rule-break` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#row-rule-break).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | spanning-item | intersection
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#row-rule-break
#[syntax(" none | spanning-item | intersection ")]
#[initial("spanning-item")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum RowRuleBreakStyleValue {}

/// Represents the style value for `rule-break` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule-break).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'column-rule-break'>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#rule-break
#[syntax(" <'column-rule-break'> ")]
#[initial("see individual properties")]
#[applies_to("Same as column-rule-break and row-rule-break")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct RuleBreakStyleValue;

/// Represents the style value for `column-rule-outset` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#column-rule-outset).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#column-rule-outset
#[syntax(" <length-percentage> ")]
#[initial("50%")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("refer to the crossing gap width")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct ColumnRuleOutsetStyleValue;

/// Represents the style value for `row-rule-outset` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#row-rule-outset).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#row-rule-outset
#[syntax(" <length-percentage> ")]
#[initial("50%")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("refer to the crossing gap width")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct RowRuleOutsetStyleValue;

/// Represents the style value for `rule-outset` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule-outset).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'column-rule-outset'>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#rule-outset
#[syntax(" <'column-rule-outset'> ")]
#[initial("see individual properties")]
#[applies_to("Same as column-rule-outset and row-rule-outset")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct RuleOutsetStyleValue;

/// Represents the style value for `rule-overlap` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule-overlap).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// row-over-column | column-over-row
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#rule-overlap
#[syntax(" row-over-column | column-over-row ")]
#[initial("row-over-column")]
#[applies_to("grid containers, flex containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum RuleOverlapStyleValue {}

/// Represents the style value for `column-rule-color` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#column-rule-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-color-list> | <auto-line-color-list>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#column-rule-color
#[syntax(" <line-color-list> | <auto-line-color-list> ")]
#[initial("currentcolor")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum ColumnRuleColorStyleValue {}

/// Represents the style value for `row-rule-color` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#row-rule-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-color-list> | <auto-line-color-list>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#row-rule-color
#[syntax(" <line-color-list> | <auto-line-color-list> ")]
#[initial("currentcolor")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum RowRuleColorStyleValue {}

/// Represents the style value for `column-rule-style` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#column-rule-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style-list> | <auto-line-style-list>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#column-rule-style
#[syntax(" <line-style-list> | <auto-line-style-list> ")]
#[initial("none")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum ColumnRuleStyleStyleValue {}

/// Represents the style value for `row-rule-style` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#row-rule-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style-list> | <auto-line-style-list>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#row-rule-style
#[syntax(" <line-style-list> | <auto-line-style-list> ")]
#[initial("none")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum RowRuleStyleStyleValue {}

/// Represents the style value for `column-rule-width` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#column-rule-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width-list> | <auto-line-width-list>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#column-rule-width
#[syntax(" <line-width-list> | <auto-line-width-list> ")]
#[initial("medium")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum ColumnRuleWidthStyleValue {}

/// Represents the style value for `row-rule-width` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#row-rule-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width-list> | <auto-line-width-list>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#row-rule-width
#[syntax(" <line-width-list> | <auto-line-width-list> ")]
#[initial("medium")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum RowRuleWidthStyleValue {}

/// Represents the style value for `column-rule` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#column-rule).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <gap-rule-list> | <gap-auto-rule-list>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#column-rule
#[syntax(" <gap-rule-list> | <gap-auto-rule-list> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum ColumnRuleStyleValue {}

/// Represents the style value for `row-rule` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#row-rule).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <gap-rule-list> | <gap-auto-rule-list>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#row-rule
#[syntax(" <gap-rule-list> | <gap-auto-rule-list> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum RowRuleStyleValue {}

/// Represents the style value for `rule-color` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'column-rule-color'>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#rule-color
#[syntax(" <'column-rule-color'> ")]
#[initial("see individual properties")]
#[applies_to("Same as column-rule-color and row-rule-color")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct RuleColorStyleValue;

/// Represents the style value for `rule-style` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'column-rule-style'>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#rule-style
#[syntax(" <'column-rule-style'> ")]
#[initial("see individual properties")]
#[applies_to("Same as column-rule-style and row-rule-style")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct RuleStyleStyleValue;

/// Represents the style value for `rule-width` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'column-rule-width'>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#rule-width
#[syntax(" <'column-rule-width'> ")]
#[initial("see individual properties")]
#[applies_to("Same as column-rule-width and row-rule-width")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct RuleWidthStyleValue;

/// Represents the style value for `rule` as defined in [css-gaps-1](https://drafts.csswg.org/css-gaps-1/#rule).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'column-rule'>
/// ```
///
// https://drafts.csswg.org/css-gaps-1/#rule
#[syntax(" <'column-rule'> ")]
#[initial("see individual properties")]
#[applies_to("Same as column-rule and row-rule")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct RuleStyleValue;
