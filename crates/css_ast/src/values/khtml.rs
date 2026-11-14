use super::prelude::*;

/// `-khtml-border-bottom-left-radius` is an alias for `border-bottom-left-radius`.
#[syntax(" <'border-bottom-left-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _KhtmlBorderRadius,
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
pub struct KhtmlBorderBottomLeftRadiusStyleValue<'a>;

/// `-khtml-border-bottom-right-radius` is an alias for `border-bottom-right-radius`.
#[syntax(" <'border-bottom-right-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    shorthands = _KhtmlBorderRadius,
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
pub struct KhtmlBorderBottomRightRadiusStyleValue<'a>;

/// `-khtml-border-radius` is an alias for `border-radius`.
#[syntax(" <'border-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    longhands = _KhtmlBorderTopLeftRadius|_KhtmlBorderTopRightRadius|_KhtmlBorderBottomRightRadius|_KhtmlBorderBottomLeftRadius,
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
pub struct KhtmlBorderRadiusStyleValue<'a>;

/// `-khtml-user-select` is an alias for `user-select`.
#[syntax(" <'user-select'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Unknown,
    animation_type = Discrete,
    property_group = Ui,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct KhtmlUserSelectStyleValue<'a>;

/// `-khtml-border-top-left-radius` is an alias for `border-top-left-radius`.
#[syntax(" <'border-top-left-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    percentages = BorderBox,
    shorthands = _KhtmlBorderRadius,
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
pub struct KhtmlBorderTopLeftRadiusStyleValue<'a>;

/// `-khtml-border-top-right-radius` is an alias for `border-top-right-radius`.
#[syntax(" <'border-top-right-radius'> ")]
#[derive(
	Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    percentages = BorderBox,
    shorthands = _KhtmlBorderRadius,
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
pub struct KhtmlBorderTopRightRadiusStyleValue<'a>;
