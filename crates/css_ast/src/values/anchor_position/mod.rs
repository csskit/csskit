#![allow(warnings)]
//! CSS Anchor Positioning
//! https://drafts.csswg.org/css-anchor-position-1/

mod impls;
use impls::*;

/// Represents the style value for `anchor-name` as defined in [css-anchor-position-1](https://drafts.csswg.org/css-anchor-position-1/#anchor-name).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <dashed-ident>#
/// ```
///
// https://drafts.csswg.org/css-anchor-position-1/#anchor-name
#[syntax(" none | <dashed-ident># ")]
#[initial("none")]
#[applies_to("all elements that generate a principal box")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct AnchorNameStyleValue<'a>;

/// Represents the style value for `anchor-scope` as defined in [css-anchor-position-1](https://drafts.csswg.org/css-anchor-position-1/#anchor-scope).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | all | <dashed-ident>#
/// ```
///
// https://drafts.csswg.org/css-anchor-position-1/#anchor-scope
#[syntax(" none | all | <dashed-ident># ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum AnchorScopeStyleValue<'a> {}

/// Represents the style value for `position-anchor` as defined in [css-anchor-position-1](https://drafts.csswg.org/css-anchor-position-1/#position-anchor).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <anchor-name>
/// ```
///
// https://drafts.csswg.org/css-anchor-position-1/#position-anchor
#[syntax(" auto | <anchor-name> ")]
#[initial("auto")]
#[applies_to("absolutely positioned boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct PositionAnchorStyleValue;

/// Represents the style value for `position-area` as defined in [css-anchor-position-1](https://drafts.csswg.org/css-anchor-position-1/#position-area).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <position-area>
/// ```
///
// https://drafts.csswg.org/css-anchor-position-1/#position-area
#[syntax(" none | <position-area> ")]
#[initial("none")]
#[applies_to("positioned boxes with a default anchor box")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("tbd")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct PositionAreaStyleValue;

// /// Represents the style value for `position-visibility` as defined in [css-anchor-position-1](https://drafts.csswg.org/css-anchor-position-1/#position-visibility).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// always | [ anchors-valid || anchors-visible || no-overflow ]
// /// ```
// ///
// // https://drafts.csswg.org/css-anchor-position-1/#position-visibility
// #[syntax(" always | [ anchors-valid || anchors-visible || no-overflow ] ")]
// #[initial("anchors-visible")]
// #[applies_to("absolutely positioned boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum PositionVisibilityStyleValue {}

// /// Represents the style value for `position-try-fallbacks` as defined in [css-anchor-position-1](https://drafts.csswg.org/css-anchor-position-1/#position-try-fallbacks).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ [<dashed-ident> || <try-tactic>] | <'position-area'> ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-anchor-position-1/#position-try-fallbacks
// #[syntax(" none | [ [<dashed-ident> || <try-tactic>] | <'position-area'> ]# ")]
// #[initial("none")]
// #[applies_to("absolutely positioned boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum PositionTryFallbacksStyleValue<'a> {}

/// Represents the style value for `position-try-order` as defined in [css-anchor-position-1](https://drafts.csswg.org/css-anchor-position-1/#position-try-order).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <try-size>
/// ```
///
// https://drafts.csswg.org/css-anchor-position-1/#position-try-order
#[syntax(" normal | <try-size> ")]
#[initial("normal")]
#[applies_to("absolutely positioned boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum PositionTryOrderStyleValue {}

// /// Represents the style value for `position-try` as defined in [css-anchor-position-1](https://drafts.csswg.org/css-anchor-position-1/#position-try).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'position-try-order'>? <'position-try-fallbacks'>
// /// ```
// ///
// // https://drafts.csswg.org/css-anchor-position-1/#position-try
// #[syntax(" <'position-try-order'>? <'position-try-fallbacks'> ")]
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
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct PositionTryStyleValue;
