#![allow(warnings)]
//! CSS Logical Properties and Values Level 1
//! https://drafts.csswg.org/css-logical-1/

mod impls;
use impls::*;

/// Represents the style value for `block-size` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#block-size).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'width'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#block-size
#[syntax(" <'width'> ")]
#[initial("auto")]
#[applies_to("Same as height and width")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BlockSizeStyleValue;

/// Represents the style value for `inline-size` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#inline-size).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'width'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#inline-size
#[syntax(" <'width'> ")]
#[initial("auto")]
#[applies_to("Same as height and width")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct InlineSizeStyleValue;

/// Represents the style value for `min-block-size` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#min-block-size).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'min-width'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#min-block-size
#[syntax(" <'min-width'> ")]
#[initial("0")]
#[applies_to("same as height and width")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct MinBlockSizeStyleValue;

/// Represents the style value for `min-inline-size` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#min-inline-size).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'min-width'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#min-inline-size
#[syntax(" <'min-width'> ")]
#[initial("0")]
#[applies_to("same as height and width")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct MinInlineSizeStyleValue;

/// Represents the style value for `max-block-size` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#max-block-size).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'max-width'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#max-block-size
#[syntax(" <'max-width'> ")]
#[initial("none")]
#[applies_to("same as height and width")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct MaxBlockSizeStyleValue;

/// Represents the style value for `max-inline-size` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#max-inline-size).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'max-width'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#max-inline-size
#[syntax(" <'max-width'> ")]
#[initial("none")]
#[applies_to("same as height and width")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct MaxInlineSizeStyleValue;

/// Represents the style value for `margin-block-start` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#margin-block-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#margin-block-start
#[syntax(" <'margin-top'> ")]
#[initial("0")]
#[applies_to("Same as margin-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct MarginBlockStartStyleValue;

/// Represents the style value for `margin-block-end` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#margin-block-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#margin-block-end
#[syntax(" <'margin-top'> ")]
#[initial("0")]
#[applies_to("Same as margin-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct MarginBlockEndStyleValue;

/// Represents the style value for `margin-inline-start` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#margin-inline-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#margin-inline-start
#[syntax(" <'margin-top'> ")]
#[initial("0")]
#[applies_to("Same as margin-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct MarginInlineStartStyleValue;

/// Represents the style value for `margin-inline-end` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#margin-inline-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#margin-inline-end
#[syntax(" <'margin-top'> ")]
#[initial("0")]
#[applies_to("Same as margin-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct MarginInlineEndStyleValue;

/// Represents the style value for `margin-block` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#margin-block).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-logical-1/#margin-block
#[syntax(" <'margin-top'>{1,2} ")]
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
pub struct MarginBlockStyleValue;

/// Represents the style value for `margin-inline` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#margin-inline).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-logical-1/#margin-inline
#[syntax(" <'margin-top'>{1,2} ")]
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
pub struct MarginInlineStyleValue;

/// Represents the style value for `padding-block-start` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#padding-block-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#padding-block-start
#[syntax(" <'padding-top'> ")]
#[initial("0")]
#[applies_to("Same as padding-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct PaddingBlockStartStyleValue;

/// Represents the style value for `padding-block-end` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#padding-block-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#padding-block-end
#[syntax(" <'padding-top'> ")]
#[initial("0")]
#[applies_to("Same as padding-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct PaddingBlockEndStyleValue;

/// Represents the style value for `padding-inline-start` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#padding-inline-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#padding-inline-start
#[syntax(" <'padding-top'> ")]
#[initial("0")]
#[applies_to("Same as padding-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct PaddingInlineStartStyleValue;

/// Represents the style value for `padding-inline-end` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#padding-inline-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>
/// ```
///
// https://drafts.csswg.org/css-logical-1/#padding-inline-end
#[syntax(" <'padding-top'> ")]
#[initial("0")]
#[applies_to("Same as padding-top")]
#[inherited("no")]
#[percentages("as for the corresponding physical property")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct PaddingInlineEndStyleValue;

/// Represents the style value for `padding-block` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#padding-block).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-logical-1/#padding-block
#[syntax(" <'padding-top'>{1,2} ")]
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
pub struct PaddingBlockStyleValue;

/// Represents the style value for `padding-inline` as defined in [css-logical-1](https://drafts.csswg.org/css-logical-1/#padding-inline).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-logical-1/#padding-inline
#[syntax(" <'padding-top'>{1,2} ")]
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
pub struct PaddingInlineStyleValue;
