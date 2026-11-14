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
pub enum OObjectFitStyleValue<'a> {}

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
pub enum OBoxSizingStyleValue<'a> {}

/// `-o-filter` - alias for `filter`.
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
pub struct OFilterStyleValue<'a>;

/// `-o-animation` is an alias for `animation`.
#[syntax(" <'animation'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    longhands = _OAnimationName|_OAnimationDuration|_OAnimationTimingFunction|_OAnimationDelay|_OAnimationIterationCount|_OAnimationDirection|_OAnimationFillMode|_OAnimationPlayState,
    initial = "see individual properties",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OAnimationStyleValue<'a>;

/// `-o-animation-delay` is an alias for `animation-delay`.
#[syntax(" <'animation-delay'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _OAnimation,
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
pub struct OAnimationDelayStyleValue<'a>;

/// `-o-animation-direction` is an alias for `animation-direction`.
#[syntax(" <'animation-direction'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _OAnimation,
    initial = "normal",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OAnimationDirectionStyleValue<'a>;

/// `-o-animation-duration` is an alias for `animation-duration`.
#[syntax(" <'animation-duration'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _OAnimation,
    initial = "auto",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OAnimationDurationStyleValue<'a>;

/// `-o-animation-fill-mode` is an alias for `animation-fill-mode`.
#[syntax(" <'animation-fill-mode'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _OAnimation,
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OAnimationFillModeStyleValue<'a>;

/// `-o-animation-iteration-count` is an alias for `animation-iteration-count`.
#[syntax(" <'animation-iteration-count'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _OAnimation,
    initial = "1",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OAnimationIterationCountStyleValue<'a>;

/// `-o-animation-name` is an alias for `animation-name`.
#[syntax(" <'animation-name'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _OAnimation,
    initial = "none",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OAnimationNameStyleValue<'a>;

/// `-o-animation-timing-function` is an alias for `animation-timing-function`.
#[syntax(" <'animation-timing-function'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _OAnimation,
    initial = "ease",
    applies_to = Elements,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OAnimationTimingFunctionStyleValue<'a>;

/// `-o-backface-visibility` is an alias for `backface-visibility`.
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
pub struct OBackfaceVisibilityStyleValue<'a>;

/// `-o-background-size` is an alias for `background-size`.
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
pub struct OBackgroundSizeStyleValue<'a>;

/// `-o-border-bottom-left-radius` is an alias for `border-bottom-left-radius`.
#[syntax(" <'border-bottom-left-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _OBorderRadius,
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
pub struct OBorderBottomLeftRadiusStyleValue<'a>;

/// `-o-border-bottom-right-radius` is an alias for `border-bottom-right-radius`.
#[syntax(" <'border-bottom-right-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _OBorderRadius,
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
pub struct OBorderBottomRightRadiusStyleValue<'a>;

/// `-o-border-image` is an alias for `border-image`.
#[syntax(" <'border-image'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    longhands = _OBorderImageSource|_OBorderImageSlice|_OBorderImageWidth|_OBorderImageOutset|_OBorderImageRepeat,
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
pub struct OBorderImageStyleValue<'a>;

/// `-o-border-radius` is an alias for `border-radius`.
#[syntax(" <'border-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    longhands = _OBorderTopLeftRadius|_OBorderTopRightRadius|_OBorderBottomRightRadius|_OBorderBottomLeftRadius,
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
pub struct OBorderRadiusStyleValue<'a>;

/// `-o-box-shadow` is an alias for `box-shadow`.
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
pub struct OBoxShadowStyleValue<'a>;

/// `-o-font-feature-settings` is an alias for `font-feature-settings`.
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
pub struct OFontFeatureSettingsStyleValue<'a>;

/// `-o-object-position` is an alias for `object-position`.
#[syntax(" <'object-position'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "50% 50%",
    applies_to = Unknown,
    animation_type = Unknown,
    percentages = SelfSize,
    property_group = Images,
    computed_value_type = Unknown,
    canonical_order = "the horizontal component of the <position>, followed by the vertical component",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OObjectPositionStyleValue<'a>;

/// `-o-perspective` is an alias for `perspective`.
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
pub struct OPerspectiveStyleValue<'a>;

/// `-o-perspective-origin` is an alias for `perspective-origin`.
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
pub struct OPerspectiveOriginStyleValue<'a>;

/// `-o-tab-size` is an alias for `tab-size`.
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
pub struct OTabSizeStyleValue<'a>;

/// `-o-transform` is an alias for `transform`.
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
pub struct OTransformStyleValue<'a>;

/// `-o-transform-origin` is an alias for `transform-origin`.
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
pub struct OTransformOriginStyleValue<'a>;

/// `-o-transform-style` is an alias for `transform-style`.
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
pub struct OTransformStyleStyleValue<'a>;

/// `-o-transition` is an alias for `transition`.
#[syntax(" <'transition'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    longhands = _OTransitionProperty|_OTransitionDuration|_OTransitionTimingFunction|_OTransitionDelay,
    initial = "see individual properties",
    applies_to = Elements,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OTransitionStyleValue<'a>;

/// `-o-transition-duration` is an alias for `transition-duration`.
#[syntax(" <'transition-duration'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _OTransition,
    initial = "0s",
    applies_to = Elements,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OTransitionDurationStyleValue<'a>;

/// `-o-transition-property` is an alias for `transition-property`.
#[syntax(" <'transition-property'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _OTransition,
    initial = "all",
    applies_to = Elements,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OTransitionPropertyStyleValue<'a>;

/// `-o-animation-play-state` is an alias for `animation-play-state`.
#[syntax(" <'animation-play-state'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "running",
    applies_to = Elements,
    shorthands = _OAnimation,
    property_group = Animations,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OAnimationPlayStateStyleValue<'a>;

/// `-o-border-image-outset` is an alias for `border-image-outset`.
#[syntax(" <'border-image-outset'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    shorthands = _OBorderImage,
    property_group = Borders,
    computed_value_type = AbsoluteLength,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OBorderImageOutsetStyleValue<'a>;

/// `-o-border-image-repeat` is an alias for `border-image-repeat`.
#[syntax(" <'border-image-repeat'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "stretch",
    applies_to = Unknown,
    animation_type = Discrete,
    shorthands = _OBorderImage,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OBorderImageRepeatStyleValue<'a>;

/// `-o-border-image-slice` is an alias for `border-image-slice`.
#[syntax(" <'border-image-slice'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "100%",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = BorderImageArea,
    shorthands = _OBorderImage,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OBorderImageSliceStyleValue<'a>;

/// `-o-border-image-source` is an alias for `border-image-source`.
#[syntax(" <'border-image-source'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "none",
    applies_to = Unknown,
    animation_type = Discrete,
    shorthands = _OBorderImage,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OBorderImageSourceStyleValue<'a>;

/// `-o-border-image-width` is an alias for `border-image-width`.
#[syntax(" <'border-image-width'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "1",
    applies_to = Unknown,
    animation_type = ByComputedValue,
    percentages = BorderImageArea,
    shorthands = _OBorderImage,
    property_group = Borders,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
    box_portion = Border,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OBorderImageWidthStyleValue<'a>;

/// `-o-border-top-left-radius` is an alias for `border-top-left-radius`.
#[syntax(" <'border-top-left-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    percentages = BorderBox,
    shorthands = _OBorderRadius,
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
pub struct OBorderTopLeftRadiusStyleValue<'a>;

/// `-o-border-top-right-radius` is an alias for `border-top-right-radius`.
#[syntax(" <'border-top-right-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    percentages = BorderBox,
    shorthands = _OBorderRadius,
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
pub struct OBorderTopRightRadiusStyleValue<'a>;

/// `-o-transition-delay` is an alias for `transition-delay`.
#[syntax(" <'transition-delay'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0s",
    applies_to = Elements,
    shorthands = _OTransition,
    property_group = Transitions,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OTransitionDelayStyleValue<'a>;

/// `-o-transition-timing-function` is an alias for `transition-timing-function`.
#[syntax(" <'transition-timing-function'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "ease",
    applies_to = Elements,
    shorthands = _OTransition,
    property_group = Transitions,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OTransitionTimingFunctionStyleValue<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

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
		assert_peek_false!(CssAtomSet::ATOMS, OObjectFitStyleValue, "invalid");
	}

	#[test]
	fn test_o_box_sizing_parses() {
		assert_parse!(CssAtomSet::ATOMS, OBoxSizingStyleValue, "content-box");
		assert_parse!(CssAtomSet::ATOMS, OBoxSizingStyleValue, "border-box");
	}

	#[test]
	fn test_o_box_sizing_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, OBoxSizingStyleValue, "invalid");
	}

	#[test]
	fn test_o_filter_parses() {
		assert_parse!(CssAtomSet::ATOMS, OFilterStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, OFilterStyleValue, "blur(4px)");
		assert_parse!(CssAtomSet::ATOMS, OFilterStyleValue, "brightness(0.5) contrast(1.2)");
	}

	#[test]
	fn test_o_filter_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, OFilterStyleValue, "invalid");
	}
}
