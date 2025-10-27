#![allow(warnings)]
//! https://drafts.csswg.org/css-cascade-6/

mod impls;
use super::prelude::*;
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
/// https://drafts.csswg.org/css-cascade-6/#all
#[syntax(" initial | inherit | unset | revert | revert-layer ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    percentages = Unknown,
    animation_type = Unknown,
    property_group = Cascade,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.all"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum AllStyleValue {}
