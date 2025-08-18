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
#[value(" <'width'> ")]
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
pub struct BlockSizeStyleValue<'a>;

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
#[value(" <'width'> ")]
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
pub struct InlineSizeStyleValue<'a>;

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
#[value(" <'min-width'> ")]
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
pub struct MinBlockSizeStyleValue<'a>;

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
#[value(" <'min-width'> ")]
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
pub struct MinInlineSizeStyleValue<'a>;

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
#[value(" <'max-width'> ")]
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
pub struct MaxBlockSizeStyleValue<'a>;

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
#[value(" <'max-width'> ")]
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
pub struct MaxInlineSizeStyleValue<'a>;

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
#[value(" <'margin-top'> ")]
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
#[value(" <'margin-top'> ")]
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
#[value(" <'margin-top'> ")]
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
#[value(" <'margin-top'> ")]
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
#[value(" <'margin-top'>{1,2} ")]
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
#[value(" <'margin-top'>{1,2} ")]
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
#[value(" <'padding-top'> ")]
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
#[value(" <'padding-top'> ")]
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
#[value(" <'padding-top'> ")]
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
#[value(" <'padding-top'> ")]
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
#[value(" <'padding-top'>{1,2} ")]
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
#[value(" <'padding-top'>{1,2} ")]
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
pub struct PaddingInlineStyleValue;
