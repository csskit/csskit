//! Non-standard CSS property value types.
//!
//! Non-standard aliases for standardised properties, kept for compatibility
//! with legacy stylesheets.

use super::prelude::*;

/// Represents the style value for `word-wrap`.
///
/// Legacy alias for `overflow-wrap`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | break-word | anywhere
/// ```
#[syntax(" normal | break-word | anywhere ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "normal",
    inherits,
    applies_to = Text,
    animation_type = Discrete,
    property_group = Text,
    computed_value_type = Unknown,
    canonical_order = "n/a",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.overflow-wrap"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WordWrapStyleValue {}
