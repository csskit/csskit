mod impls;
use impls::*;

/*
 * https://drafts.csswg.org/css-gaps-1/
 * CSS Gap Decorations Module Level 1
 */

// https://drafts.csswg.org/css-gaps-1/#column-rule-break
#[value(" none | spanning-item | intersection ")]
#[initial("spanning-item")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ColumnRuleBreakStyleValue {}

// https://drafts.csswg.org/css-gaps-1/#row-rule-break
#[value(" none | spanning-item | intersection ")]
#[initial("spanning-item")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum RowRuleBreakStyleValue {}

// https://drafts.csswg.org/css-gaps-1/#rule-break
#[value(" <'column-rule-break'> ")]
#[initial("see individual properties")]
#[applies_to("Same as column-rule-break and row-rule-break")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct RuleBreakStyleValue;

// https://drafts.csswg.org/css-gaps-1/#column-rule-outset
#[value(" <length-percentage> ")]
#[initial("50%")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("refer to the crossing gap width")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct ColumnRuleOutsetStyleValue;

// https://drafts.csswg.org/css-gaps-1/#row-rule-outset
#[value(" <length-percentage> ")]
#[initial("50%")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("refer to the crossing gap width")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct RowRuleOutsetStyleValue;

// https://drafts.csswg.org/css-gaps-1/#rule-outset
#[value(" <'column-rule-outset'> ")]
#[initial("see individual properties")]
#[applies_to("Same as column-rule-outset and row-rule-outset")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct RuleOutsetStyleValue;

// https://drafts.csswg.org/css-gaps-1/#rule-paint-order
#[value(" row-over-column | column-over-row ")]
#[initial("row-over-column")]
#[applies_to("grid containers, flex containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum RulePaintOrderStyleValue {}

// https://drafts.csswg.org/css-gaps-1/#column-rule-color
#[value(" <line-color-list> | <auto-line-color-list> ")]
#[initial("currentcolor")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ColumnRuleColorStyleValue {}

// https://drafts.csswg.org/css-gaps-1/#row-rule-color
#[value(" <line-color-list> | <auto-line-color-list> ")]
#[initial("currentcolor")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum RowRuleColorStyleValue {}

// https://drafts.csswg.org/css-gaps-1/#column-rule-style
#[value(" <line-style-list> | <auto-line-style-list> ")]
#[initial("none")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ColumnRuleStyleStyleValue {}

// https://drafts.csswg.org/css-gaps-1/#row-rule-style
#[value(" <line-style-list> | <auto-line-style-list> ")]
#[initial("none")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum RowRuleStyleStyleValue {}

// https://drafts.csswg.org/css-gaps-1/#column-rule-width
#[value(" <line-width-list> | <auto-line-width-list> ")]
#[initial("medium")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ColumnRuleWidthStyleValue {}

// https://drafts.csswg.org/css-gaps-1/#row-rule-width
#[value(" <line-width-list> | <auto-line-width-list> ")]
#[initial("medium")]
#[applies_to("grid containers, flex containers, multicol containers, and masonry containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum RowRuleWidthStyleValue {}

// https://drafts.csswg.org/css-gaps-1/#column-rule
#[value(" <gap-rule-list> | <gap-auto-rule-list> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub enum ColumnRuleStyleValue {}

// https://drafts.csswg.org/css-gaps-1/#row-rule
#[value(" <gap-rule-list> | <gap-auto-rule-list> ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub enum RowRuleStyleValue {}

// https://drafts.csswg.org/css-gaps-1/#rule-color
#[value(" <'column-rule-color'> ")]
#[initial("see individual properties")]
#[applies_to("Same as column-rule-color and row-rule-color")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct RuleColorStyleValue;

// https://drafts.csswg.org/css-gaps-1/#rule-style
#[value(" <'column-rule-style'> ")]
#[initial("see individual properties")]
#[applies_to("Same as column-rule-style and row-rule-style")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct RuleStyleStyleValue;

// https://drafts.csswg.org/css-gaps-1/#rule-width
#[value(" <'column-rule-width'> ")]
#[initial("see individual properties")]
#[applies_to("Same as column-rule-width and row-rule-width")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct RuleWidthStyleValue;

// https://drafts.csswg.org/css-gaps-1/#rule
#[value(" <'column-rule'> ")]
#[initial("see individual properties")]
#[applies_to("Same as column-rule and row-rule")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct RuleStyleValue;
