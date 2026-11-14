//! Microsoft-prefixed CSS property value types.
//!
//! Non-standard aliases for standardised properties, kept for compatibility
//! with legacy stylesheets targeting IE/Edge Legacy.

use super::prelude::*;
use crate::{CalcableValue, FilterValueList, Length, PositionOne, PositionTwo};
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
pub enum MsOverflowStyleStyleValue<'a> {}

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
pub enum MsInterpolationModeStyleValue<'a> {}

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
pub enum MsBoxSizingStyleValue<'a> {}

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
pub enum MsTouchActionStyleValue<'a> {}

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
pub enum MsUserSelectStyleValue<'a> {}

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
pub struct MsTextSizeAdjustStyleValue<'a>;

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
pub struct MsFlexStyleValue<'a>;

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
pub struct MsFlexOrderStyleValue<'a>;

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
pub enum MsFlexDirectionStyleValue<'a> {}

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
pub enum MsFlexWrapStyleValue<'a> {}

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
pub struct MsFlexFlowStyleValue<'a>;

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
pub struct MsFlexPositiveStyleValue<'a>;

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
pub struct MsFlexNegativeStyleValue<'a>;

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
pub enum MsFlexPreferredSizeStyleValue<'a> {}

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
pub enum MsFlexPackStyleValue<'a> {}

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
pub enum MsFlexAlignStyleValue<'a> {}

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
pub enum MsFlexItemAlignStyleValue<'a> {}

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
pub enum MsFlexLinePackStyleValue<'a> {}

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
pub enum MsTransformOriginStyleValue<'a> {}

impl<'a> Parse<'a> for MsTransformOriginStyleValue<'a> {
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
#[node]
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
pub enum MsWordBreakStyleValue<'a> {}

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
    longhands = _MsTransitionProperty|_MsTransitionDuration|_MsTransitionTimingFunction|_MsTransitionDelay,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsTransitionStyleValue<'a>;

/// `-ms-animation` is an alias for `animation`.
#[syntax(" <'animation'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "see individual properties",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    longhands = _MsAnimationName|_MsAnimationDuration|_MsAnimationTimingFunction|_MsAnimationDelay|_MsAnimationIterationCount|_MsAnimationDirection|_MsAnimationFillMode|_MsAnimationPlayState,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsAnimationStyleValue<'a>;

/// `-ms-backface-visibility` is an alias for `backface-visibility`.
#[syntax(" <'backface-visibility'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "visible",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Transforms,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsBackfaceVisibilityStyleValue<'a>;

/// `-ms-border-bottom-left-radius` is an alias for `border-bottom-left-radius`.
#[syntax(" <'border-bottom-left-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    percentages = BorderBox,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = Bottom|Left,
    box_portion = Border,
    shorthands = _MsBorderRadius,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsBorderBottomLeftRadiusStyleValue<'a>;

/// `-ms-border-bottom-right-radius` is an alias for `border-bottom-right-radius`.
#[syntax(" <'border-bottom-right-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    percentages = BorderBox,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    logical_property_group = BorderRadius,
    box_side = Bottom|Right,
    box_portion = Border,
    shorthands = _MsBorderRadius,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsBorderBottomRightRadiusStyleValue<'a>;

/// `-ms-border-radius` is an alias for `border-radius`.
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
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
    longhands = _MsBorderTopLeftRadius|_MsBorderTopRightRadius|_MsBorderBottomRightRadius|_MsBorderBottomLeftRadius,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsBorderRadiusStyleValue<'a>;

/// `-ms-font-feature-settings` is an alias for `font-feature-settings`.
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
pub struct MsFontFeatureSettingsStyleValue<'a>;

/// `-ms-hyphens` is an alias for `hyphens`.
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
pub struct MsHyphensStyleValue<'a>;

/// `-ms-perspective` is an alias for `perspective`.
#[syntax(" <'perspective'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Transforms,
    computed_value_type = AbsoluteLengthOrNone,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsPerspectiveStyleValue<'a>;

/// `-ms-perspective-origin` is an alias for `perspective-origin`.
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
pub struct MsPerspectiveOriginStyleValue<'a>;

/// `-ms-transform-style` is an alias for `transform-style`.
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
pub struct MsTransformStyleStyleValue<'a>;

/// `-ms-transition-property` is an alias for `transition-property`.
#[syntax(" <'transition-property'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "all",
    applies_to = Elements,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    shorthands = _MsTransition,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsTransitionPropertyStyleValue<'a>;

/// `-ms-word-wrap` is an alias for `overflow-wrap`.
#[syntax(" <'overflow-wrap'> ")]
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
pub struct MsWordWrapStyleValue<'a>;

/// Represents the value of `-ms-grid-column`.
#[syntax(" <integer [1,∞]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "1",
    applies_to = Elements,
    property_group = Grid,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsGridColumnStyleValue<'a>;

/// Represents the value of `-ms-grid-row`.
#[syntax(" <integer [1,∞]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "1",
    applies_to = Elements,
    property_group = Grid,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsGridRowStyleValue<'a>;

/// Represents the value of `-ms-grid-column-span`.
#[syntax(" <integer [1,∞]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "1",
    applies_to = Elements,
    property_group = Grid,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsGridColumnSpanStyleValue<'a>;

/// Represents the value of `-ms-grid-row-align`.
#[syntax(" start | end | center | stretch ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "stretch",
    applies_to = Elements,
    property_group = Grid,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MsGridRowAlignStyleValue<'a> {}

/// Represents the value of `-ms-grid-columns`.
#[syntax(" none | <ms-track-size>+ ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Grid,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsGridColumnsStyleValue<'a>;

/// Represents the value of `-ms-grid-rows`.
#[syntax(" none | <ms-track-size>+ ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    property_group = Grid,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsGridRowsStyleValue<'a>;

/// Represents the value of `-ms-scroll-chaining`.
#[syntax(" chained | none ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "chained",
    applies_to = Elements,
    property_group = Overscroll,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MsScrollChainingStyleValue<'a> {}

/// `-ms-animation-delay` is a legacy alias for `animation-delay`.
#[syntax(" <time># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0s",
    applies_to = Elements,
    shorthands = _MsAnimation,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsAnimationDelayStyleValue<'a>;

/// `-ms-animation-direction` is an alias for `animation-direction`.
#[syntax(" <'animation-direction'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "normal",
    applies_to = Elements,
    shorthands = _MsAnimation,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsAnimationDirectionStyleValue<'a>;

/// `-ms-animation-duration` is an alias for `animation-duration`.
#[syntax(" <'animation-duration'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    shorthands = _MsAnimation,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsAnimationDurationStyleValue<'a>;

/// `-ms-animation-fill-mode` is an alias for `animation-fill-mode`.
#[syntax(" <'animation-fill-mode'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    shorthands = _MsAnimation,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsAnimationFillModeStyleValue<'a>;

/// `-ms-animation-iteration-count` is an alias for `animation-iteration-count`.
#[syntax(" <'animation-iteration-count'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "1",
    applies_to = Elements,
    shorthands = _MsAnimation,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsAnimationIterationCountStyleValue<'a>;

/// `-ms-animation-name` is an alias for `animation-name`.
#[syntax(" <'animation-name'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    shorthands = _MsAnimation,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsAnimationNameStyleValue<'a>;

/// `-ms-animation-play-state` is an alias for `animation-play-state`.
#[syntax(" <'animation-play-state'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "running",
    applies_to = Elements,
    shorthands = _MsAnimation,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsAnimationPlayStateStyleValue<'a>;

/// `-ms-animation-timing-function` is an alias for `animation-timing-function`.
#[syntax(" <'animation-timing-function'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "ease",
    applies_to = Elements,
    shorthands = _MsAnimation,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsAnimationTimingFunctionStyleValue<'a>;

/// `-ms-border-top-left-radius` is an alias for `border-top-left-radius`.
#[syntax(" <'border-top-left-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    percentages = BorderBox,
    shorthands = _MsBorderRadius,
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
pub struct MsBorderTopLeftRadiusStyleValue<'a>;

/// `-ms-border-top-right-radius` is an alias for `border-top-right-radius`.
#[syntax(" <'border-top-right-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    percentages = BorderBox,
    shorthands = _MsBorderRadius,
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
pub struct MsBorderTopRightRadiusStyleValue<'a>;

/// `-ms-transition-delay` is an alias for `transition-delay`.
#[syntax(" <'transition-delay'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0s",
    applies_to = Elements,
    shorthands = _MsTransition,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsTransitionDelayStyleValue<'a>;

/// `-ms-transition-duration` is an alias for `transition-duration`.
#[syntax(" <'transition-duration'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0s",
    applies_to = Elements,
    shorthands = _MsTransition,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsTransitionDurationStyleValue<'a>;

/// `-ms-transition-timing-function` is an alias for `transition-timing-function`.
#[syntax(" <'transition-timing-function'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "ease",
    applies_to = Elements,
    shorthands = _MsTransition,
    property_group = Transitions,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MsTransitionTimingFunctionStyleValue<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn grid_tracks_accept_legacy_repetition() {
		assert_parse!(CssAtomSet::ATOMS, MsGridColumnsStyleValue, "10px (250px 10px)[4]");
		assert_parse!(CssAtomSet::ATOMS, MsGridRowsStyleValue, "auto (minmax(10px, 1fr) 5%)[2] 1fr");
		assert_parse_error!(CssAtomSet::ATOMS, MsGridColumnsStyleValue, "10px (250px 10px)[0]");
		assert_parse_error!(CssAtomSet::ATOMS, MsGridRowsStyleValue, "10px ()[4]");
		assert_parse_error!(CssAtomSet::ATOMS, MsGridColumnsStyleValue, "10px (250px 10px)[4");
		assert_parse_error!(CssAtomSet::ATOMS, MsGridRowsStyleValue, "10px [name] 1fr");
	}

	#[test]
	fn grid_indices_require_positive_integers() {
		assert_parse!(CssAtomSet::ATOMS, MsGridColumnStyleValue, "2");
		assert_parse!(CssAtomSet::ATOMS, MsGridRowStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, MsGridColumnSpanStyleValue, "3");
		assert_parse_error!(CssAtomSet::ATOMS, MsGridColumnStyleValue, "0");
		assert_parse_error!(CssAtomSet::ATOMS, MsGridRowStyleValue, "-1");
		assert_peek_false!(CssAtomSet::ATOMS, MsGridColumnSpanStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, MsGridColumnStyleValue, "1 / 3");
	}

	#[test]
	fn test_ms_interpolation_mode_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsInterpolationModeStyleValue, "nearest-neighbor");
		assert_parse!(CssAtomSet::ATOMS, MsInterpolationModeStyleValue, "bicubic");
	}

	#[test]
	fn test_ms_interpolation_mode_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MsInterpolationModeStyleValue, "auto");
		assert_peek_false!(CssAtomSet::ATOMS, MsInterpolationModeStyleValue, "invalid");
	}

	#[test]
	fn test_ms_box_sizing_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsBoxSizingStyleValue, "content-box");
		assert_parse!(CssAtomSet::ATOMS, MsBoxSizingStyleValue, "border-box");
	}

	#[test]
	fn test_ms_box_sizing_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MsBoxSizingStyleValue, "invalid");
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
		assert_peek_false!(CssAtomSet::ATOMS, MsOverflowStyleStyleValue, "invalid");
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
		assert_peek_false!(CssAtomSet::ATOMS, MsTouchActionStyleValue, "invalid");
	}

	#[test]
	fn test_ms_user_select_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsUserSelectStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MsUserSelectStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MsUserSelectStyleValue, "all");
	}

	#[test]
	fn test_ms_user_select_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MsUserSelectStyleValue, "invalid");
	}

	#[test]
	fn test_ms_text_size_adjust_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsTextSizeAdjustStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MsTextSizeAdjustStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MsTextSizeAdjustStyleValue, "100%");
	}

	#[test]
	fn test_ms_text_size_adjust_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MsTextSizeAdjustStyleValue, "invalid");
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
		assert_peek_false!(CssAtomSet::ATOMS, MsFlexStyleValue, "invalid");
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
		assert_peek_false!(CssAtomSet::ATOMS, MsFlexDirectionStyleValue, "invalid");
	}

	#[test]
	fn test_ms_flex_wrap_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsFlexWrapStyleValue, "nowrap");
		assert_parse!(CssAtomSet::ATOMS, MsFlexWrapStyleValue, "wrap");
		assert_parse!(CssAtomSet::ATOMS, MsFlexWrapStyleValue, "wrap-reverse");
	}

	#[test]
	fn test_ms_flex_wrap_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MsFlexWrapStyleValue, "invalid");
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
		assert_peek_false!(CssAtomSet::ATOMS, MsFlexPackStyleValue, "invalid");
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
		assert_peek_false!(CssAtomSet::ATOMS, MsFlexAlignStyleValue, "invalid");
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
		assert_peek_false!(CssAtomSet::ATOMS, MsFlexItemAlignStyleValue, "invalid");
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
		assert_peek_false!(CssAtomSet::ATOMS, MsFlexLinePackStyleValue, "invalid");
	}

	#[test]
	fn test_ms_transform_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsTransformStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MsTransformStyleValue, "rotate(45deg)");
		assert_parse!(CssAtomSet::ATOMS, MsTransformStyleValue, "scale(-1, 1)");
	}

	#[test]
	fn test_ms_transform_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MsTransformStyleValue, "invalid");
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
		assert_peek_false!(CssAtomSet::ATOMS, MsFilterStyleValue, "invalid");
	}

	#[test]
	fn test_ms_word_break_parses() {
		assert_parse!(CssAtomSet::ATOMS, MsWordBreakStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, MsWordBreakStyleValue, "break-all");
		assert_parse!(CssAtomSet::ATOMS, MsWordBreakStyleValue, "keep-all");
	}

	#[test]
	fn test_ms_word_break_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MsWordBreakStyleValue, "invalid");
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
