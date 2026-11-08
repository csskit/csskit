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
pub enum MozColumnGapStyleValue<'a> {}

/// `-moz-column-count` — alias for `column-count`.
#[syntax(" auto | <integer [1,∞]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozColumns,
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
pub struct MozColumnCountStyleValue<'a>;

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
pub enum MozUserSelectStyleValue<'a> {}

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
pub enum MozAppearanceStyleValue<'a> {}

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
pub enum MozOsxFontSmoothingStyleValue<'a> {}

/// `-moz-transition` — alias for `transition`.
#[syntax(" <single-transition># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    longhands = _MozTransitionProperty|_MozTransitionDuration|_MozTransitionTimingFunction|_MozTransitionDelay,
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
pub enum MozBoxSizingStyleValue<'a> {}

/// `-moz-filter` - alias for `filter`.
#[syntax(" none | <filter-value-list> ")]
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
pub struct MozFilterStyleValue<'a>;

/// `-moz-animation` is an alias for `animation`.
#[syntax(" <'animation'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    longhands = _MozAnimationName|_MozAnimationDuration|_MozAnimationTimingFunction|_MozAnimationDelay|_MozAnimationIterationCount|_MozAnimationDirection|_MozAnimationFillMode|_MozAnimationPlayState,
    initial = "see individual properties",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozAnimationStyleValue<'a>;

/// `-moz-animation-delay` is an alias for `animation-delay`.
#[syntax(" <'animation-delay'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozAnimation,
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    animation_type = Unknown,
    percentages = Unknown,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozAnimationDelayStyleValue<'a>;

/// `-moz-animation-direction` is an alias for `animation-direction`.
#[syntax(" <'animation-direction'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozAnimation,
    initial = "normal",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozAnimationDirectionStyleValue<'a>;

/// `-moz-animation-duration` is an alias for `animation-duration`.
#[syntax(" <'animation-duration'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozAnimation,
    initial = "auto",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozAnimationDurationStyleValue<'a>;

/// `-moz-animation-fill-mode` is an alias for `animation-fill-mode`.
#[syntax(" <'animation-fill-mode'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozAnimation,
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozAnimationFillModeStyleValue<'a>;

/// `-moz-animation-iteration-count` is an alias for `animation-iteration-count`.
#[syntax(" <'animation-iteration-count'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozAnimation,
    initial = "1",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozAnimationIterationCountStyleValue<'a>;

/// `-moz-animation-name` is an alias for `animation-name`.
#[syntax(" <'animation-name'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozAnimation,
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozAnimationNameStyleValue<'a>;

/// `-moz-animation-timing-function` is an alias for `animation-timing-function`.
#[syntax(" <'animation-timing-function'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozAnimation,
    initial = "ease",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozAnimationTimingFunctionStyleValue<'a>;

/// `-moz-backface-visibility` is an alias for `backface-visibility`.
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
pub struct MozBackfaceVisibilityStyleValue<'a>;

/// `-moz-background-size` is an alias for `background-size`.
#[syntax(" <'background-size'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    animation_type = RepeatableList,
    percentages = Unknown,
    property_group = Backgrounds,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBackgroundSizeStyleValue<'a>;

/// `-moz-border-image` is an alias for `border-image`.
#[syntax(" <'border-image'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    longhands = _MozBorderImageSource|_MozBorderImageSlice|_MozBorderImageWidth|_MozBorderImageOutset|_MozBorderImageRepeat,
    initial = "See individual properties",
    applies_to = Unknown,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBorderImageStyleValue<'a>;

/// `-moz-border-radius` is an alias for `border-radius`.
#[syntax(" <'border-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    longhands = _MozBorderRadiusTopleft|_MozBorderRadiusTopright|_MozBorderRadiusBottomright|_MozBorderRadiusBottomleft,
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    animation_type = Unknown,
    percentages = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBorderRadiusStyleValue<'a>;

/// `-moz-border-radius-bottomleft` is an alias for `border-bottom-left-radius`.
#[syntax(" <'border-bottom-left-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozBorderRadius,
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
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBorderRadiusBottomleftStyleValue<'a>;

/// `-moz-border-radius-bottomright` is an alias for `border-bottom-right-radius`.
#[syntax(" <'border-bottom-right-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozBorderRadius,
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
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBorderRadiusBottomrightStyleValue<'a>;

/// `-moz-border-radius-topleft` is an alias for `border-top-left-radius`.
#[syntax(" <'border-top-left-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozBorderRadius,
    initial = "0",
    applies_to = Elements,
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
pub struct MozBorderRadiusTopleftStyleValue<'a>;

/// `-moz-border-radius-topright` is an alias for `border-top-right-radius`.
#[syntax(" <'border-top-right-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozBorderRadius,
    initial = "0",
    applies_to = Elements,
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
pub struct MozBorderRadiusToprightStyleValue<'a>;

/// `-moz-box-shadow` is an alias for `box-shadow`.
#[syntax(" <'box-shadow'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Elements,
    animation_type = Unknown,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBoxShadowStyleValue<'a>;

/// `-moz-column-fill` is an alias for `column-fill`.
#[syntax(" <'column-fill'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "balance",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Multicol,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozColumnFillStyleValue<'a>;

/// `-moz-column-rule` is an alias for `column-rule`.
#[syntax(" <'column-rule'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    longhands = _MozColumnRuleWidth|_MozColumnRuleStyle|_MozColumnRuleColor,
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    animation_type = Unknown,
    percentages = Unknown,
    property_group = Gaps,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozColumnRuleStyleValue<'a>;

/// `-moz-column-span` is an alias for `column-span`.
#[syntax(" <'column-span'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Multicol,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozColumnSpanStyleValue<'a>;

/// `-moz-column-width` is an alias for `column-width`.
#[syntax(" <'column-width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozColumns,
    initial = "auto",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    property_group = Multicol,
    computed_value_type = SpecifiedKeywordPlusAbsoluteLength,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozColumnWidthStyleValue<'a>;

/// `-moz-columns` is an alias for `columns`.
#[syntax(" <'columns'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    longhands = _MozColumnWidth|_MozColumnCount,
    initial = "see individual properties",
    inherits = Unknown,
    applies_to = Unknown,
    animation_type = Unknown,
    percentages = Unknown,
    property_group = Multicol,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozColumnsStyleValue<'a>;

/// `-moz-font-feature-settings` is an alias for `font-feature-settings`.
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
pub struct MozFontFeatureSettingsStyleValue<'a>;

/// `-moz-hyphens` is an alias for `hyphens`.
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
pub struct MozHyphensStyleValue<'a>;

/// `-moz-perspective` is an alias for `perspective`.
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
pub struct MozPerspectiveStyleValue<'a>;

/// `-moz-perspective-origin` is an alias for `perspective-origin`.
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
pub struct MozPerspectiveOriginStyleValue<'a>;

/// `-moz-tab-size` is an alias for `tab-size`.
#[syntax(" <'tab-size'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "8",
    inherits,
    applies_to = Text,
    animation_type = ByComputedValue,
    property_group = Text,
    computed_value_type = SpecifiedKeywordPlusAbsoluteLength,
    canonical_order = "n/a",
    unitless_zero_resolves = Number,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozTabSizeStyleValue<'a>;

/// `-moz-text-size-adjust` is an alias for `text-size-adjust`.
#[syntax(" <'text-size-adjust'> ")]
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
pub struct MozTextSizeAdjustStyleValue<'a>;

/// `-moz-transform` is an alias for `transform`.
#[syntax(" <'transform'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = TransformList,
    percentages = ReferenceBox,
    property_group = Transforms,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozTransformStyleValue<'a>;

/// `-moz-transform-origin` is an alias for `transform-origin`.
#[syntax(" <'transform-origin'> ")]
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
pub struct MozTransformOriginStyleValue<'a>;

/// `-moz-transform-style` is an alias for `transform-style`.
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
pub struct MozTransformStyleStyleValue<'a>;

/// `-moz-transition-duration` is an alias for `transition-duration`.
#[syntax(" <'transition-duration'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozTransition,
    initial = "0s",
    applies_to = Elements,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozTransitionDurationStyleValue<'a>;

/// `-moz-transition-property` is an alias for `transition-property`.
#[syntax(" <'transition-property'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _MozTransition,
    initial = "all",
    applies_to = Elements,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozTransitionPropertyStyleValue<'a>;

/// Represents the value of `-moz-background-clip`.
#[syntax(" [ border | padding | content | border-box | padding-box | content-box | text ]# ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "border",
    applies_to = Elements,
    property_group = Backgrounds,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBackgroundClipStyleValue<'a>;

/// Represents the value of `-moz-box-align`.
#[syntax(" <'-webkit-box-align'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "stretch",
    applies_to = Elements,
    property_group = Flexbox,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBoxAlignStyleValue<'a>;

/// Represents the value of `-moz-box-flex`.
#[syntax(" <'-webkit-box-flex'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    property_group = Flexbox,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBoxFlexStyleValue<'a>;

/// Represents the value of `-moz-box-ordinal-group`.
#[syntax(" <integer [0,∞]> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "1",
    applies_to = Elements,
    property_group = Flexbox,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBoxOrdinalGroupStyleValue<'a>;

/// Represents the value of `-moz-column-break-inside`.
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
pub enum MozColumnBreakInsideStyleValue<'a> {}

/// Represents the value of `-moz-context-properties`.
#[syntax(" none | <context-property># ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    inherits,
    applies_to = Elements,
    property_group = SvgPainting,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozContextPropertiesStyleValue<'a>;

/// Represents the value of `-moz-user-modify`.
#[syntax(" read-only | read-write | write-only ")]
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
pub enum MozUserModifyStyleValue<'a> {}

/// Represents the value of `-moz-window-shadow`.
#[syntax(" default | menu | tooltip | sheet | none ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "default",
    applies_to = Elements,
    property_group = Ui,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MozWindowShadowStyleValue<'a> {}

/// `-moz-animation-play-state` is an alias for `animation-play-state`.
#[syntax(" <'animation-play-state'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "running",
    applies_to = Elements,
    shorthands = _MozAnimation,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozAnimationPlayStateStyleValue<'a>;

/// `-moz-border-image-outset` is an alias for `border-image-outset`.
#[syntax(" <'border-image-outset'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    shorthands = _MozBorderImage,
    property_group = Borders,
    computed_value_type = AbsoluteLength,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBorderImageOutsetStyleValue<'a>;

/// `-moz-border-image-repeat` is an alias for `border-image-repeat`.
#[syntax(" <'border-image-repeat'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "stretch",
    applies_to = Unknown,
    animation_type = Discrete,
    shorthands = _MozBorderImage,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBorderImageRepeatStyleValue<'a>;

/// `-moz-border-image-slice` is an alias for `border-image-slice`.
#[syntax(" <'border-image-slice'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "100%",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = BorderImageArea,
    shorthands = _MozBorderImage,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBorderImageSliceStyleValue<'a>;

/// `-moz-border-image-source` is an alias for `border-image-source`.
#[syntax(" <'border-image-source'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    shorthands = _MozBorderImage,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBorderImageSourceStyleValue<'a>;

/// `-moz-border-image-width` is an alias for `border-image-width`.
#[syntax(" <'border-image-width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "1",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = BorderImageArea,
    shorthands = _MozBorderImage,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozBorderImageWidthStyleValue<'a>;

/// `-moz-column-rule-color` is an alias for `column-rule-color`.
#[syntax(" <'column-rule-color'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "currentcolor",
    applies_to = Unknown,
    animation_type = RepeatableList,
    shorthands = _MozColumnRule,
    property_group = Gaps,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozColumnRuleColorStyleValue<'a>;

/// `-moz-column-rule-style` is an alias for `column-rule-style`.
#[syntax(" <'column-rule-style'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    shorthands = _MozColumnRule,
    property_group = Gaps,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozColumnRuleStyleStyleValue<'a>;

/// `-moz-column-rule-width` is an alias for `column-rule-width`.
#[syntax(" <'column-rule-width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "medium",
    applies_to = Unknown,
    animation_type = RepeatableList,
    shorthands = _MozColumnRule,
    property_group = Gaps,
    computed_value_type = ListOfAbsoluteLengths,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozColumnRuleWidthStyleValue<'a>;

/// `-moz-transition-delay` is an alias for `transition-delay`.
#[syntax(" <'transition-delay'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0s",
    applies_to = Elements,
    shorthands = _MozTransition,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozTransitionDelayStyleValue<'a>;

/// `-moz-transition-timing-function` is an alias for `transition-timing-function`.
#[syntax(" <'transition-timing-function'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "ease",
    applies_to = Elements,
    shorthands = _MozTransition,
    property_group = Transitions,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MozTransitionTimingFunctionStyleValue<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn context_properties_exclude_none_from_lists() {
		assert_parse!(CssAtomSet::ATOMS, MozContextPropertiesStyleValue, "fill, fill-opacity, stroke");
		assert_parse!(CssAtomSet::ATOMS, MozContextPropertiesStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, MozContextPropertiesStyleValue, "none, fill");
		assert_peek_false!(CssAtomSet::ATOMS, MozContextPropertiesStyleValue, "background-color");
	}

	#[test]
	fn test_moz_column_gap_parses() {
		assert_parse!(CssAtomSet::ATOMS, MozColumnGapStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, MozColumnGapStyleValue, "1rem");
		assert_parse!(CssAtomSet::ATOMS, MozColumnGapStyleValue, "0");
	}

	#[test]
	fn test_moz_column_gap_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MozColumnGapStyleValue, "invalid");
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
		assert_peek_false!(CssAtomSet::ATOMS, MozUserSelectStyleValue, "invalid");
	}

	#[test]
	fn test_moz_appearance_parses() {
		assert_parse!(CssAtomSet::ATOMS, MozAppearanceStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MozAppearanceStyleValue, "auto");
	}

	#[test]
	fn test_moz_appearance_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MozAppearanceStyleValue, "invalid");
	}

	#[test]
	fn test_moz_osx_font_smoothing_parses() {
		assert_parse!(CssAtomSet::ATOMS, MozOsxFontSmoothingStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MozOsxFontSmoothingStyleValue, "grayscale");
	}

	#[test]
	fn test_moz_osx_font_smoothing_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MozOsxFontSmoothingStyleValue, "antialiased");
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
		assert_peek_false!(CssAtomSet::ATOMS, MozBoxSizingStyleValue, "invalid");
	}

	#[test]
	fn test_moz_filter_parses() {
		assert_parse!(CssAtomSet::ATOMS, MozFilterStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MozFilterStyleValue, "blur(4px)");
		assert_parse!(CssAtomSet::ATOMS, MozFilterStyleValue, "brightness(0.5) contrast(1.2)");
	}

	#[test]
	fn test_moz_filter_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MozFilterStyleValue, "invalid");
	}
}
