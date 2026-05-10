//! CSS properties defined in the CSS2/CSS2.2 specification.
//! <https://drafts.csswg.org/css2/>

mod impls;
use super::prelude::*;

/// Represents the style value for `z-index` as defined in [css2](https://drafts.csswg.org/css2/#z-index).
///
/// The z-index CSS property orders overlapping elements, with higher values appearing in front of
/// or on top of lower values.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <integer>
/// ```
///
/// <https://drafts.csswg.org/css2/#z-index>
#[syntax(" auto | <integer> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Css2,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.z-index"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ZIndexStyleValue;
