#![allow(warnings)]
//! CSS Borders and Box Decorations Module Level 4
//! https://drafts.csswg.org/css-borders-4/

mod impls;
use impls::*;

/// Represents the style value for `border-top-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-top-color
#[syntax(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderTopColorStyleValue<'a> {}

/// Represents the style value for `border-right-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-right-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-right-color
#[syntax(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderRightColorStyleValue<'a> {}

/// Represents the style value for `border-bottom-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-bottom-color
#[syntax(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderBottomColorStyleValue<'a> {}

/// Represents the style value for `border-left-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-left-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-left-color
#[syntax(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderLeftColorStyleValue<'a> {}

/// Represents the style value for `border-block-start-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-start-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-start-color
#[syntax(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderBlockStartColorStyleValue<'a> {}

/// Represents the style value for `border-block-end-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-end-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-end-color
#[syntax(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderBlockEndColorStyleValue<'a> {}

/// Represents the style value for `border-inline-start-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-start-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-start-color
#[syntax(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderInlineStartColorStyleValue<'a> {}

/// Represents the style value for `border-inline-end-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-end-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-end-color
#[syntax(" <color> | <image-1D> ")]
#[initial("currentcolor")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderInlineEndColorStyleValue<'a> {}

// /// Represents the style value for `border-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-color).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <color> | <image-1D> ]{1,4}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-color
// #[syntax(" [ <color> | <image-1D> ]{1,4} ")]
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
// pub struct BorderColorStyleValue<'a>;

/// Represents the style value for `border-block-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-color'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-color
#[syntax(" <'border-top-color'>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderBlockColorStyleValue<'a>;

/// Represents the style value for `border-inline-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-color'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-color
#[syntax(" <'border-top-color'>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderInlineColorStyleValue<'a>;

/// Represents the style value for `border-top-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-top-style
#[syntax(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
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
pub struct BorderTopStyleStyleValue;

/// Represents the style value for `border-right-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-right-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-right-style
#[syntax(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
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
pub struct BorderRightStyleStyleValue;

/// Represents the style value for `border-bottom-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-bottom-style
#[syntax(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
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
pub struct BorderBottomStyleStyleValue;

/// Represents the style value for `border-left-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-left-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-left-style
#[syntax(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
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
pub struct BorderLeftStyleStyleValue;

/// Represents the style value for `border-block-start-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-start-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-start-style
#[syntax(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
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
pub struct BorderBlockStartStyleStyleValue;

/// Represents the style value for `border-block-end-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-end-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-end-style
#[syntax(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
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
pub struct BorderBlockEndStyleStyleValue;

/// Represents the style value for `border-inline-start-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-start-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-start-style
#[syntax(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
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
pub struct BorderInlineStartStyleStyleValue;

/// Represents the style value for `border-inline-end-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-end-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-style>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-end-style
#[syntax(" <line-style> ")]
#[initial("none")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
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
pub struct BorderInlineEndStyleStyleValue;

/// Represents the style value for `border-block-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-style'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-style
#[syntax(" <'border-top-style'>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderBlockStyleStyleValue;

/// Represents the style value for `border-inline-style` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-style'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-style
#[syntax(" <'border-top-style'>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderInlineStyleStyleValue;

/// Represents the style value for `border-top-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-top-width
#[syntax(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderTopWidthStyleValue;

/// Represents the style value for `border-right-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-right-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-right-width
#[syntax(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderRightWidthStyleValue;

/// Represents the style value for `border-bottom-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-bottom-width
#[syntax(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderBottomWidthStyleValue;

/// Represents the style value for `border-left-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-left-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-left-width
#[syntax(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderLeftWidthStyleValue;

/// Represents the style value for `border-block-start-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-start-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-start-width
#[syntax(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderBlockStartWidthStyleValue;

/// Represents the style value for `border-block-end-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-end-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-end-width
#[syntax(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderBlockEndWidthStyleValue;

/// Represents the style value for `border-inline-start-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-start-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-start-width
#[syntax(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderInlineStartWidthStyleValue;

/// Represents the style value for `border-inline-end-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-end-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-end-width
#[syntax(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderInlineEndWidthStyleValue;

/// Represents the style value for `border-block-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-width'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-width
#[syntax(" <'border-top-width'>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderBlockWidthStyleValue;

/// Represents the style value for `border-inline-width` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-top-width'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-width
#[syntax(" <'border-top-width'>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderInlineWidthStyleValue;

/// Represents the style value for `border-top` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-top
#[syntax(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderTopStyleValue;

/// Represents the style value for `border-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-right).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-right
#[syntax(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderRightStyleValue;

/// Represents the style value for `border-bottom` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-bottom
#[syntax(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderBottomStyleValue;

/// Represents the style value for `border-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-left).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-left
#[syntax(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderLeftStyleValue;

/// Represents the style value for `border-block-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-start
#[syntax(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderBlockStartStyleValue;

/// Represents the style value for `border-block-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block-end
#[syntax(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderBlockEndStyleValue;

/// Represents the style value for `border-inline-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-start
#[syntax(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderInlineStartStyleValue;

/// Represents the style value for `border-inline-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width> || <line-style> || <color>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline-end
#[syntax(" <line-width> || <line-style> || <color> ")]
#[initial("See individual properties")]
#[applies_to("all elements except ruby base containers and ruby annotation containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderInlineEndStyleValue;

/// Represents the style value for `border-block` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-block-start'>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-block
#[syntax(" <'border-block-start'> ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderBlockStyleValue;

/// Represents the style value for `border-inline` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'border-block-start'>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-inline
#[syntax(" <'border-block-start'> ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderInlineStyleValue;

/// Represents the style value for `border-top-left-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-left-radius).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-top-left-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderTopLeftRadiusStyleValue;

/// Represents the style value for `border-top-right-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-right-radius).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-top-right-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderTopRightRadiusStyleValue;

/// Represents the style value for `border-bottom-right-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-right-radius).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-bottom-right-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderBottomRightRadiusStyleValue;

/// Represents the style value for `border-bottom-left-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-left-radius).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-bottom-left-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderBottomLeftRadiusStyleValue;

/// Represents the style value for `border-start-start-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-start-start-radius).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-start-start-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderStartStartRadiusStyleValue;

/// Represents the style value for `border-start-end-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-start-end-radius).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-start-end-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderStartEndRadiusStyleValue;

/// Represents the style value for `border-end-start-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-end-start-radius).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-end-start-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderEndStartRadiusStyleValue;

/// Represents the style value for `border-end-end-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-end-end-radius).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-end-end-radius
#[syntax(" <length-percentage [0,∞]>{1,2} ")]
#[initial("0")]
#[applies_to("all elements (but see prose)")]
#[inherited("no")]
#[percentages("refer to corresponding dimension of the border box.")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BorderEndEndRadiusStyleValue;

// /// Represents the style value for `border-top-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-top-radius).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-top-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct BorderTopRadiusStyleValue;

// /// Represents the style value for `border-right-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-right-radius).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-right-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct BorderRightRadiusStyleValue;

// /// Represents the style value for `border-bottom-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-bottom-radius).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-bottom-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct BorderBottomRadiusStyleValue;

// /// Represents the style value for `border-left-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-left-radius).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-left-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct BorderLeftRadiusStyleValue;

// /// Represents the style value for `border-block-start-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-start-radius).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-block-start-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct BorderBlockStartRadiusStyleValue;

// /// Represents the style value for `border-block-end-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-block-end-radius).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-block-end-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct BorderBlockEndRadiusStyleValue;

// /// Represents the style value for `border-inline-start-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-start-radius).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-inline-start-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct BorderInlineStartRadiusStyleValue;

// /// Represents the style value for `border-inline-end-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-inline-end-radius).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-inline-end-radius
// #[syntax(" <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct BorderInlineEndRadiusStyleValue;

// /// Represents the style value for `border-radius` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-radius).
// ///
// /// The border-radius CSS property rounds the corners of the border drawn around an element.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-radius
// #[syntax(" <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]? ")]
// #[initial("0")]
// #[applies_to("all elements, except table element when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/border-radius")]
// #[baseline(widely)]
// #[versions(chrome:4,chrome_android:18,edge:12,firefox:4,firefox_android:4,safari:5,safari_ios:4.2)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct BorderRadiusStyleValue;

/// Represents the style value for `corner-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,4}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-shape
#[syntax(" <corner-shape-value>{1,4} ")]
#[initial("round")]
#[applies_to("all elements where border-radius can apply")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerShapeStyleValue;

/// Represents the style value for `corner-top-left-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-left-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-top-left-shape
#[syntax(" <corner-shape-value> ")]
#[initial("round")]
#[applies_to("all elements where border-radius can apply")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see superellipse interpolation")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerTopLeftShapeStyleValue;

/// Represents the style value for `corner-top-right-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-right-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-top-right-shape
#[syntax(" <corner-shape-value> ")]
#[initial("round")]
#[applies_to("all elements where border-radius can apply")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see superellipse interpolation")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerTopRightShapeStyleValue;

/// Represents the style value for `corner-bottom-right-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-right-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-bottom-right-shape
#[syntax(" <corner-shape-value> ")]
#[initial("round")]
#[applies_to("all elements where border-radius can apply")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see superellipse interpolation")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerBottomRightShapeStyleValue;

/// Represents the style value for `corner-bottom-left-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-left-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-bottom-left-shape
#[syntax(" <corner-shape-value> ")]
#[initial("round")]
#[applies_to("all elements where border-radius can apply")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see superellipse interpolation")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerBottomLeftShapeStyleValue;

/// Represents the style value for `corner-start-start-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-start-start-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-start-start-shape
#[syntax(" <corner-shape-value> ")]
#[initial("round")]
#[applies_to("all elements where border-radius can apply")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see superellipse interpolation")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerStartStartShapeStyleValue;

/// Represents the style value for `corner-start-end-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-start-end-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-start-end-shape
#[syntax(" <corner-shape-value> ")]
#[initial("round")]
#[applies_to("all elements where border-radius can apply")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see superellipse interpolation")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerStartEndShapeStyleValue;

/// Represents the style value for `corner-end-start-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-end-start-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-end-start-shape
#[syntax(" <corner-shape-value> ")]
#[initial("round")]
#[applies_to("all elements where border-radius can apply")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see superellipse interpolation")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerEndStartShapeStyleValue;

/// Represents the style value for `corner-end-end-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-end-end-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-end-end-shape
#[syntax(" <corner-shape-value> ")]
#[initial("round")]
#[applies_to("all elements where border-radius can apply")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see superellipse interpolation")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerEndEndShapeStyleValue;

/// Represents the style value for `corner-top-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-top-shape
#[syntax(" <corner-shape-value>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerTopShapeStyleValue;

/// Represents the style value for `corner-right-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-right-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-right-shape
#[syntax(" <corner-shape-value>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerRightShapeStyleValue;

/// Represents the style value for `corner-bottom-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-bottom-shape
#[syntax(" <corner-shape-value>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerBottomShapeStyleValue;

/// Represents the style value for `corner-left-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-left-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-left-shape
#[syntax(" <corner-shape-value>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerLeftShapeStyleValue;

/// Represents the style value for `corner-block-start-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-block-start-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-block-start-shape
#[syntax(" <corner-shape-value>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerBlockStartShapeStyleValue;

/// Represents the style value for `corner-block-end-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-block-end-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-block-end-shape
#[syntax(" <corner-shape-value>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerBlockEndShapeStyleValue;

/// Represents the style value for `corner-inline-start-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-inline-start-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-inline-start-shape
#[syntax(" <corner-shape-value>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerInlineStartShapeStyleValue;

/// Represents the style value for `corner-inline-end-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-inline-end-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <corner-shape-value>{1,2}
/// ```
///
// https://drafts.csswg.org/css-borders-4/#corner-inline-end-shape
#[syntax(" <corner-shape-value>{1,2} ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CornerInlineEndShapeStyleValue;

// /// Represents the style value for `corner-top-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-left).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-top-left
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerTopLeftStyleValue;

// /// Represents the style value for `corner-top-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top-right).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-top-right
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerTopRightStyleValue;

// /// Represents the style value for `corner-bottom-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-left).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-bottom-left
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerBottomLeftStyleValue;

// /// Represents the style value for `corner-bottom-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom-right).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-bottom-right
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerBottomRightStyleValue;

// /// Represents the style value for `corner-start-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-start-start).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-start-start
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerStartStartStyleValue;

// /// Represents the style value for `corner-start-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-start-end).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-start-end
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerStartEndStyleValue;

// /// Represents the style value for `corner-end-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-end-start).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-end-start
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerEndStartStyleValue;

// /// Represents the style value for `corner-end-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-end-end).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-end-end
// #[syntax(" <length-percentage [0,∞]>{1,2} || <corner-shape-value> ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerEndEndStyleValue;

// /// Represents the style value for `corner-top` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-top).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-top
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerTopStyleValue;

// /// Represents the style value for `corner-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-right).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-right
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerRightStyleValue;

// /// Represents the style value for `corner-bottom` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-bottom).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-bottom
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerBottomStyleValue;

// /// Represents the style value for `corner-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-left).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-left
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerLeftStyleValue;

// /// Represents the style value for `corner-block-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-block-start).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-block-start
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerBlockStartStyleValue;

// /// Represents the style value for `corner-block-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-block-end).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-block-end
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerBlockEndStyleValue;

// /// Represents the style value for `corner-inline-start` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-inline-start).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-inline-start
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerInlineStartStyleValue;

// /// Represents the style value for `corner-inline-end` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner-inline-end).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner-inline-end
// #[syntax(" [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2} ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerInlineEndStyleValue;

// /// Represents the style value for `corner` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#corner).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]? ] || <corner-shape-value>{1,4}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#corner
// #[syntax(" [ <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]? ] || <corner-shape-value>{1,4} ")]
// #[initial("0")]
// #[applies_to("all elements (but see prose)")]
// #[inherited("no")]
// #[percentages("refer to corresponding dimension of the border box.")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct CornerStyleValue;

// /// Represents the style value for `border-limit` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-limit).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// all | [ sides | corners ] <length-percentage [0,∞]>? | [ top | right | bottom | left ] <length-percentage [0,∞]>
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-limit
// #[syntax(
// 	" all | [ sides | corners ] <length-percentage [0,∞]>? | [ top | right | bottom | left ] <length-percentage [0,∞]> "
// )]
// #[initial("all")]
// #[applies_to("all elements, except table element when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("relative to border-box")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum BorderLimitStyleValue {}

/// Represents the style value for `border-clip` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-clip
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("refer to length of border-edge side")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderClipStyleValue<'a> {}

/// Represents the style value for `border-clip-top` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip-top).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-clip-top
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("refer to length of border-edge side")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderClipTopStyleValue<'a> {}

/// Represents the style value for `border-clip-right` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip-right).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-clip-right
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("refer to length of border-edge side")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderClipRightStyleValue<'a> {}

/// Represents the style value for `border-clip-bottom` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip-bottom).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-clip-bottom
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("refer to length of border-edge side")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderClipBottomStyleValue<'a> {}

/// Represents the style value for `border-clip-left` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-clip-left).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | [ <length-percentage [0,∞]> | <flex> ]+
/// ```
///
// https://drafts.csswg.org/css-borders-4/#border-clip-left
#[syntax(" normal | [ <length-percentage [0,∞]> | <flex> ]+ ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("refer to length of border-edge side")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BorderClipLeftStyleValue<'a> {}

/// Represents the style value for `box-shadow-color` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color>#
/// ```
///
// https://drafts.csswg.org/css-borders-4/#box-shadow-color
#[syntax(" <color># ")]
#[initial("currentcolor")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BoxShadowColorStyleValue<'a>;

// /// Represents the style value for `box-shadow-offset` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-offset).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ none | <length>{2} ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#box-shadow-offset
// #[syntax(" [ none | <length>{2} ]# ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value, treating none as 0 0 when interpolated with non-none values.")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct BoxShadowOffsetStyleValue<'a>;

/// Represents the style value for `box-shadow-blur` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-blur).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length [0,∞]>#
/// ```
///
// https://drafts.csswg.org/css-borders-4/#box-shadow-blur
#[syntax(" <length [0,∞]># ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BoxShadowBlurStyleValue<'a>;

/// Represents the style value for `box-shadow-spread` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-spread).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>#
/// ```
///
// https://drafts.csswg.org/css-borders-4/#box-shadow-spread
#[syntax(" <length># ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BoxShadowSpreadStyleValue<'a>;

/// Represents the style value for `box-shadow-position` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow-position).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ outset | inset ]#
/// ```
///
// https://drafts.csswg.org/css-borders-4/#box-shadow-position
#[syntax(" [ outset | inset ]# ")]
#[initial("outset")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BoxShadowPositionStyleValue<'a>;

/// Represents the style value for `box-shadow` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#box-shadow).
///
/// The box-shadow CSS property applies shadow effects around an element's frame. This can create drop shadow and inner shadow effects.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <spread-shadow>#
/// ```
///
// https://drafts.csswg.org/css-borders-4/#box-shadow
#[syntax(" <spread-shadow># ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-boxshadow")]
#[baseline(widely)]
#[versions(chrome:10,chrome_android:18,edge:12,firefox:4,firefox_android:4,safari:5.1,safari_ios:5)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BoxShadowStyleValue<'a>;

// /// Represents the style value for `border-shape` as defined in [css-borders-4](https://drafts.csswg.org/css-borders-4/#border-shape).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ <basic-shape> <geometry-box>?]{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-borders-4/#border-shape
// #[syntax(" none | [ <basic-shape> <geometry-box>?]{1,2} ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("relative to the given <geometry-box>, or to border box if not given.")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum BorderShapeStyleValue {}
