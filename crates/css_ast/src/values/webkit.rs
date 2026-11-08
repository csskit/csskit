//! Webkit-prefixed CSS property value types.
//!
//! Non-standard aliases for standardised properties, kept for compatibility
//! with legacy stylesheets.

use super::prelude::*;
use crate::{CalcableValue, Length, PositionOne, PositionTwo};
use css_parse::{Cursor, Parse, Parser, Result as ParseResult};

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
pub struct WebkitFlexStyleValue<'a>;

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
pub struct WebkitOrderStyleValue<'a>;

/// Represents the style value for `-webkit-transform-origin`.
///
/// Legacy alias for `transform-origin`. Accepts the same grammar.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <position-one> | <position-two> <length>?
/// ```
#[syntax(" <position-one> | <position-two> <length>? ")]
#[derive(
	Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
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
pub enum WebkitTransformOriginStyleValue<'a> {}

impl<'a> Parse<'a> for WebkitTransformOriginStyleValue<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParseResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let first = p.parse::<PositionOne>()?;
		if !p.peek::<PositionOne>() {
			return Ok(Self::PositionOne(crate::Value::Literal(first)));
		}
		let second_c = p.peek_n(1);
		let second = p.parse::<PositionOne>()?;
		let two = PositionTwo::from_two(p, first, second, second_c)?;
		Ok(Self::PositionTwo(crate::Value::Literal(two), p.parse_if_peek::<CalcableValue<Length>>()?))
	}
}

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
pub enum WebkitAppearanceStyleValue<'a> {}

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
pub enum WebkitFontSmoothingStyleValue<'a> {}

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
pub struct WebkitTextSizeAdjustStyleValue<'a>;

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
    shorthands = _WebkitAnimation,
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
    shorthands = _WebkitAnimation,
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
    shorthands = _WebkitAnimation,
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
    shorthands = _WebkitAnimation,
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
    shorthands = _WebkitAnimation,
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
    shorthands = _WebkitAnimation,
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
pub enum WebkitBackfaceVisibilityStyleValue<'a> {}

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

// ── Flexbox aliases ──────────────────────────────────────────────────────────

/// `-webkit-flex-direction` - alias for `flex-direction`.
#[syntax(" row | row-reverse | column | column-reverse ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "row",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Flexbox,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitFlexDirectionStyleValue<'a> {}

/// `-webkit-flex-wrap` - alias for `flex-wrap`.
#[syntax(" nowrap | wrap | wrap-reverse ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "nowrap",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Flexbox,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitFlexWrapStyleValue<'a> {}

/// `-webkit-flex-basis` - alias for `flex-basis`.
#[syntax(" content | <'width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Flexbox,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitFlexBasisStyleValue<'a> {}

/// `-webkit-flex-flow` - alias for `flex-flow`.
#[syntax(" <'flex-direction'> || <'flex-wrap'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "see individual properties",
    applies_to = Elements,
    property_group = Flexbox,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitFlexFlowStyleValue<'a>;

/// `-webkit-flex-grow` - alias for `flex-grow`.
#[syntax(" <number [0,∞]> ")]
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
pub struct WebkitFlexGrowStyleValue<'a>;

/// `-webkit-justify-content` - alias for `justify-content`.
#[syntax(" normal | <content-distribution> | <overflow-position>? [ <content-position> | left | right ] ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Align,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitJustifyContentStyleValue<'a> {}

/// `-webkit-align-items` - alias for `align-items`.
#[syntax(" normal | stretch | <baseline-position> | <overflow-position>? <self-position> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Align,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitAlignItemsStyleValue<'a> {}

/// `-webkit-align-self` - alias for `align-self`.
#[syntax(" auto | <overflow-position>? [ normal | <self-position> ] | stretch | <baseline-position> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Align,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitAlignSelfStyleValue<'a> {}

/// `-webkit-align-content` - alias for `align-content`.
#[syntax(" normal | <baseline-position> | <content-distribution> | <overflow-position>? <content-position> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Align,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitAlignContentStyleValue<'a> {}

// ── Legacy box model ─────────────────────────────────────────────────────────

/// `-webkit-box-orient` - legacy flexbox axis direction.
#[syntax(" horizontal | vertical | inline-axis | block-axis ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "inline-axis",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Flexbox,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitBoxOrientStyleValue<'a> {}

/// `-webkit-box-direction` - legacy flexbox item order direction.
#[syntax(" normal | reverse ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Flexbox,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitBoxDirectionStyleValue<'a> {}

/// `-webkit-box-pack` - legacy flexbox main-axis alignment.
#[syntax(" start | end | center | justify ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "start",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Flexbox,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitBoxPackStyleValue<'a> {}

/// `-webkit-box-align` - legacy flexbox cross-axis alignment.
#[syntax(" stretch | start | end | center | baseline ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "stretch",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Flexbox,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitBoxAlignStyleValue<'a> {}

/// `-webkit-box-flex` - legacy flexbox grow factor.
#[syntax(" <number> ")]
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
pub struct WebkitBoxFlexStyleValue<'a>;

/// `-webkit-box-ordinal-group` - legacy flexbox ordering.
#[syntax(" <integer [1,∞]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "1",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Flexbox,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBoxOrdinalGroupStyleValue<'a>;

/// `-webkit-box-shadow` - alias for `box-shadow`.
#[syntax(" <spread-shadow># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBoxShadowStyleValue<'a>;

/// `-webkit-box-sizing` - alias for `box-sizing`.
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
pub enum WebkitBoxSizingStyleValue<'a> {}

/// `-webkit-box-decoration-break` - alias for `box-decoration-break`.
#[syntax(" clone | slice ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "slice",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Borders,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitBoxDecorationBreakStyleValue<'a> {}

// ── Misc aliases ─────────────────────────────────────────────────────────────

/// `-webkit-user-select` - alias for `user-select`.
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
pub enum WebkitUserSelectStyleValue<'a> {}

/// `-webkit-overflow-scrolling` - non-standard momentum scrolling on iOS.
#[syntax(" auto | touch ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Overflow,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitOverflowScrollingStyleValue<'a> {}

/// `-webkit-print-color-adjust` - alias for `print-color-adjust`.
#[syntax(" economy | exact ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "economy",
    inherits,
    applies_to = Elements,
    animation_type = Discrete,
    property_group = ColorAdjust,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitPrintColorAdjustStyleValue<'a> {}

/// `-webkit-text-decoration` - alias for `text-decoration`.
#[syntax(
	" <'text-decoration-line'> || <'text-decoration-thickness'> || <'text-decoration-style'> || <'text-decoration-color'> "
)]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "see individual properties",
    applies_to = Elements,
    animation_type = Unknown,
    property_group = TextDecor,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTextDecorationStyleValue<'a>;

/// `-webkit-text-decoration-color` - alias for `text-decoration-color`.
#[syntax(" <color> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "currentcolor",
    inherits,
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = TextDecor,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTextDecorationColorStyleValue<'a>;

/// `-webkit-text-decoration-skip-ink` - alias for `text-decoration-skip-ink`.
#[syntax(" auto | none | all ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    inherits,
    applies_to = Elements,
    animation_type = Discrete,
    property_group = TextDecor,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitTextDecorationSkipInkStyleValue<'a> {}

/// `-webkit-column-gap` - alias for `column-gap`.
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
pub enum WebkitColumnGapStyleValue<'a> {}

/// `-webkit-column-count` - alias for `column-count`.
#[syntax(" auto | <integer [1,∞]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    shorthands = _WebkitColumns,
    animation_type = ByComputedValue,
    property_group = Multicol,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitColumnCountStyleValue<'a>;

/// `-webkit-margin-end` - alias for `margin-inline-end`.
#[syntax(" <'margin-top'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitMarginEndStyleValue<'a>;

/// `-webkit-mask-position` - alias for `mask-position`.
#[syntax(" <position># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0% 0%",
    applies_to = Elements,
    animation_type = Unknown,
    property_group = Masking,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    shorthands = _WebkitMask,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitMaskPositionStyleValue<'a>;

/// `-webkit-mask-size` - alias for `mask-size`.
#[syntax(" <bg-size># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    animation_type = Unknown,
    property_group = Masking,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    shorthands = _WebkitMask,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitMaskSizeStyleValue<'a>;

/// `-webkit-transition-property` - alias for `transition-property`.
#[syntax(" none | <single-transition-property># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "all",
    applies_to = Elements,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTransitionPropertyStyleValue<'a>;

/// `-webkit-transition-delay` - alias for `transition-delay`.
#[syntax(" <time># ")]
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
pub struct WebkitTransitionDelayStyleValue<'a>;

/// `-webkit-user-drag` - non-standard drag behaviour.
#[syntax(" auto | none | element ")]
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
pub enum WebkitUserDragStyleValue<'a> {}

/// `-webkit-touch-callout` - non-standard iOS touch callout behaviour.
#[syntax(" default | none ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "default",
    inherits,
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Ui,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitTouchCalloutStyleValue<'a> {}

/// `-webkit-text-fill-color` - non-standard text fill colour.
#[syntax(" <color> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "currentcolor",
    inherits,
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Fonts,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTextFillColorStyleValue<'a>;

/// `-webkit-text-stroke` shorthand for `-webkit-text-stroke-width` and `-webkit-text-stroke-color`.
#[syntax(" <'-webkit-text-stroke-width'> || <'-webkit-text-stroke-color'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits,
    applies_to = Elements,
    animation_type = Unknown,
    longhands = _WebkitTextStrokeWidth|_WebkitTextStrokeColor,
    property_group = Fonts,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[declaration_writes(_WebkitTextStrokeWidth?, _WebkitTextStrokeColor?)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTextStrokeStyleValue<'a>;

/// `-webkit-text-stroke-color` - non-standard text stroke colour.
#[syntax(" <color> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "currentcolor",
    inherits,
    applies_to = Elements,
    animation_type = ByComputedValue,
    shorthands = _WebkitTextStroke,
    property_group = Fonts,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTextStrokeColorStyleValue<'a>;

/// `-webkit-text-stroke-width` - non-standard text stroke width.
#[syntax(" <line-width> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    inherits,
    applies_to = Elements,
    animation_type = Discrete,
    shorthands = _WebkitTextStroke,
    property_group = Fonts,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTextStrokeWidthStyleValue<'a>;

/// `-webkit-background-clip` - alias for `background-clip`.
#[syntax(" <bg-clip># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "border-box",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Backgrounds,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBackgroundClipStyleValue<'a>;

/// `-webkit-background-size` - alias for `background-size`.
#[syntax(" <bg-size># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Backgrounds,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBackgroundSizeStyleValue<'a>;

// `-webkit-animation` - alias for `animation`.
#[syntax(" <single-animation># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Animations,
    longhands = _WebkitAnimationName|_WebkitAnimationDuration|_WebkitAnimationTimingFunction|_WebkitAnimationDelay|_WebkitAnimationIterationCount|_WebkitAnimationDirection|_WebkitAnimationFillMode|_WebkitAnimationPlayState,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitAnimationStyleValue<'a>;

/// `-webkit-perspective` - alias for `perspective`.
#[syntax(" none | <length [0,∞]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = Transforms,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitPerspectiveStyleValue<'a>;

/// `-webkit-clip-path` - alias for `clip-path`.
#[syntax(" none | <clip-source> | [ <basic-shape> || <geometry-box> ] ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Masking,
    computed_value_type = SpecifiedWithAbsoluteUrls,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitClipPathStyleValue<'a> {}

/// `-webkit-animation-direction` - alias for `animation-direction`.
#[syntax(" <'animation-direction'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    shorthands = _WebkitAnimation,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitAnimationDirectionStyleValue<'a>;

/// `-webkit-animation-play-state` - alias for `animation-play-state`.
#[syntax(" <'animation-play-state'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "running",
    applies_to = Elements,
    shorthands = _WebkitAnimation,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitAnimationPlayStateStyleValue<'a>;

/// `-webkit-backdrop-filter` - alias for `backdrop-filter`.
#[syntax(" <'backdrop-filter'> ")]
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
pub struct WebkitBackdropFilterStyleValue<'a>;

/// `-webkit-background-origin` - alias for `background-origin`.
#[syntax(" <'background-origin'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "padding-box",
    applies_to = Elements,
    animation_type = RepeatableList,
    property_group = Backgrounds,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBackgroundOriginStyleValue<'a>;

/// `-webkit-border-bottom-left-radius` - alias for `border-bottom-left-radius`.
#[syntax(" <'border-bottom-left-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    shorthands = _WebkitBorderRadius,
    animation_type = ByComputedValue,
    percentages = BorderBox,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = Bottom|Left,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderBottomLeftRadiusStyleValue<'a>;

/// `-webkit-border-bottom-right-radius` - alias for `border-bottom-right-radius`.
#[syntax(" <'border-bottom-right-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    shorthands = _WebkitBorderRadius,
    animation_type = ByComputedValue,
    percentages = BorderBox,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = Bottom|Right,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderBottomRightRadiusStyleValue<'a>;

/// `-webkit-border-image` - alias for `border-image`.
#[syntax(" <'border-image'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "See individual properties",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    longhands = _WebkitBorderImageSource|_WebkitBorderImageSlice|_WebkitBorderImageWidth|_WebkitBorderImageOutset|_WebkitBorderImageRepeat,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderImageStyleValue<'a>;

/// `-webkit-border-radius` - alias for `border-radius`.
#[syntax(" <'border-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    animation_type = Unknown,
    percentages = Unknown,
    property_group = Borders,
    longhands = _WebkitBorderTopLeftRadius|_WebkitBorderTopRightRadius|_WebkitBorderBottomRightRadius|_WebkitBorderBottomLeftRadius,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderRadiusStyleValue<'a>;

/// `-webkit-border-top-left-radius` - alias for `border-top-left-radius`.
#[syntax(" <'border-top-left-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    shorthands = _WebkitBorderRadius,
    animation_type = ByComputedValue,
    percentages = BorderBox,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = Top|Left,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderTopLeftRadiusStyleValue<'a>;

/// `-webkit-border-top-right-radius` - alias for `border-top-right-radius`.
#[syntax(" <'border-top-right-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    shorthands = _WebkitBorderRadius,
    animation_type = ByComputedValue,
    percentages = BorderBox,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = Top|Right,
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderTopRightRadiusStyleValue<'a>;

/// `-webkit-column-rule` - alias for `column-rule`.
#[syntax(" <'column-rule'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    animation_type = Unknown,
    percentages = Unknown,
    property_group = Gaps,
    longhands = _WebkitColumnRuleWidth|_WebkitColumnRuleStyle|_WebkitColumnRuleColor,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitColumnRuleStyleValue<'a>;

/// `-webkit-column-width` - alias for `column-width`.
#[syntax(" <'column-width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Unknown,
    shorthands = _WebkitColumns,
    animation_type = ByComputedValue,
    property_group = Multicol,
    computed_value_type = SpecifiedKeywordPlusAbsoluteLength,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitColumnWidthStyleValue<'a>;

/// `-webkit-columns` - alias for `columns`.
#[syntax(" <'columns'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    animation_type = Unknown,
    percentages = Unknown,
    property_group = Multicol,
    longhands = _WebkitColumnWidth|_WebkitColumnCount,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitColumnsStyleValue<'a>;

/// `-webkit-flex-shrink` - alias for `flex-shrink`.
#[syntax(" <'flex-shrink'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "1",
    applies_to = Unknown,
    animation_type = Number,
    property_group = Flexbox,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitFlexShrinkStyleValue<'a>;

/// `-webkit-font-feature-settings` - alias for `font-feature-settings`.
#[syntax(" <'font-feature-settings'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "normal",
    inherits,
    applies_to = Elements|Text,
    animation_type = Discrete,
    property_group = Fonts,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitFontFeatureSettingsStyleValue<'a>;

/// `-webkit-font-variant-ligatures` - alias for `font-variant-ligatures`.
#[syntax(" <'font-variant-ligatures'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "normal",
    inherits,
    applies_to = Elements|Text,
    animation_type = Discrete,
    property_group = Fonts,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitFontVariantLigaturesStyleValue<'a>;

/// `-webkit-hyphens` - alias for `hyphens`.
#[syntax(" <'hyphens'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "manual",
    inherits,
    applies_to = Text,
    animation_type = Discrete,
    property_group = Text,
    computed_value_type = Unknown,
    canonical_order = "n/a",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitHyphensStyleValue<'a>;

/// `-webkit-margin-start` - alias for `margin-inline-start`.
#[syntax(" <'margin-inline-start'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Margin,
    box_side = InlineStart,
    box_portion = Margin,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitMarginStartStyleValue<'a>;

/// `-webkit-mask-image` - alias for `mask-image`.
#[syntax(" <'mask-image'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Masking,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    shorthands = _WebkitMask,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitMaskImageStyleValue<'a>;

/// `-webkit-mask-repeat` - alias for `mask-repeat`.
#[syntax(" <'mask-repeat'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "repeat",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Masking,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    shorthands = _WebkitMask,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitMaskRepeatStyleValue<'a>;

/// `-webkit-padding-after` - alias for `padding-block-end`.
#[syntax(" <'padding-block-end'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Padding,
    box_side = BlockEnd,
    box_portion = Padding,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitPaddingAfterStyleValue<'a>;

/// `-webkit-padding-start` - alias for `padding-inline-start`.
#[syntax(" <'padding-inline-start'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = Unknown,
    property_group = Logical,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = Padding,
    box_side = InlineStart,
    box_portion = Padding,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitPaddingStartStyleValue<'a>;

/// `-webkit-perspective-origin` - alias for `perspective-origin`.
#[syntax(" <'perspective-origin'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "50% 50%",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = ReferenceBox,
    property_group = Transforms,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitPerspectiveOriginStyleValue<'a>;

/// `-webkit-text-decoration-style` - alias for `text-decoration-style`.
#[syntax(" <'text-decoration-style'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "solid",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = TextDecor,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTextDecorationStyleStyleValue<'a>;

/// `-webkit-transform-style` - alias for `transform-style`.
#[syntax(" <'transform-style'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "flat",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Transforms,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitTransformStyleStyleValue<'a>;

/// Represents the value of `-webkit-mask`.
#[syntax(" [ <mask-reference> || [ <position> [ / <bg-size> ]? ] || <repeat-style> || <webkit-mask-box>{1,2} ]# ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Masking,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    longhands = _WebkitMaskImage|_WebkitMaskRepeat|_WebkitMaskPosition|_WebkitMaskSize|_WebkitMaskOrigin|_WebkitMaskClip|_WebkitMaskAttachment,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitMaskStyleValue<'a>;

/// Represents the value of `-webkit-mask-composite`.
#[syntax(
	" [ clear | copy | source-over | source-in | source-out | source-atop | destination-over | destination-in | destination-out | destination-atop | xor ]# "
)]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "source-over",
    applies_to = Elements,
    property_group = Masking,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitMaskCompositeStyleValue<'a>;

/// Represents the value of `-webkit-column-span`.
#[syntax(" none | all | <integer [1,1]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Multicol,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitColumnSpanStyleValue<'a> {}

/// Represents the value of `-webkit-column-break-inside`.
#[syntax(" auto | avoid ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    property_group = Break,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitColumnBreakInsideStyleValue<'a> {}

/// Represents the value of `-webkit-text-decoration-skip`.
#[syntax(
	" none | [ objects || ink || [ spaces | [ leading-spaces || trailing-spaces ] ] || edges || box-decoration ] "
)]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "objects",
    inherits,
    applies_to = Elements,
    property_group = TextDecor,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitTextDecorationSkipStyleValue<'a> {}

/// Represents the value of `-webkit-box-reflect`.
#[syntax(" none | [ above | below | right | left ] <length-percentage>? <'border-image'>? ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Box,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitBoxReflectStyleValue<'a> {}

/// Represents the value of `-webkit-line-break`.
#[syntax(" auto | loose | normal | strict | anywhere | after-white-space ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    inherits,
    applies_to = Elements,
    property_group = Text,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitLineBreakStyleValue<'a> {}

/// Represents the value of `-webkit-user-modify`.
#[syntax(" read-only | read-write | read-write-plaintext-only ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "read-only",
    inherits,
    applies_to = Elements,
    property_group = Ui,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitUserModifyStyleValue<'a> {}

/// Represents the value of `-webkit-app-region`.
#[syntax(" drag | no-drag ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "no-drag",
    applies_to = Elements,
    property_group = Ui,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitAppRegionStyleValue<'a> {}

/// Represents the value of `-webkit-border-horizontal-spacing`.
#[syntax(" <length [0,∞]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    inherits,
    applies_to = Elements,
    property_group = Tables,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderHorizontalSpacingStyleValue<'a>;

/// Represents the value of `-webkit-border-vertical-spacing`.
#[syntax(" <length [0,∞]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    inherits,
    applies_to = Elements,
    property_group = Tables,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderVerticalSpacingStyleValue<'a>;

/// Represents the value of `-webkit-margin-top-collapse`.
#[syntax(" collapse | separate | discard ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "collapse",
    applies_to = Elements,
    property_group = Box,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitMarginTopCollapseStyleValue<'a> {}

/// `-webkit-border-image-outset` - alias for `border-image-outset`.
#[syntax(" <'border-image-outset'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    shorthands = _WebkitBorderImage,
    property_group = Borders,
    computed_value_type = AbsoluteLength,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderImageOutsetStyleValue<'a>;

/// `-webkit-border-image-repeat` - alias for `border-image-repeat`.
#[syntax(" <'border-image-repeat'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "stretch",
    applies_to = Unknown,
    animation_type = Discrete,
    shorthands = _WebkitBorderImage,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderImageRepeatStyleValue<'a>;

/// `-webkit-border-image-slice` - alias for `border-image-slice`.
#[syntax(" <'border-image-slice'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "100%",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = BorderImageArea,
    shorthands = _WebkitBorderImage,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderImageSliceStyleValue<'a>;

/// `-webkit-border-image-source` - alias for `border-image-source`.
#[syntax(" <'border-image-source'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    shorthands = _WebkitBorderImage,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderImageSourceStyleValue<'a>;

/// `-webkit-border-image-width` - alias for `border-image-width`.
#[syntax(" <'border-image-width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "1",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = BorderImageArea,
    shorthands = _WebkitBorderImage,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitBorderImageWidthStyleValue<'a>;

/// `-webkit-column-rule-color` - alias for `column-rule-color`.
#[syntax(" <'column-rule-color'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "currentcolor",
    applies_to = Unknown,
    animation_type = RepeatableList,
    shorthands = _WebkitColumnRule,
    property_group = Gaps,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitColumnRuleColorStyleValue<'a>;

/// `-webkit-column-rule-style` - alias for `column-rule-style`.
#[syntax(" <'column-rule-style'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    shorthands = _WebkitColumnRule,
    property_group = Gaps,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitColumnRuleStyleStyleValue<'a>;

/// `-webkit-column-rule-width` - alias for `column-rule-width`.
#[syntax(" <'column-rule-width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "medium",
    applies_to = Unknown,
    animation_type = RepeatableList,
    shorthands = _WebkitColumnRule,
    property_group = Gaps,
    computed_value_type = ListOfAbsoluteLengths,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitColumnRuleWidthStyleValue<'a>;

/// Represents the value of `-webkit-mask-attachment`.
#[syntax(" <'background-attachment'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "scroll",
    applies_to = Elements,
    animation_type = Discrete,
    shorthands = _WebkitMask,
    property_group = Masking,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitMaskAttachmentStyleValue<'a>;

/// Represents the value of `-webkit-mask-clip`, including legacy box keywords.
#[syntax(" [ border | padding | content | border-box | padding-box | content-box | text ]# ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "border",
    applies_to = Unknown,
    animation_type = Discrete,
    shorthands = _WebkitMask,
    property_group = Masking,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitMaskClipStyleValue<'a>;

/// Represents the value of `-webkit-mask-origin`, including legacy box keywords.
#[syntax(" [ border | padding | content | border-box | padding-box | content-box ]# ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "padding",
    applies_to = Unknown,
    animation_type = Discrete,
    shorthands = _WebkitMask,
    property_group = Masking,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitMaskOriginStyleValue<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn mask_longhands_accept_legacy_box_keywords() {
		assert_parse!(CssAtomSet::ATOMS, WebkitMaskClipStyleValue, "border, text, padding-box");
		assert_parse!(CssAtomSet::ATOMS, WebkitMaskOriginStyleValue, "padding, content-box");
		assert_peek_false!(CssAtomSet::ATOMS, WebkitMaskOriginStyleValue, "text");
		assert_peek_false!(CssAtomSet::ATOMS, WebkitMaskClipStyleValue, "unknown-box");
	}

	#[test]
	fn masks_accept_legacy_boxes_and_positioned_images() {
		assert_parse!(
			CssAtomSet::ATOMS,
			WebkitMaskStyleValue,
			"url(mask.svg) center/cover no-repeat padding-box border"
		);
		assert_parse_error!(CssAtomSet::ATOMS, WebkitMaskStyleValue, "url(mask.svg) center/cover border unknown-box");
	}

	#[test]
	fn mask_composite_uses_legacy_operators() {
		assert_parse!(CssAtomSet::ATOMS, WebkitMaskCompositeStyleValue, "source-in, xor");
		assert_peek_false!(CssAtomSet::ATOMS, WebkitMaskCompositeStyleValue, "add");
		assert_peek_false!(CssAtomSet::ATOMS, WebkitMaskCompositeStyleValue, "exclude");
	}

	#[test]
	fn column_span_accepts_legacy_single_column() {
		assert_parse!(CssAtomSet::ATOMS, WebkitColumnSpanStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, WebkitColumnSpanStyleValue, "all");
		assert_peek_false!(CssAtomSet::ATOMS, WebkitColumnSpanStyleValue, "0");
		assert_peek_false!(CssAtomSet::ATOMS, WebkitColumnSpanStyleValue, "2");
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
		assert_peek_false!(CssAtomSet::ATOMS, WebkitFilterStyleValue, "invalid");
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
	fn test_flex_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitFlexStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitFlexStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, WebkitFlexStyleValue, "1 1 auto");
	}

	#[test]
	fn test_flex_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitFlexStyleValue, "invalid");
	}

	#[test]
	fn test_order_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitOrderStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, WebkitOrderStyleValue, "-1");
		assert_parse!(CssAtomSet::ATOMS, WebkitOrderStyleValue, "5");
	}

	#[test]
	fn test_order_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitOrderStyleValue, "none");
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
		assert_peek_false!(CssAtomSet::ATOMS, WebkitAppearanceStyleValue, "invalid");
	}

	#[test]
	fn test_transform_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitTransformStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransformStyleValue, "rotate(45deg)");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransformStyleValue, "scale(-1, 1)");
	}

	#[test]
	fn test_transform_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitTransformStyleValue, "invalid");
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
		assert_peek_false!(CssAtomSet::ATOMS, WebkitFontSmoothingStyleValue, "invalid");
	}

	#[test]
	fn test_text_size_adjust_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitTextSizeAdjustStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, WebkitTextSizeAdjustStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitTextSizeAdjustStyleValue, "100%");
	}

	#[test]
	fn test_text_size_adjust_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitTextSizeAdjustStyleValue, "invalid");
	}

	#[test]
	fn test_animation_delay_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationDelayStyleValue, "0s");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationDelayStyleValue, "0.5s, 1s");
	}

	#[test]
	fn test_animation_delay_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitAnimationDelayStyleValue, "invalid");
	}

	#[test]
	fn test_animation_duration_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationDurationStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationDurationStyleValue, "0.3s");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationDurationStyleValue, "1s, 2s");
	}

	#[test]
	fn test_animation_duration_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitAnimationDurationStyleValue, "invalid");
	}

	#[test]
	fn test_animation_fill_mode_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationFillModeStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationFillModeStyleValue, "forwards");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationFillModeStyleValue, "both");
	}

	#[test]
	fn test_animation_fill_mode_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitAnimationFillModeStyleValue, "invalid");
	}

	#[test]
	fn test_animation_iteration_count_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationIterationCountStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationIterationCountStyleValue, "infinite");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationIterationCountStyleValue, "3");
	}

	#[test]
	fn test_animation_iteration_count_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitAnimationIterationCountStyleValue, "invalid");
	}

	#[test]
	fn test_animation_name_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitAnimationNameStyleValue, "my-animation");
	}

	#[test]
	fn test_animation_name_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitAnimationNameStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, WebkitAnimationNameStyleValue, "1px");
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
		assert_peek_false!(CssAtomSet::ATOMS, WebkitAnimationTimingFunctionStyleValue, "invalid");
	}

	#[test]
	fn test_backface_visibility_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitBackfaceVisibilityStyleValue, "visible");
		assert_parse!(CssAtomSet::ATOMS, WebkitBackfaceVisibilityStyleValue, "hidden");
	}

	#[test]
	fn test_backface_visibility_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitBackfaceVisibilityStyleValue, "invalid");
	}

	#[test]
	fn test_tap_highlight_color_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitTapHighlightColorStyleValue, "transparent");
		assert_parse!(CssAtomSet::ATOMS, WebkitTapHighlightColorStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, WebkitTapHighlightColorStyleValue, "rgba(0, 0, 0, 0)");
	}

	#[test]
	fn test_tap_highlight_color_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitTapHighlightColorStyleValue, "invalid");
	}

	#[test]
	fn test_transition_duration_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionDurationStyleValue, "0s");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionDurationStyleValue, "0.3s");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionDurationStyleValue, "1s, 200ms");
	}

	#[test]
	fn test_transition_duration_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitTransitionDurationStyleValue, "invalid");
	}

	#[test]
	fn test_transition_timing_function_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionTimingFunctionStyleValue, "ease");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionTimingFunctionStyleValue, "linear");
		assert_parse!(CssAtomSet::ATOMS, WebkitTransitionTimingFunctionStyleValue, "cubic-bezier(0.4, 0, 0.2, 1)");
	}

	#[test]
	fn test_transition_timing_function_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitTransitionTimingFunctionStyleValue, "invalid");
	}

	#[test]
	fn test_webkit_perspective_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitPerspectiveStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WebkitPerspectiveStyleValue, "800px");
	}

	#[test]
	fn test_webkit_perspective_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitPerspectiveStyleValue, "invalid");
	}

	#[test]
	fn test_text_stroke_parses() {
		assert_parse!(CssAtomSet::ATOMS, WebkitTextStrokeStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, WebkitTextStrokeStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, WebkitTextStrokeStyleValue, "1px red");
		assert_parse!(CssAtomSet::ATOMS, WebkitTextStrokeStyleValue, "red 1px");
	}

	#[test]
	fn test_text_stroke_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, WebkitTextStrokeStyleValue, "");
	}
}
