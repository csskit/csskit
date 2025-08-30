#![allow(warnings)]
//! CSS Will Change Module Level 1
//! https://drafts.csswg.org/css-will-change-1/

mod impls;
use impls::*;

/// Represents the style value for `will-change` as defined in [css-will-change-1](https://drafts.csswg.org/css-will-change-1/#will-change).
///
/// The will-change CSS property gives hints to the browser about expected changes to an element's scroll position, contents, or style. These hints allow browsers to optimize for upcoming style changes.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <animateable-feature>#
/// ```
///
// https://drafts.csswg.org/css-will-change-1/#will-change
#[value(" auto | <animateable-feature># ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(36.864)]
#[caniuse("https://caniuse.com/will-change")]
#[baseline(widely)]
#[versions(chrome:36,chrome_android:36,edge:79,firefox:36,firefox_android:36,safari:9.1,safari_ios:9.3)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct WillChangeStyleValue<'a>;
