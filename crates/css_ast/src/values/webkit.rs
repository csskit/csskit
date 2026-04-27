//! Webkit-prefixed CSS property value types.
//!
//! Non-standard aliases for standardised properties, kept for compatibility
//! with legacy stylesheets.

use super::prelude::*;

/// Represents the style value for `-webkit-filter`.
///
/// Legacy alias for `filter`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <filter-value-list>
/// ```
#[syntax(" none | <filter-value-list> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = FilterEffects,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitFilterStyleValue<'a>;

/// Represents the style value for `-webkit-flex`.
///
/// Legacy alias for `flex`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | [ <'flex-grow'> <'flex-shrink'>? || <'flex-basis'> ]
/// ```
#[syntax(" none | [ <'flex-grow'> <'flex-shrink'>? || <'flex-basis'> ] ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0 1 auto",
    applies_to = Elements,
    animation_type = Unknown,
    property_group = Flexbox,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitFlexStyleValue;

/// Represents the style value for `-webkit-order`.
///
/// Legacy alias for `order`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <integer>
/// ```
#[syntax(" <integer> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Flexbox,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitOrderStyleValue;

/// Represents the style value for `-webkit-transition`.
///
/// Legacy alias for `transition`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-transition>#
/// ```
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
pub struct WebkitTransitionStyleValue<'a>;

/// Represents the style value for `-webkit-appearance`.
///
/// Legacy alias for `appearance`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | auto | base | base-select | <compat-auto> | <compat-special>
/// ```
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
pub enum WebkitAppearanceStyleValue {}

/// Represents the style value for `-webkit-transform`.
///
/// Legacy alias for `transform`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <transform-list>
/// ```
#[syntax(" none | <transform-list> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Transforms,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTransformStyleValue<'a>;

/// Represents the style value for `-webkit-font-smoothing`.
///
/// Non-standard WebKit property controlling font antialiasing.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none | antialiased | subpixel-antialiased
/// ```
#[syntax(" auto | none | antialiased | subpixel-antialiased ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Fonts,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitFontSmoothingStyleValue {}

/// Represents the style value for `-webkit-text-size-adjust`.
///
/// Legacy alias for `text-size-adjust`.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none | <percentage [0,∞]>
/// ```
#[syntax(" auto | none | <percentage [0,∞]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    inherits,
    applies_to = Elements,
    animation_type = ByComputedValue,
    percentages = Unknown,
    property_group = SizeAdjust,
    computed_value_type = Unknown,
    canonical_order = "N/A",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTextSizeAdjustStyleValue;

/// Represents the style value for `-webkit-animation-delay`.
///
/// Legacy alias for `animation-delay`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time>#
/// ```
#[syntax(" <time># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0s",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitAnimationDelayStyleValue<'a>;

/// Represents the style value for `-webkit-animation-duration`.
///
/// Legacy alias for `animation-duration`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ auto | <time [0s,∞]> ]#
/// ```
#[syntax(" [ auto | <time [0s,∞]> ]# ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitAnimationDurationStyleValue<'a>;

/// Represents the style value for `-webkit-animation-fill-mode`.
///
/// Legacy alias for `animation-fill-mode`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-fill-mode>#
/// ```
#[syntax(" <single-animation-fill-mode># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitAnimationFillModeStyleValue<'a>;

/// Represents the style value for `-webkit-animation-iteration-count`.
///
/// Legacy alias for `animation-iteration-count`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <single-animation-iteration-count>#
/// ```
#[syntax(" <single-animation-iteration-count># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "1",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitAnimationIterationCountStyleValue<'a>;

/// Represents the style value for `-webkit-animation-name`.
///
/// Legacy alias for `animation-name`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ none | <keyframes-name> ]#
/// ```
#[syntax(" [ none | <keyframes-name> ]# ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitAnimationNameStyleValue<'a>;

/// Represents the style value for `-webkit-animation-timing-function`.
///
/// Legacy alias for `animation-timing-function`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <easing-function>#
/// ```
#[syntax(" <easing-function># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "ease",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitAnimationTimingFunctionStyleValue<'a>;

/// Represents the style value for `-webkit-backface-visibility`.
///
/// Legacy alias for `backface-visibility`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// visible | hidden
/// ```
#[syntax(" visible | hidden ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "visible",
    applies_to = Elements,
    property_group = Transforms,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitBackfaceVisibilityStyleValue {}

/// Represents the style value for `-webkit-tap-highlight-color`.
///
/// Non-standard property. Sets the highlight colour when an element is tapped.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color>
/// ```
#[syntax(" <color> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "transparent",
    applies_to = Elements,
    property_group = Ui,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTapHighlightColorStyleValue<'a>;

/// Represents the style value for `-webkit-transition-duration`.
///
/// Legacy alias for `transition-duration`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <time [0s,∞]>#
/// ```
#[syntax(" <time [0s,∞]># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0s",
    applies_to = Elements,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTransitionDurationStyleValue<'a>;

/// Represents the style value for `-webkit-transition-timing-function`.
///
/// Legacy alias for `transition-timing-function`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <easing-function>#
/// ```
#[syntax(" <easing-function># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "ease",
    applies_to = Elements,
    property_group = Transitions,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTransitionTimingFunctionStyleValue<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WebkitFilterStyleValue>(), 32);
	}

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitFilterStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitFilterStyleValue, "blur(4px)");
		assert_parse!(CssAtomSet::ATOMS, WebkitFilterStyleValue, "brightness(0.5) contrast(1.2)");
		assert_parse!(CssAtomSet::ATOMS, WebkitFilterStyleValue, "drop-shadow(2px 4px)");
		assert_parse!(CssAtomSet::ATOMS, WebkitFilterStyleValue, "drop-shadow(red 2px 4px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitFilterStyleValue, "invalid");
	}

	#[test]
	fn size_test_transition() {
		assert_eq!(
			std::mem::size_of::<WebkitTransitionStyleValue>(),
			std::mem::size_of::<crate::values::TransitionStyleValue>()
		);
	}

	#[test]
	fn test_transition_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionStyleValue, "all 0.3s ease");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionStyleValue, "opacity 1s, transform 0.5s");
	}

	#[test]
	fn test_transition_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitTransitionStyleValue, "invalid!!!!");
	}

	#[test]
	fn size_test_flex() {
		assert_eq!(std::mem::size_of::<WebkitFlexStyleValue>(), std::mem::size_of::<crate::values::FlexStyleValue>());
	}

	#[test]
	fn test_flex_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitFlexStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitFlexStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, WebkitFlexStyleValue, "1 1 auto");
	}

	#[test]
	fn test_flex_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitFlexStyleValue, "invalid");
	}

	#[test]
	fn size_test_order() {
		assert_eq!(std::mem::size_of::<WebkitOrderStyleValue>(), std::mem::size_of::<crate::values::OrderStyleValue>());
	}

	#[test]
	fn test_order_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitOrderStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, WebkitOrderStyleValue, "-1");
		assert_parse!(CssAtomSet::ATOMS, WebkitOrderStyleValue, "5");
	}

	#[test]
	fn test_order_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitOrderStyleValue, "none");
	}

	#[test]
	fn size_test_appearance() {
		assert_eq!(std::mem::size_of::<WebkitAppearanceStyleValue>(), 20);
	}

	#[test]
	fn test_appearance_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitAppearanceStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitAppearanceStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, WebkitAppearanceStyleValue, "textfield");
		assert_parse!(CssAtomSet::ATOMS, WebkitAppearanceStyleValue, "button");
	}

	#[test]
	fn test_appearance_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitAppearanceStyleValue, "invalid");
	}

	#[test]
	fn size_test_transform() {
		assert_eq!(std::mem::size_of::<WebkitTransformStyleValue>(), 32);
	}

	#[test]
	fn test_transform_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitTransformStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransformStyleValue, "rotate(45deg)");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransformStyleValue, "scale(-1, 1)");
	}

	#[test]
	fn test_transform_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitTransformStyleValue, "invalid");
	}

	#[test]
	fn size_test_font_smoothing() {
		assert_eq!(std::mem::size_of::<WebkitFontSmoothingStyleValue>(), 16);
	}

	#[test]
	fn test_font_smoothing_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitFontSmoothingStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, WebkitFontSmoothingStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitFontSmoothingStyleValue, "antialiased");
		assert_parse!(CssAtomSet::ATOMS, WebkitFontSmoothingStyleValue, "subpixel-antialiased");
	}

	#[test]
	fn test_font_smoothing_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitFontSmoothingStyleValue, "invalid");
	}

	#[test]
	fn size_test_text_size_adjust() {
		assert_eq!(std::mem::size_of::<WebkitTextSizeAdjustStyleValue>(), 16);
	}

	#[test]
	fn test_text_size_adjust_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitTextSizeAdjustStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, WebkitTextSizeAdjustStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitTextSizeAdjustStyleValue, "100%");
	}

	#[test]
	fn test_text_size_adjust_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitTextSizeAdjustStyleValue, "invalid");
	}

	#[test]
	fn size_test_animation_delay() {
		assert_eq!(
			std::mem::size_of::<WebkitAnimationDelayStyleValue>(),
			std::mem::size_of::<crate::values::AnimationDelayStyleValue>()
		);
	}

	#[test]
	fn test_animation_delay_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationDelayStyleValue, "0s");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationDelayStyleValue, "0.5s, 1s");
	}

	#[test]
	fn test_animation_delay_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitAnimationDelayStyleValue, "invalid");
	}

	#[test]
	fn size_test_animation_duration() {
		assert_eq!(
			std::mem::size_of::<WebkitAnimationDurationStyleValue>(),
			std::mem::size_of::<crate::values::AnimationDurationStyleValue>()
		);
	}

	#[test]
	fn test_animation_duration_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationDurationStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationDurationStyleValue, "0.3s");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationDurationStyleValue, "1s, 2s");
	}

	#[test]
	fn test_animation_duration_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitAnimationDurationStyleValue, "invalid");
	}

	#[test]
	fn size_test_animation_fill_mode() {
		assert_eq!(
			std::mem::size_of::<WebkitAnimationFillModeStyleValue>(),
			std::mem::size_of::<crate::values::AnimationFillModeStyleValue>()
		);
	}

	#[test]
	fn test_animation_fill_mode_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationFillModeStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationFillModeStyleValue, "forwards");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationFillModeStyleValue, "both");
	}

	#[test]
	fn test_animation_fill_mode_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitAnimationFillModeStyleValue, "invalid");
	}

	#[test]
	fn size_test_animation_iteration_count() {
		assert_eq!(
			std::mem::size_of::<WebkitAnimationIterationCountStyleValue>(),
			std::mem::size_of::<crate::values::AnimationIterationCountStyleValue>()
		);
	}

	#[test]
	fn test_animation_iteration_count_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationIterationCountStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationIterationCountStyleValue, "infinite");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationIterationCountStyleValue, "3");
	}

	#[test]
	fn test_animation_iteration_count_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitAnimationIterationCountStyleValue, "invalid");
	}

	#[test]
	fn size_test_animation_name() {
		assert_eq!(
			std::mem::size_of::<WebkitAnimationNameStyleValue>(),
			std::mem::size_of::<crate::values::AnimationNameStyleValue>()
		);
	}

	#[test]
	fn test_animation_name_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationNameStyleValue, "my-animation");
	}

	#[test]
	fn test_animation_name_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitAnimationNameStyleValue, "");
	}

	#[test]
	fn size_test_animation_timing_function() {
		assert_eq!(
			std::mem::size_of::<WebkitAnimationTimingFunctionStyleValue>(),
			std::mem::size_of::<crate::values::AnimationTimingFunctionStyleValue>()
		);
	}

	#[test]
	fn test_animation_timing_function_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationTimingFunctionStyleValue, "ease");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationTimingFunctionStyleValue, "linear");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationTimingFunctionStyleValue, "ease-in-out");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationTimingFunctionStyleValue, "cubic-bezier(0.4, 0, 0.2, 1)");
	}

	#[test]
	fn test_animation_timing_function_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitAnimationTimingFunctionStyleValue, "invalid");
	}

	#[test]
	fn size_test_backface_visibility() {
		assert_eq!(
			std::mem::size_of::<WebkitBackfaceVisibilityStyleValue>(),
			std::mem::size_of::<crate::values::BackfaceVisibilityStyleValue>()
		);
	}

	#[test]
	fn test_backface_visibility_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitBackfaceVisibilityStyleValue, "visible");
		assert_parse!(CssAtomSet::ATOMS, WebkitBackfaceVisibilityStyleValue, "hidden");
	}

	#[test]
	fn test_backface_visibility_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitBackfaceVisibilityStyleValue, "invalid");
	}

	#[test]
	fn size_test_tap_highlight_color() {
		assert_eq!(
			std::mem::size_of::<WebkitTapHighlightColorStyleValue>(),
			std::mem::size_of::<crate::values::BackgroundColorStyleValue>()
		);
	}

	#[test]
	fn test_tap_highlight_color_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitTapHighlightColorStyleValue, "transparent");
		assert_parse!(CssAtomSet::ATOMS, WebkitTapHighlightColorStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, WebkitTapHighlightColorStyleValue, "rgba(0, 0, 0, 0)");
	}

	#[test]
	fn test_tap_highlight_color_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitTapHighlightColorStyleValue, "invalid");
	}

	#[test]
	fn size_test_transition_duration() {
		assert_eq!(
			std::mem::size_of::<WebkitTransitionDurationStyleValue>(),
			std::mem::size_of::<crate::values::TransitionDurationStyleValue>()
		);
	}

	#[test]
	fn test_transition_duration_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionDurationStyleValue, "0s");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionDurationStyleValue, "0.3s");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionDurationStyleValue, "1s, 200ms");
	}

	#[test]
	fn test_transition_duration_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitTransitionDurationStyleValue, "invalid");
	}

	#[test]
	fn size_test_transition_timing_function() {
		assert_eq!(
			std::mem::size_of::<WebkitTransitionTimingFunctionStyleValue>(),
			std::mem::size_of::<crate::values::TransitionTimingFunctionStyleValue>()
		);
	}

	#[test]
	fn test_transition_timing_function_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionTimingFunctionStyleValue, "ease");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionTimingFunctionStyleValue, "linear");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionTimingFunctionStyleValue, "cubic-bezier(0.4, 0, 0.2, 1)");
	}

	#[test]
	fn test_transition_timing_function_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WebkitTransitionTimingFunctionStyleValue, "invalid");
	}
}
