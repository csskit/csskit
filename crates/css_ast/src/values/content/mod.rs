#![allow(warnings)]
//! CSS Generated Content Module Level 3
//! https://drafts.csswg.org/css-content-3/

mod impls;
use impls::*;

// /// Represents the style value for `content` as defined in [css-content-3](https://drafts.csswg.org/css-content-3/#content).
// ///
// /// The content CSS property sets the content inside of an element or pseudo-element, replacing the current value. It's often used with the ::before and ::after pseudo-elements to generate cosmetic content.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | none | [ <content-replacement> | <content-list> ] [/ [ <string> | <counter> | <attr()> ]+ ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-content-3/#content
// #[value(" normal | none | [ <content-replacement> | <content-list> ] [/ [ <string> | <counter> | <attr()> ]+ ]? ")]
// #[initial("normal")]
// #[applies_to("all elements, tree-abiding pseudo-elements, and page margin boxes")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(81.613)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
// pub enum ContentStyleValue<'a> {}

// /// Represents the style value for `quotes` as defined in [css-content-3](https://drafts.csswg.org/css-content-3/#quotes).
// ///
// /// The quotes CSS property sets the quotation marks inserted via the content CSS property or <q> element.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | none | match-parent | [ <string> <string> ]+
// /// ```
// ///
// // https://drafts.csswg.org/css-content-3/#quotes
// #[value(" auto | none | match-parent | [ <string> <string> ]+ ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(81.613)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:87,chrome_android:87,edge:87,firefox:70,firefox_android:79,safari:14.1,safari_ios:14.5)]
// pub enum QuotesStyleValue<'a> {}

/// Represents the style value for `bookmark-level` as defined in [css-content-3](https://drafts.csswg.org/css-content-3/#bookmark-level).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <integer [1,∞]>
/// ```
///
// https://drafts.csswg.org/css-content-3/#bookmark-level
#[value(" none | <integer [1,∞]> ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(81.613)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct BookmarkLevelStyleValue;

/// Represents the style value for `bookmark-label` as defined in [css-content-3](https://drafts.csswg.org/css-content-3/#bookmark-label).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <content-list>
/// ```
///
// https://drafts.csswg.org/css-content-3/#bookmark-label
#[value(" <content-list> ")]
#[initial("content(text)")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(81.613)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct BookmarkLabelStyleValue<'a>;

/// Represents the style value for `bookmark-state` as defined in [css-content-3](https://drafts.csswg.org/css-content-3/#bookmark-state).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// open | closed
/// ```
///
// https://drafts.csswg.org/css-content-3/#bookmark-state
#[value(" open | closed ")]
#[initial("open")]
#[applies_to("block-level elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(81.613)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum BookmarkStateStyleValue {}
