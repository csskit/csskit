#![allow(warnings)]
//! CSS Form Control Styling Level 1
//! https://drafts.csswg.org/css-forms-1/

mod impls;
use impls::*;

/// Represents the style value for `field-sizing` as defined in [css-forms-1](https://drafts.csswg.org/css-forms-1/#field-sizing).
///
/// The field-sizing CSS property allows form controls such as <textarea> to be sized based on their content.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// fixed | content
/// ```
///
// https://drafts.csswg.org/css-forms-1/#field-sizing
#[syntax(" fixed | content ")]
#[initial("fixed")]
#[applies_to("elements with default preferred size")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:123,chrome_android:123,edge:123)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum FieldSizingStyleValue {}

/// Represents the style value for `slider-orientation` as defined in [css-forms-1](https://drafts.csswg.org/css-forms-1/#slider-orientation).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | left-to-right | right-to-left | top-to-bottom | bottom-to-top
/// ```
///
// https://drafts.csswg.org/css-forms-1/#slider-orientation
#[syntax(" auto | left-to-right | right-to-left | top-to-bottom | bottom-to-top ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum SliderOrientationStyleValue {}

/// Represents the style value for `input-security` as defined in [css-forms-1](https://drafts.csswg.org/css-forms-1/#input-security).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none
/// ```
///
// https://drafts.csswg.org/css-forms-1/#input-security
#[syntax(" auto | none ")]
#[initial("auto")]
#[applies_to("sensitive text inputs")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum InputSecurityStyleValue {}
