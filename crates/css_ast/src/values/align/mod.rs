#![allow(warnings)]
//! CSS Box Alignment Module Level 3
//! https://drafts.csswg.org/css-align-3/

mod impls;
use impls::*;

/// Represents the style value for `align-content` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#align-content).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <baseline-position> | <content-distribution> | <overflow-position>? <content-position>
/// ```
///
// https://drafts.csswg.org/css-align-3/#align-content
#[value(" normal | <baseline-position> | <content-distribution> | <overflow-position>? <content-position> ")]
#[initial("normal")]
#[applies_to("block containers, multicol containers, flex containers, and grid containers")]
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
pub enum AlignContentStyleValue {}

// /// Represents the style value for `justify-content` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#justify-content).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | <content-distribution> | <overflow-position>? [ <content-position> | left | right ]
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#justify-content
// #[value(" normal | <content-distribution> | <overflow-position>? [ <content-position> | left | right ] ")]
// #[initial("normal")]
// #[applies_to("multicol containers, flex containers, and grid containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum JustifyContentStyleValue {}

// /// Represents the style value for `place-content` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#place-content).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'align-content'> <'justify-content'>?
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#place-content
// #[value(" <'align-content'> <'justify-content'>? ")]
// #[initial("normal")]
// #[applies_to("see individual properties")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct PlaceContentStyleValue;

// /// Represents the style value for `justify-self` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#justify-self).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | normal | stretch | <baseline-position> | <overflow-position>? [ <self-position> | left | right ]
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#justify-self
// #[value(" auto | normal | stretch | <baseline-position> | <overflow-position>? [ <self-position> | left | right ] ")]
// #[initial("auto")]
// #[applies_to("block-level boxes, absolutely-positioned boxes, and grid items")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum JustifySelfStyleValue {}

/// Represents the style value for `align-self` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#align-self).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | normal | stretch | <baseline-position> | <overflow-position>? <self-position>
/// ```
///
// https://drafts.csswg.org/css-align-3/#align-self
#[value(" auto | normal | stretch | <baseline-position> | <overflow-position>? <self-position> ")]
#[initial("auto")]
#[applies_to("flex items, grid items, and absolutely-positioned boxes")]
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
pub enum AlignSelfStyleValue {}

// /// Represents the style value for `place-self` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#place-self).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'align-self'> <'justify-self'>?
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#place-self
// #[value(" <'align-self'> <'justify-self'>? ")]
// #[initial("auto")]
// #[applies_to("see individual properties")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct PlaceSelfStyleValue;

// /// Represents the style value for `justify-items` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#justify-items).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | stretch | <baseline-position> | <overflow-position>? [ <self-position> | left | right ] | legacy | legacy && [ left | right | center ]
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#justify-items
// #[value(
// 	" normal | stretch | <baseline-position> | <overflow-position>? [ <self-position> | left | right ] | legacy | legacy && [ left | right | center ] "
// )]
// #[initial("legacy")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum JustifyItemsStyleValue {}

// /// Represents the style value for `align-items` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#align-items).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | stretch | <baseline-position> | [ <overflow-position>? <self-position> ]
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#align-items
// #[value(" normal | stretch | <baseline-position> | [ <overflow-position>? <self-position> ] ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum AlignItemsStyleValue {}

// /// Represents the style value for `place-items` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#place-items).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'align-items'> <'justify-items'>?
// /// ```
// ///
// // https://drafts.csswg.org/css-align-3/#place-items
// #[value(" <'align-items'> <'justify-items'>? ")]
// #[initial("see individual properties")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct PlaceItemsStyleValue;

/// Represents the style value for `row-gap` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#row-gap).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-align-3/#row-gap
#[value(" normal | <length-percentage [0,∞]> ")]
#[initial("normal")]
#[applies_to("multi-column containers, flex containers, grid containers")]
#[inherited("no")]
#[percentages("see § 8.3 percentages in gap properties")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum RowGapStyleValue {}

/// Represents the style value for `column-gap` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#column-gap).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-align-3/#column-gap
#[value(" normal | <length-percentage [0,∞]> ")]
#[initial("normal")]
#[applies_to("multi-column containers, flex containers, grid containers")]
#[inherited("no")]
#[percentages("see § 8.3 percentages in gap properties")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum ColumnGapStyleValue {}

/// Represents the style value for `gap` as defined in [css-align-3](https://drafts.csswg.org/css-align-3/#gap).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'row-gap'> <'column-gap'>?
/// ```
///
// https://drafts.csswg.org/css-align-3/#gap
#[value(" <'row-gap'> <'column-gap'>? ")]
#[initial("see individual properties")]
#[applies_to("multi-column containers, flex containers, grid containers")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the content area")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct GapStyleValue;
