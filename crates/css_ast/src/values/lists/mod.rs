#![allow(warnings)]
//! CSS Lists and Counters Module Level 3
//! https://drafts.csswg.org/css-lists-3/

mod impls;
use impls::*;

/// Represents the style value for `list-style-image` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#list-style-image).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <image> | none
/// ```
///
// https://drafts.csswg.org/css-lists-3/#list-style-image
#[value(" <image> | none ")]
#[initial("none")]
#[applies_to("list items")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct ListStyleImageStyleValue<'a>;

/// Represents the style value for `list-style-type` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#list-style-type).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <counter-style> | <string> | none
/// ```
///
// https://drafts.csswg.org/css-lists-3/#list-style-type
#[value(" <counter-style> | <string> | none ")]
#[initial("disc")]
#[applies_to("list items")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum ListStyleTypeStyleValue<'a> {}

/// Represents the style value for `list-style-position` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#list-style-position).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// inside | outside
/// ```
///
// https://drafts.csswg.org/css-lists-3/#list-style-position
#[value(" inside | outside ")]
#[initial("outside")]
#[applies_to("list items")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum ListStylePositionStyleValue {}

// /// Represents the style value for `list-style` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#list-style).
// ///
// /// The list-style shorthand CSS property and the list-style-image, list-style-position, and list-style-type longhand properties set the position and appearance of a list item's marker.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'list-style-position'> || <'list-style-image'> || <'list-style-type'>
// /// ```
// ///
// // https://drafts.csswg.org/css-lists-3/#list-style
// #[value(" <'list-style-position'> || <'list-style-image'> || <'list-style-type'> ")]
// #[initial("see individual properties")]
// #[applies_to("list items")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
// pub struct ListStyleStyleValue;

/// Represents the style value for `marker-side` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#marker-side).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// match-self | match-parent
/// ```
///
// https://drafts.csswg.org/css-lists-3/#marker-side
#[value(" match-self | match-parent ")]
#[initial("match-self")]
#[applies_to("list items")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum MarkerSideStyleValue {}

// /// Represents the style value for `counter-reset` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#counter-reset).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <counter-name> <integer>? | <reversed-counter-name> <integer>? ]+ | none
// /// ```
// ///
// // https://drafts.csswg.org/css-lists-3/#counter-reset
// #[value(" [ <counter-name> <integer>? | <reversed-counter-name> <integer>? ]+ | none ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum CounterResetStyleValue<'a> {}

// /// Represents the style value for `counter-increment` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#counter-increment).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <counter-name> <integer>? ]+ | none
// /// ```
// ///
// // https://drafts.csswg.org/css-lists-3/#counter-increment
// #[value(" [ <counter-name> <integer>? ]+ | none ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum CounterIncrementStyleValue<'a> {}

// /// Represents the style value for `counter-set` as defined in [css-lists-3](https://drafts.csswg.org/css-lists-3/#counter-set).
// ///
// /// The counter-set CSS property creates (and optionally sets a value for) a counter, the numbers for a series of headings or ordered list items.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <counter-name> <integer>? ]+ | none
// /// ```
// ///
// // https://drafts.csswg.org/css-lists-3/#counter-set
// #[value(" [ <counter-name> <integer>? ]+ | none ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(newly)]
// #[versions(chrome:85,chrome_android:85,edge:85,firefox:68,firefox_android:68,safari:17.2,safari_ios:17.2)]
// pub enum CounterSetStyleValue<'a> {}
