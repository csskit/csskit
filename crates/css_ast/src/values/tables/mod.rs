#![allow(warnings)]
//! CSS Table Module Level 3
//! https://drafts.csswg.org/css-tables-3/

mod impls;
use impls::*;

/// Represents the style value for `table-layout` as defined in [css-tables-3](https://drafts.csswg.org/css-tables-3/#table-layout).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | fixed
/// ```
///
// https://drafts.csswg.org/css-tables-3/#table-layout
#[syntax(" auto | fixed ")]
#[initial("auto")]
#[applies_to("table grid boxes")]
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
pub enum TableLayoutStyleValue {}

/// Represents the style value for `border-collapse` as defined in [css-tables-3](https://drafts.csswg.org/css-tables-3/#border-collapse).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// separate | collapse
/// ```
///
// https://drafts.csswg.org/css-tables-3/#border-collapse
#[syntax(" separate | collapse ")]
#[initial("separate")]
#[applies_to("table grid boxes")]
#[inherited("yes")]
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
pub enum BorderCollapseStyleValue {}

/// Represents the style value for `border-spacing` as defined in [css-tables-3](https://drafts.csswg.org/css-tables-3/#border-spacing).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>{1,2}
/// ```
///
// https://drafts.csswg.org/css-tables-3/#border-spacing
#[syntax(" <length>{1,2} ")]
#[initial("0px 0px")]
#[applies_to("table grid boxes when border-collapse is separate")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderSpacingStyleValue;

/// Represents the style value for `caption-side` as defined in [css-tables-3](https://drafts.csswg.org/css-tables-3/#caption-side).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// top | bottom
/// ```
///
// https://drafts.csswg.org/css-tables-3/#caption-side
#[syntax(" top | bottom ")]
#[initial("top")]
#[applies_to("table-caption boxes")]
#[inherited("yes")]
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
pub enum CaptionSideStyleValue {}

/// Represents the style value for `empty-cells` as defined in [css-tables-3](https://drafts.csswg.org/css-tables-3/#empty-cells).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// show | hide
/// ```
///
// https://drafts.csswg.org/css-tables-3/#empty-cells
#[syntax(" show | hide ")]
#[initial("show")]
#[applies_to("table-cell boxes")]
#[inherited("yes")]
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
pub enum EmptyCellsStyleValue {}
