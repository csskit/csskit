//! Microsoft-prefixed CSS property value types.
//!
//! Non-standard aliases for standardised properties, kept for compatibility
//! with legacy stylesheets targeting IE/Edge Legacy.

use super::prelude::*;
use crate::{FilterValueList, Length, PositionOne, PositionTwo};
use css_parse::{Cursor, Parse, Parser, Result as ParseResult, T};

/// `-ms-overflow-style` — IE/Edge scrollbar display behaviour.
#[syntax(" auto | none | scrollbar | -ms-autohiding-scrollbar ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    inherits,
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Scrollbars,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MsOverflowStyleStyleValue {}

/// `-ms-interpolation-mode` - IE image scaling algorithm.
#[syntax(" nearest-neighbor | bicubic ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "undefined",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Images,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MsInterpolationModeStyleValue {}

/// `-ms-box-sizing` — alias for `box-sizing`.
#[syntax(" content-box | border-box ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "content-box",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Sizing,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MsBoxSizingStyleValue {}

/// `-ms-touch-action` — alias for `touch-action`.
#[syntax(
	" auto | none | [ [ pan-x | pan-left | pan-right ] || [ pan-y | pan-up | pan-down ] || pinch-zoom ] | manipulation "
)]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = PointerAnimations,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MsTouchActionStyleValue {}

/// `-ms-user-select` — alias for `user-select`.
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
pub enum MsUserSelectStyleValue {}

/// `-ms-text-size-adjust` — alias for `text-size-adjust`.
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
pub struct MsTextSizeAdjustStyleValue;

/// `-ms-flex` — IE10 alias for `flex`.
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
pub struct MsFlexStyleValue;

/// `-ms-flex-order` — IE10 alias for `order`.
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
pub struct MsFlexOrderStyleValue;

/// `-ms-flex-direction` — IE10 alias for `flex-direction`.
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
pub enum MsFlexDirectionStyleValue {}

/// `-ms-flex-wrap` — IE10 alias for `flex-wrap`.
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
pub enum MsFlexWrapStyleValue {}

/// `-ms-flex-flow` — IE10 alias for `flex-flow`.
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
pub struct MsFlexFlowStyleValue;

/// `-ms-flex-positive` — IE10 alias for `flex-grow`.
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
pub struct MsFlexPositiveStyleValue;

/// `-ms-flex-negative` — IE10 alias for `flex-shrink`.
#[syntax(" <number [0,∞]> ")]
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
pub struct MsFlexNegativeStyleValue;

/// `-ms-flex-preferred-size` — IE10 alias for `flex-basis`.
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
pub enum MsFlexPreferredSizeStyleValue {}

/// `-ms-flex-pack` — IE10 alias for `justify-content`.
///
/// Uses legacy IE10 value names: `start`=`flex-start`, `end`=`flex-end`,
/// `center`=`center`, `justify`=`space-between`, `distribute`=`space-around`.
#[syntax(" start | end | center | justify | distribute ")]
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
pub enum MsFlexPackStyleValue {}

/// `-ms-flex-align` — IE10 alias for `align-items`.
///
/// Uses legacy IE10 value names: `start`=`flex-start`, `end`=`flex-end`,
/// `center`=`center`, `stretch`=`stretch`, `baseline`=`baseline`.
#[syntax(" start | end | center | stretch | baseline ")]
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
pub enum MsFlexAlignStyleValue {}

/// `-ms-flex-item-align` — IE10 alias for `align-self`.
///
/// Uses legacy IE10 value names: `auto`, `start`, `end`, `center`,
/// `stretch`, `baseline`.
#[syntax(" auto | start | end | center | stretch | baseline ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = Flexbox,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MsFlexItemAlignStyleValue {}

/// `-ms-flex-line-pack` — IE10 alias for `align-content`.
///
/// Uses legacy IE10 value names: `start`, `end`, `center`, `justify`,
/// `distribute`, `stretch`.
#[syntax(" start | end | center | justify | distribute | stretch ")]
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
pub enum MsFlexLinePackStyleValue {}

/// `-ms-transform` — IE9 alias for `transform`.
#[syntax(" none | <transform-list> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    animation_type = Unknown,
    property_group = Transforms,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsTransformStyleValue<'a>;

/// Represents the style value for `-ms-transform-origin`.
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
pub enum MsTransformOriginStyleValue {}

impl<'a> Parse<'a> for MsTransformOriginStyleValue {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParseResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let first = p.parse::<PositionOne>()?;
		let Some(second) = p.parse_if_peek::<PositionOne>()? else { return Ok(Self::PositionOne(first)) };
		let two = PositionTwo::from_two(p, first, second)?;
		Ok(Self::PositionTwo(two, p.parse_if_peek::<Length>()?))
	}
}

/// `-ms-filter` — IE8/IE9 `filter` alias.
///
/// IE8 used a quoted string with proprietary `progid:DXImageTransform` syntax.
/// IE9+ accepted the same CSS filter functions as the standard `filter` property.
///
/// The grammar is:
///
/// ```text,ignore
/// none | <string> | <filter-value-list>
/// ```
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    animation_type = Unknown,
    property_group = FilterEffects,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MsFilterStyleValue<'a> {
	None(#[atom(CssAtomSet::None)] T![Ident]),
	String(T![String]),
	FilterValueList(FilterValueList<'a>),
}

/// `-ms-word-break` — IE alias for `word-break`.
#[syntax(" normal | break-all | keep-all | manual | auto-phrase | break-word ")]
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
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MsWordBreakStyleValue {}

/// `-ms-transition` — IE10 alias for `transition`.
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
pub struct MsTransitionStyleValue<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_ms_interpolation_mode_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsInterpolationModeStyleValue, "nearest-neighbor");
		assert_parse!(CssAtomSet::ATOMS, MsInterpolationModeStyleValue, "bicubic");
	}

	#[test]
	fn test_ms_interpolation_mode_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsInterpolationModeStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, MsInterpolationModeStyleValue, "invalid");
	}

	#[test]
	fn test_ms_box_sizing_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsBoxSizingStyleValue, "content-box");
		assert_parse!(CssAtomSet::ATOMS, MsBoxSizingStyleValue, "border-box");
	}

	#[test]
	fn test_ms_box_sizing_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsBoxSizingStyleValue, "invalid");
	}

	#[test]
	fn test_ms_overflow_style_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsOverflowStyleStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MsOverflowStyleStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MsOverflowStyleStyleValue, "scrollbar");
		assert_parse!(CssAtomSet::ATOMS, MsOverflowStyleStyleValue, "-ms-autohiding-scrollbar");
	}

	#[test]
	fn test_ms_overflow_style_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsOverflowStyleStyleValue, "invalid");
	}

	#[test]
	fn test_ms_touch_action_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsTouchActionStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MsTouchActionStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MsTouchActionStyleValue, "manipulation");
		assert_parse!(CssAtomSet::ATOMS, MsTouchActionStyleValue, "pan-y");
		assert_parse!(CssAtomSet::ATOMS, MsTouchActionStyleValue, "pan-x pan-y");
	}

	#[test]
	fn test_ms_touch_action_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsTouchActionStyleValue, "invalid");
	}

	#[test]
	fn test_ms_user_select_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsUserSelectStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MsUserSelectStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MsUserSelectStyleValue, "all");
	}

	#[test]
	fn test_ms_user_select_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsUserSelectStyleValue, "invalid");
	}

	#[test]
	fn test_ms_text_size_adjust_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsTextSizeAdjustStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MsTextSizeAdjustStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MsTextSizeAdjustStyleValue, "100%");
	}

	#[test]
	fn test_ms_text_size_adjust_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsTextSizeAdjustStyleValue, "invalid");
	}

	#[test]
	fn test_ms_flex_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsFlexStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MsFlexStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, MsFlexStyleValue, "0 0 auto");
		assert_parse!(CssAtomSet::ATOMS, MsFlexStyleValue, "0 0 100%");
	}

	#[test]
	fn test_ms_flex_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsFlexStyleValue, "invalid");
	}

	#[test]
	fn test_ms_flex_order_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsFlexOrderStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, MsFlexOrderStyleValue, "-1");
		assert_parse!(CssAtomSet::ATOMS, MsFlexOrderStyleValue, "8");
	}

	#[test]
	fn test_ms_flex_direction_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsFlexDirectionStyleValue, "row");
		assert_parse!(CssAtomSet::ATOMS, MsFlexDirectionStyleValue, "column");
		assert_parse!(CssAtomSet::ATOMS, MsFlexDirectionStyleValue, "row-reverse");
		assert_parse!(CssAtomSet::ATOMS, MsFlexDirectionStyleValue, "column-reverse");
	}

	#[test]
	fn test_ms_flex_direction_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsFlexDirectionStyleValue, "invalid");
	}

	#[test]
	fn test_ms_flex_wrap_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsFlexWrapStyleValue, "nowrap");
		assert_parse!(CssAtomSet::ATOMS, MsFlexWrapStyleValue, "wrap");
		assert_parse!(CssAtomSet::ATOMS, MsFlexWrapStyleValue, "wrap-reverse");
	}

	#[test]
	fn test_ms_flex_wrap_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsFlexWrapStyleValue, "invalid");
	}

	#[test]
	fn test_ms_flex_pack_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsFlexPackStyleValue, "start");
		assert_parse!(CssAtomSet::ATOMS, MsFlexPackStyleValue, "end");
		assert_parse!(CssAtomSet::ATOMS, MsFlexPackStyleValue, "center");
		assert_parse!(CssAtomSet::ATOMS, MsFlexPackStyleValue, "justify");
		assert_parse!(CssAtomSet::ATOMS, MsFlexPackStyleValue, "distribute");
	}

	#[test]
	fn test_ms_flex_pack_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsFlexPackStyleValue, "invalid");
	}

	#[test]
	fn test_ms_flex_align_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsFlexAlignStyleValue, "start");
		assert_parse!(CssAtomSet::ATOMS, MsFlexAlignStyleValue, "end");
		assert_parse!(CssAtomSet::ATOMS, MsFlexAlignStyleValue, "center");
		assert_parse!(CssAtomSet::ATOMS, MsFlexAlignStyleValue, "stretch");
		assert_parse!(CssAtomSet::ATOMS, MsFlexAlignStyleValue, "baseline");
	}

	#[test]
	fn test_ms_flex_align_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsFlexAlignStyleValue, "invalid");
	}

	#[test]
	fn test_ms_flex_item_align_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsFlexItemAlignStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MsFlexItemAlignStyleValue, "start");
		assert_parse!(CssAtomSet::ATOMS, MsFlexItemAlignStyleValue, "end");
		assert_parse!(CssAtomSet::ATOMS, MsFlexItemAlignStyleValue, "center");
		assert_parse!(CssAtomSet::ATOMS, MsFlexItemAlignStyleValue, "stretch");
		assert_parse!(CssAtomSet::ATOMS, MsFlexItemAlignStyleValue, "baseline");
	}

	#[test]
	fn test_ms_flex_item_align_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsFlexItemAlignStyleValue, "invalid");
	}

	#[test]
	fn test_ms_flex_line_pack_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsFlexLinePackStyleValue, "start");
		assert_parse!(CssAtomSet::ATOMS, MsFlexLinePackStyleValue, "end");
		assert_parse!(CssAtomSet::ATOMS, MsFlexLinePackStyleValue, "center");
		assert_parse!(CssAtomSet::ATOMS, MsFlexLinePackStyleValue, "justify");
		assert_parse!(CssAtomSet::ATOMS, MsFlexLinePackStyleValue, "distribute");
		assert_parse!(CssAtomSet::ATOMS, MsFlexLinePackStyleValue, "stretch");
	}

	#[test]
	fn test_ms_flex_line_pack_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsFlexLinePackStyleValue, "invalid");
	}

	#[test]
	fn test_ms_transform_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsTransformStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MsTransformStyleValue, "rotate(45deg)");
		assert_parse!(CssAtomSet::ATOMS, MsTransformStyleValue, "scale(-1, 1)");
	}

	#[test]
	fn test_ms_transform_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsTransformStyleValue, "invalid");
	}

	#[test]
	fn test_ms_filter_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsFilterStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MsFilterStyleValue, "blur(4px)");
		assert_parse!(
			CssAtomSet::ATOMS,
			MsFilterStyleValue,
			"\"progid:DXImageTransform.Microsoft.blur(pixelradius=2)\""
		);
	}

	#[test]
	fn test_ms_filter_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsFilterStyleValue, "invalid");
	}

	#[test]
	fn test_ms_word_break_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsWordBreakStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, MsWordBreakStyleValue, "break-all");
		assert_parse!(CssAtomSet::ATOMS, MsWordBreakStyleValue, "keep-all");
	}

	#[test]
	fn test_ms_word_break_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsWordBreakStyleValue, "invalid");
	}

	#[test]
	fn test_ms_transition_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsTransitionStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MsTransitionStyleValue, "all 0.3s ease");
	}

	#[test]
	fn test_ms_transition_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MsTransitionStyleValue, "invalid!!!!");
	}
}
