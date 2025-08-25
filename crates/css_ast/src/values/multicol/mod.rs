#![allow(warnings)]
//! CSS Multi-column Layout Module Level 2
//! https://drafts.csswg.org/css-multicol-2/

mod impls;
use impls::*;

/// Represents the style value for `column-width` as defined in [css-multicol-2](https://drafts.csswg.org/css-multicol-2/#column-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-multicol-2/#column-width
#[value(" auto | <length [0,∞]> ")]
#[initial("auto")]
#[applies_to("block containers except table wrapper boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ColumnWidthStyleValue;

/// Represents the style value for `column-count` as defined in [css-multicol-2](https://drafts.csswg.org/css-multicol-2/#column-count).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <integer [1,∞]>
/// ```
///
// https://drafts.csswg.org/css-multicol-2/#column-count
#[value(" auto | <integer [1,∞]> ")]
#[initial("auto")]
#[applies_to("block containers except table wrapper boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum ColumnCountStyleValue {}

// /// Represents the style value for `columns` as defined in [css-multicol-2](https://drafts.csswg.org/css-multicol-2/#columns).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <'column-width'> || <'column-count'> ] [ / <'column-height'> ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-multicol-2/#columns
// #[value(" [ <'column-width'> || <'column-count'> ] [ / <'column-height'> ]? ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct ColumnsStyleValue;

/// Represents the style value for `column-span` as defined in [css-multicol-2](https://drafts.csswg.org/css-multicol-2/#column-span).
///
/// The column-span CSS property controls whether a child element extends across all columns of a multi-column parent.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <integer [1,∞]> | all | auto
/// ```
///
// https://drafts.csswg.org/css-multicol-2/#column-span
#[value(" none | <integer [1,∞]> | all | auto ")]
#[initial("none")]
#[applies_to("in-flow block-level elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:50,chrome_android:50,edge:12,firefox:71,firefox_android:79,safari:9,safari_ios:9)]
pub enum ColumnSpanStyleValue {}

/// Represents the style value for `column-fill` as defined in [css-multicol-2](https://drafts.csswg.org/css-multicol-2/#column-fill).
///
/// The column-fill CSS property sets the distribution of content across columns in a multi-column layout.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | balance | balance-all
/// ```
///
// https://drafts.csswg.org/css-multicol-2/#column-fill
#[value(" auto | balance | balance-all ")]
#[initial("balance")]
#[applies_to("multicol containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:50,chrome_android:50,edge:12,firefox:52,firefox_android:52,safari:9,safari_ios:9)]
pub enum ColumnFillStyleValue {}

/// Represents the style value for `column-height` as defined in [css-multicol-2](https://drafts.csswg.org/css-multicol-2/#column-height).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-multicol-2/#column-height
#[value(" auto | <length [0,∞]> ")]
#[initial("auto")]
#[applies_to("block containers except table wrapper boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ColumnHeightStyleValue;

/// Represents the style value for `column-wrap` as defined in [css-multicol-2](https://drafts.csswg.org/css-multicol-2/#column-wrap).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | nowrap | wrap
/// ```
///
// https://drafts.csswg.org/css-multicol-2/#column-wrap
#[value(" auto | nowrap | wrap ")]
#[initial("auto")]
#[applies_to("multicol containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum ColumnWrapStyleValue {}
