#![allow(warnings)]
//! CSS Cascading and Inheritance Level 5
//! https://drafts.csswg.org/css-cascade-6/

mod impls;
use impls::*;

/// Represents the style value for `all` as defined in [css-cascade-6](https://drafts.csswg.org/css-cascade-6/#all).
///
/// The all CSS property is a shorthand for all CSS properties, except for direction and unicode-bidi. It accepts only the keywords for explicit defaulting (such as initial and inherit), since they are the only values supported on all CSS properties.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// initial | inherit | unset | revert | revert-layer
/// ```
///
// https://drafts.csswg.org/css-cascade-6/#all
#[syntax(" initial | inherit | unset | revert | revert-layer ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-all")]
#[baseline(widely)]
#[versions(chrome:37,chrome_android:37,edge:79,firefox:27,firefox_android:27,safari:9.1,safari_ios:9.3)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum AllStyleValue {}
