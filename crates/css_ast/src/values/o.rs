//! Opera-prefixed CSS property value types.
//!
//! Non-standard aliases for standardised properties, kept for compatibility
//! with legacy stylesheets targeting Opera Presto.

use super::prelude::*;

/// `-o-object-fit` — alias for `object-fit`.
#[syntax(" fill | none | [ contain | cover ] || scale-down ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "fill",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Images,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum OObjectFitStyleValue {}

/// `-o-box-sizing` — alias for `box-sizing`.
#[syntax(" content-box | border-box ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "content-box",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Sizing,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum OBoxSizingStyleValue {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_o_object_fit_parses() {
		assert_parse!(CssAtomSet::ATOMS, OObjectFitStyleValue, "fill");
		assert_parse!(CssAtomSet::ATOMS, OObjectFitStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, OObjectFitStyleValue, "contain");
		assert_parse!(CssAtomSet::ATOMS, OObjectFitStyleValue, "cover");
		assert_parse!(CssAtomSet::ATOMS, OObjectFitStyleValue, "scale-down");
	}

	#[test]
	fn test_o_object_fit_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, OObjectFitStyleValue, "invalid");
	}

	#[test]
	fn test_o_box_sizing_parses() {
		assert_parse!(CssAtomSet::ATOMS, OBoxSizingStyleValue, "content-box");
		assert_parse!(CssAtomSet::ATOMS, OBoxSizingStyleValue, "border-box");
	}

	#[test]
	fn test_o_box_sizing_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, OBoxSizingStyleValue, "invalid");
	}
}
