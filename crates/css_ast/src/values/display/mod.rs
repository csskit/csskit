#![allow(warnings)]
//! CSS Display Module Level 4
//! https://drafts.csswg.org/css-display-4/

mod impls;
use impls::*;

// /// Represents the style value for `display` as defined in [css-display-4](https://drafts.csswg.org/css-display-4/#display).
// ///
// /// The display CSS property sets the display behavior of an element's box within its layout and sets the layout behavior for its child elements.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <display-outside> || <display-inside> ] | <display-listitem> | <display-internal> | <display-box> | <display-legacy>
// /// ```
// ///
// // https://drafts.csswg.org/css-display-4/#display
// #[syntax(
// 	" [ <display-outside> || <display-inside> ] | <display-listitem> | <display-internal> | <display-box> | <display-legacy> "
// )]
// #[initial("inline")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see § 2.9 animating and interpolating display")]
// #[popularity(92.052)]
// #[caniuse("https://caniuse.com/inline-block")]
// #[baseline(widely)]
// #[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum DisplayStyleValue {}

/// Represents the style value for `order` as defined in [css-display-4](https://drafts.csswg.org/css-display-4/#order).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <integer>
/// ```
///
// https://drafts.csswg.org/css-display-4/#order
#[syntax(" <integer> ")]
#[initial("0")]
#[applies_to("flex items and grid items")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(92.052)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct OrderStyleValue;

/// Represents the style value for `visibility` as defined in [css-display-4](https://drafts.csswg.org/css-display-4/#visibility).
///
/// The visibility CSS property sets whether an element is shown. Invisible elements still affect the document layout.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// visible | hidden | force-hidden | collapse
/// ```
///
// https://drafts.csswg.org/css-display-4/#visibility
#[syntax(" visible | hidden | force-hidden | collapse ")]
#[initial("visible")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(92.052)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1.3,safari_ios:1)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum VisibilityStyleValue {}

/// Represents the style value for `reading-flow` as defined in [css-display-4](https://drafts.csswg.org/css-display-4/#reading-flow).
///
/// The reading-flow CSS property sets the order in which flex or grid elements are rendered to speech or reached via focus navigation. The reading-order property overrides this order.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | source-order | flex-visual | flex-flow | grid-rows | grid-columns | grid-order
/// ```
///
// https://drafts.csswg.org/css-display-4/#reading-flow
#[syntax(" normal | source-order | flex-visual | flex-flow | grid-rows | grid-columns | grid-order ")]
#[initial("normal")]
#[applies_to("block, flex and grid containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(92.052)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:137,chrome_android:137,edge:137)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum ReadingFlowStyleValue {}

/// Represents the style value for `reading-order` as defined in [css-display-4](https://drafts.csswg.org/css-display-4/#reading-order).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <integer>
/// ```
///
// https://drafts.csswg.org/css-display-4/#reading-order
#[syntax(" <integer> ")]
#[initial("0")]
#[applies_to("Direct block-level, grid item, or flex item children of a reading flow container.")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(92.052)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct ReadingOrderStyleValue;
