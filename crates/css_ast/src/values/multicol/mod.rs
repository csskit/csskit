mod impls;
use impls::*;

/*
 * https://drafts.csswg.org/css-multicol-2/
 * CSS Multi-column Layout Module Level 2
 */

// https://drafts.csswg.org/css-multicol-2/#column-width
#[value(" auto | <length [0,∞]> ")]
#[initial("auto")]
#[applies_to("block containers except table wrapper boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ColumnWidthStyleValue {}

// https://drafts.csswg.org/css-multicol-2/#column-count
#[value(" auto | <integer [1,∞]> ")]
#[initial("auto")]
#[applies_to("block containers except table wrapper boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub enum ColumnCountStyleValue {}

// // https://drafts.csswg.org/css-multicol-2/#columns
// #[value(" <'column-width'> || <'column-count'> [ / <'column-height'> ]? ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct ColumnsStyleValue;

// https://drafts.csswg.org/css-multicol-2/#column-span
#[value(" none | <integer [1,∞]> | all | auto ")]
#[initial("none")]
#[applies_to("in-flow block-level elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ColumnSpanStyleValue {}

// https://drafts.csswg.org/css-multicol-2/#column-fill
#[value(" auto | balance | balance-all ")]
#[initial("balance")]
#[applies_to("multicol containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ColumnFillStyleValue {}

// https://drafts.csswg.org/css-multicol-2/#column-height
#[value(" auto | <length [0,∞]> ")]
#[initial("auto")]
#[applies_to("block containers except table wrapper boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum ColumnHeightStyleValue {}

// https://drafts.csswg.org/css-multicol-2/#column-wrap
#[value(" auto | nowrap | wrap ")]
#[initial("auto")]
#[applies_to("multicol containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ColumnWrapStyleValue {}
