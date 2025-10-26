#![allow(warnings)]
//! https://drafts.csswg.org/css-anchor-position-2/

mod impls;
use super::prelude::*;
use impls::*;
/// Represents the style value for `anchor-name` as defined in [css-anchor-position-2](https://drafts.csswg.org/css-anchor-position-2/#anchor-name).
///
/// Anchor positioning places an element based on the position of another element. For example, you can place a tooltip next to the content it references.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <dashed-ident>#
/// ```
///
/// https://drafts.csswg.org/css-anchor-position-2/#anchor-name
#[syntax(" none | <dashed-ident># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements that generate a principal box",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.anchor-name"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct AnchorNameStyleValue<'a>;

/// Represents the style value for `anchor-scope` as defined in [css-anchor-position-2](https://drafts.csswg.org/css-anchor-position-2/#anchor-scope).
///
/// Anchor positioning places an element based on the position of another element. For example, you can place a tooltip next to the content it references.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | all | <dashed-ident>#
/// ```
///
/// https://drafts.csswg.org/css-anchor-position-2/#anchor-scope
#[syntax(" none | all | <dashed-ident># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "all elements",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.anchor-scope"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum AnchorScopeStyleValue<'a> {}

/// Represents the style value for `position-anchor` as defined in [css-anchor-position-2](https://drafts.csswg.org/css-anchor-position-2/#position-anchor).
///
/// Anchor positioning places an element based on the position of another element. For example, you can place a tooltip next to the content it references.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <anchor-name>
/// ```
///
/// https://drafts.csswg.org/css-anchor-position-2/#position-anchor
#[syntax(" auto | <anchor-name> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "auto",
	applies_to = "absolutely positioned boxes",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.position-anchor"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct PositionAnchorStyleValue;

/// Represents the style value for `position-area` as defined in [css-anchor-position-2](https://drafts.csswg.org/css-anchor-position-2/#position-area).
///
/// Anchor positioning places an element based on the position of another element. For example, you can place a tooltip next to the content it references.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <position-area>
/// ```
///
/// https://drafts.csswg.org/css-anchor-position-2/#position-area
#[syntax(" none | <position-area> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "none",
	applies_to = "positioned boxes with a default anchor box",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "tbd"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.position-area"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct PositionAreaStyleValue;

// /// Represents the style value for `position-try` as defined in [css-anchor-position-2](https://drafts.csswg.org/css-anchor-position-2/#position-try).
// ///
// /// Anchor positioning places an element based on the position of another element. For example, you can place a tooltip next to the content it references.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'position-try-order'>? <'position-try-fallbacks'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-anchor-position-2/#position-try
// #[syntax(" <'position-try-order'>? <'position-try-fallbacks'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     StyleValue,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash
// )]
// #[style_value(
//     initial = "see individual properties",
//     applies_to = "see individual properties",
//     inherited = "see individual properties",
//     percentages = "see individual properties",
//     canonical_order = "per grammar",
//     animation_type = "see individual properties",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.position-try")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct PositionTryStyleValue;

// /// Represents the style value for `position-try-fallbacks` as defined in [css-anchor-position-2](https://drafts.csswg.org/css-anchor-position-2/#position-try-fallbacks).
// ///
// /// Anchor positioning places an element based on the position of another element. For example, you can place a tooltip next to the content it references.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ [<dashed-ident> || <try-tactic>] | <position-area> ]#
// /// ```
// ///
// /// https://drafts.csswg.org/css-anchor-position-2/#position-try-fallbacks
// #[syntax(" none | [ [<dashed-ident> || <try-tactic>] | <position-area> ]# ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     StyleValue,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash
// )]
// #[style_value(
//     initial = "none",
//     applies_to = "absolutely positioned boxes",
//     inherited = "no",
//     percentages = "n/a",
//     canonical_order = "per grammar",
//     animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.position-try-fallbacks")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct PositionTryFallbacksStyleValue<'a>;

/// Represents the style value for `position-try-order` as defined in [css-anchor-position-2](https://drafts.csswg.org/css-anchor-position-2/#position-try-order).
///
/// Anchor positioning places an element based on the position of another element. For example, you can place a tooltip next to the content it references.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <try-size>
/// ```
///
/// https://drafts.csswg.org/css-anchor-position-2/#position-try-order
#[syntax(" normal | <try-size> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[style_value(
	initial = "normal",
	applies_to = "absolutely positioned boxes",
	inherited = "no",
	percentages = "n/a",
	canonical_order = "per grammar",
	animation_type = "discrete"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.position-try-order"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum PositionTryOrderStyleValue {}

// /// Represents the style value for `position-visibility` as defined in [css-anchor-position-2](https://drafts.csswg.org/css-anchor-position-2/#position-visibility).
// ///
// /// Anchor positioning places an element based on the position of another element. For example, you can place a tooltip next to the content it references.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// always | [ anchors-valid || anchors-visible || no-overflow ]
// /// ```
// ///
// /// https://drafts.csswg.org/css-anchor-position-2/#position-visibility
// #[syntax(" always | [ anchors-valid || anchors-visible || no-overflow ] ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     StyleValue,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash
// )]
// #[style_value(
//     initial = "anchors-visible",
//     applies_to = "absolutely positioned boxes",
//     inherited = "no",
//     percentages = "n/a",
//     canonical_order = "per grammar",
//     animation_type = "discrete",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.position-visibility")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum PositionVisibilityStyleValue {}
