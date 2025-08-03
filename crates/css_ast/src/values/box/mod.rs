#![allow(warnings)]
//! CSS Box Model Module Level 4
//! https://drafts.csswg.org/css-box-4/

mod impls;
use impls::*;

/// Represents the style value for `margin-top` as defined in [css-box-4](https://drafts.csswg.org/css-box-4/#margin-top).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage> | auto
/// ```
///
// https://drafts.csswg.org/css-box-4/#margin-top
#[value(" <length-percentage> | auto ")]
#[initial("0")]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct MarginTopStyleValue;

/// Represents the style value for `margin-right` as defined in [css-box-4](https://drafts.csswg.org/css-box-4/#margin-right).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage> | auto
/// ```
///
// https://drafts.csswg.org/css-box-4/#margin-right
#[value(" <length-percentage> | auto ")]
#[initial("0")]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct MarginRightStyleValue;

/// Represents the style value for `margin-bottom` as defined in [css-box-4](https://drafts.csswg.org/css-box-4/#margin-bottom).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage> | auto
/// ```
///
// https://drafts.csswg.org/css-box-4/#margin-bottom
#[value(" <length-percentage> | auto ")]
#[initial("0")]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct MarginBottomStyleValue;

/// Represents the style value for `margin-left` as defined in [css-box-4](https://drafts.csswg.org/css-box-4/#margin-left).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage> | auto
/// ```
///
// https://drafts.csswg.org/css-box-4/#margin-left
#[value(" <length-percentage> | auto ")]
#[initial("0")]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct MarginLeftStyleValue;

/// Represents the style value for `margin` as defined in [css-box-4](https://drafts.csswg.org/css-box-4/#margin).
///
/// The margin CSS property sets space around an element. It is a shorthand for margin-top, margin-right, margin-bottom, and margin-left.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'margin-top'>{1,4}
/// ```
///
// https://drafts.csswg.org/css-box-4/#margin
#[value(" <'margin-top'>{1,4} ")]
#[initial("0")]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
pub struct MarginStyleValue;

/// Represents the style value for `padding-top` as defined in [css-box-4](https://drafts.csswg.org/css-box-4/#padding-top).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-box-4/#padding-top
#[value(" <length-percentage [0,∞]> ")]
#[initial("0")]
#[applies_to(
	"all elements except: internal table elements other than table cells, ruby base containers, and ruby annotation containers"
)]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct PaddingTopStyleValue;

/// Represents the style value for `padding-right` as defined in [css-box-4](https://drafts.csswg.org/css-box-4/#padding-right).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-box-4/#padding-right
#[value(" <length-percentage [0,∞]> ")]
#[initial("0")]
#[applies_to(
	"all elements except: internal table elements other than table cells, ruby base containers, and ruby annotation containers"
)]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct PaddingRightStyleValue;

/// Represents the style value for `padding-bottom` as defined in [css-box-4](https://drafts.csswg.org/css-box-4/#padding-bottom).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-box-4/#padding-bottom
#[value(" <length-percentage [0,∞]> ")]
#[initial("0")]
#[applies_to(
	"all elements except: internal table elements other than table cells, ruby base containers, and ruby annotation containers"
)]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct PaddingBottomStyleValue;

/// Represents the style value for `padding-left` as defined in [css-box-4](https://drafts.csswg.org/css-box-4/#padding-left).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-box-4/#padding-left
#[value(" <length-percentage [0,∞]> ")]
#[initial("0")]
#[applies_to(
	"all elements except: internal table elements other than table cells, ruby base containers, and ruby annotation containers"
)]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct PaddingLeftStyleValue;

/// Represents the style value for `padding` as defined in [css-box-4](https://drafts.csswg.org/css-box-4/#padding).
///
/// The padding CSS property sets space between an element's edge and its contents. It is a shorthand for padding-top, padding-right, padding-bottom, and padding-left.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'padding-top'>{1,4}
/// ```
///
// https://drafts.csswg.org/css-box-4/#padding
#[value(" <'padding-top'>{1,4} ")]
#[initial("0")]
#[applies_to(
	"all elements except: internal table elements other than table cells, ruby base containers, and ruby annotation containers"
)]
#[inherited("no")]
#[percentages("refer to logical width of containing block")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
pub struct PaddingStyleValue;

// /// Represents the style value for `margin-trim` as defined in [css-box-4](https://drafts.csswg.org/css-box-4/#margin-trim).
// ///
// /// The margin-trim CSS property removes the margins of child elements when they meet the edges of the container.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ block || inline ] | [ block-start || inline-start || block-end || inline-end ]
// /// ```
// ///
// // https://drafts.csswg.org/css-box-4/#margin-trim
// #[value(" none | [ block || inline ] | [ block-start || inline-start || block-end || inline-end ] ")]
// #[initial("none")]
// #[applies_to("block containers, multi-column containers, flex containers, grid containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(limited)]
// #[versions(safari:16.4,safari_ios:16.4)]
// pub enum MarginTrimStyleValue {}
