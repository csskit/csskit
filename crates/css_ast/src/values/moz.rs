//! Mozilla-prefixed CSS property value types.
//!
//! Non-standard aliases for standardised properties, kept for compatibility
//! with legacy stylesheets.

use super::prelude::*;

/// `-moz-column-gap` — alias for `column-gap`.
#[syntax(" normal | <length-percentage [0,∞]> | <line-width> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Gaps,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MozColumnGapStyleValue {}

/// `-moz-column-count` — alias for `column-count`.
#[syntax(" auto | <integer [1,∞]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Multicol,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozColumnCountStyleValue;

/// `-moz-user-select` — alias for `user-select`.
#[syntax(" auto | text | none | all ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Ui,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MozUserSelectStyleValue {}

/// `-moz-appearance` — alias for `appearance`.
#[syntax(" none | auto | base | base-select | <compat-auto> | <compat-special> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Ui,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MozAppearanceStyleValue {}

/// `-moz-osx-font-smoothing` — non-standard macOS font smoothing.
///
/// Equivalent to `-webkit-font-smoothing` on macOS.
#[syntax(" auto | grayscale ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    inherits,
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Fonts,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MozOsxFontSmoothingStyleValue {}

/// `-moz-transition` — alias for `transition`.
#[syntax(" <single-transition># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "see individual properties",
    applies_to = Elements,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozTransitionStyleValue<'a>;

/// `-moz-box-sizing` — alias for `box-sizing`.
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
pub enum MozBoxSizingStyleValue {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_moz_column_gap_parses() {
		assert_parse!(CssAtomSet::ATOMS, MozColumnGapStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, MozColumnGapStyleValue, "1rem");
		assert_parse!(CssAtomSet::ATOMS, MozColumnGapStyleValue, "0");
	}

	#[test]
	fn test_moz_column_gap_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MozColumnGapStyleValue, "invalid");
	}

	#[test]
	fn test_moz_column_count_parses() {
		assert_parse!(CssAtomSet::ATOMS, MozColumnCountStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MozColumnCountStyleValue, "3");
	}

	#[test]
	fn test_moz_column_count_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MozColumnCountStyleValue, "0");
	}

	#[test]
	fn test_moz_user_select_parses() {
		assert_parse!(CssAtomSet::ATOMS, MozUserSelectStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MozUserSelectStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MozUserSelectStyleValue, "all");
	}

	#[test]
	fn test_moz_user_select_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MozUserSelectStyleValue, "invalid");
	}

	#[test]
	fn test_moz_appearance_parses() {
		assert_parse!(CssAtomSet::ATOMS, MozAppearanceStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MozAppearanceStyleValue, "auto");
	}

	#[test]
	fn test_moz_appearance_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MozAppearanceStyleValue, "invalid");
	}

	#[test]
	fn test_moz_osx_font_smoothing_parses() {
		assert_parse!(CssAtomSet::ATOMS, MozOsxFontSmoothingStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MozOsxFontSmoothingStyleValue, "grayscale");
	}

	#[test]
	fn test_moz_osx_font_smoothing_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MozOsxFontSmoothingStyleValue, "antialiased");
	}

	#[test]
	fn test_moz_transition_parses() {
		assert_parse!(CssAtomSet::ATOMS, MozTransitionStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MozTransitionStyleValue, "all 0.3s ease");
	}

	#[test]
	fn test_moz_transition_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MozTransitionStyleValue, "invalid!!!!");
	}

	#[test]
	fn test_moz_box_sizing_parses() {
		assert_parse!(CssAtomSet::ATOMS, MozBoxSizingStyleValue, "content-box");
		assert_parse!(CssAtomSet::ATOMS, MozBoxSizingStyleValue, "border-box");
	}

	#[test]
	fn test_moz_box_sizing_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MozBoxSizingStyleValue, "invalid");
	}
}
