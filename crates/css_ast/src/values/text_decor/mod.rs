#![allow(warnings)]
//! https://drafts.csswg.org/css-text-decor-4/

mod impls;
use super::prelude::*;
use impls::*;
// /// Represents the style value for `text-decoration` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration).
// ///
// /// The text-decoration CSS property sets the style and color of decorative lines including underline, overline, line-through, or a combination of lines.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'text-decoration-line'> || <'text-decoration-thickness'> || <'text-decoration-style'> || <'text-decoration-color'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-text-decor-4/#text-decoration
// #[syntax(
//     " <'text-decoration-line'> || <'text-decoration-thickness'> || <'text-decoration-style'> || <'text-decoration-color'> "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "see individual properties",
//     inherits = Unknown,
//     applies_to = Unknown,
//     percentages = Unknown,
//     animation_type = Unknown,
//     property_group = TextDecor,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.text-decoration")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct TextDecorationStyleValue;

/// Represents the style value for `text-decoration-color` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-color).
///
/// The text-decoration CSS property sets the style and color of decorative lines including underline, overline, line-through, or a combination of lines.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color>
/// ```
///
/// https://drafts.csswg.org/css-text-decor-4/#text-decoration-color
#[syntax(" <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "currentcolor",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = TextDecor,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-decoration-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct TextDecorationColorStyleValue;

// /// Represents the style value for `text-decoration-line` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-line).
// ///
// /// The text-decoration CSS property sets the style and color of decorative lines including underline, overline, line-through, or a combination of lines.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ underline || overline || line-through || blink ] | spelling-error | grammar-error
// /// ```
// ///
// /// https://drafts.csswg.org/css-text-decor-4/#text-decoration-line
// #[syntax(
//     " none | [ underline || overline || line-through || blink ] | spelling-error | grammar-error "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "none",
//     applies_to = Elements,
//     animation_type = Discrete,
//     property_group = TextDecor,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.text-decoration-line")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum TextDecorationLineStyleValue {}

/// Represents the style value for `text-decoration-skip` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip).
///
/// The text-decoration CSS property sets the style and color of decorative lines including underline, overline, line-through, or a combination of lines.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | auto
/// ```
///
/// https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip
#[syntax(" none | auto ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "See individual properties",
    inherits,
    applies_to = Elements,
    animation_type = Discrete,
    property_group = TextDecor,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-decoration-skip"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum TextDecorationSkipStyleValue {}

/// Represents the style value for `text-decoration-skip-box` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-box).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | all
/// ```
///
/// https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-box
#[syntax(" none | all ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "none",
    inherits,
    applies_to = Elements,
    animation_type = Discrete,
    property_group = TextDecor,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-decoration-skip-box"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum TextDecorationSkipBoxStyleValue {}

/// Represents the style value for `text-decoration-skip-ink` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-ink).
///
/// The text-decoration CSS property sets the style and color of decorative lines including underline, overline, line-through, or a combination of lines.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none | all
/// ```
///
/// https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-ink
#[syntax(" auto | none | all ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "auto",
    inherits,
    applies_to = Elements,
    animation_type = Discrete,
    property_group = TextDecor,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-decoration-skip-ink"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum TextDecorationSkipInkStyleValue {}

// /// Represents the style value for `text-decoration-skip-self` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-self).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | skip-all | [ skip-underline || skip-overline || skip-line-through ] | no-skip
// /// ```
// ///
// /// https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-self
// #[syntax(
//     " auto | skip-all | [ skip-underline || skip-overline || skip-line-through ] | no-skip "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "auto",
//     applies_to = Elements,
//     animation_type = Discrete,
//     property_group = TextDecor,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.text-decoration-skip-self")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum TextDecorationSkipSelfStyleValue {}

// /// Represents the style value for `text-decoration-skip-spaces` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-spaces).
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | all | [ start || end ]
// /// ```
// ///
// /// https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-spaces
// #[syntax(" none | all | [ start || end ] ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "start end",
//     inherits,
//     applies_to = Elements,
//     animation_type = Discrete,
//     property_group = TextDecor,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.text-decoration-skip-spaces")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum TextDecorationSkipSpacesStyleValue {}

/// Represents the style value for `text-decoration-style` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-style).
///
/// The text-decoration CSS property sets the style and color of decorative lines including underline, overline, line-through, or a combination of lines.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// solid | double | dotted | dashed | wavy
/// ```
///
/// https://drafts.csswg.org/css-text-decor-4/#text-decoration-style
#[syntax(" solid | double | dotted | dashed | wavy ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "solid",
    applies_to = Elements,
    animation_type = Discrete,
    property_group = TextDecor,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-decoration-style"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum TextDecorationStyleStyleValue {}

/// Represents the style value for `text-decoration-thickness` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-thickness).
///
/// The text-decoration CSS property sets the style and color of decorative lines including underline, overline, line-through, or a combination of lines.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | from-font | <length-percentage>
/// ```
///
/// https://drafts.csswg.org/css-text-decor-4/#text-decoration-thickness
#[syntax(" auto | from-font | <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "auto",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = TextDecor,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-decoration-thickness"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub enum TextDecorationThicknessStyleValue {}

/// Represents the style value for `text-decoration-trim` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-trim).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>{1,2} | auto
/// ```
///
/// https://drafts.csswg.org/css-text-decor-4/#text-decoration-trim
#[syntax(" <length>{1,2} | auto ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "0",
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = TextDecor,
    computed_value_type = SpecifiedKeywordPlusAbsoluteLength,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-decoration-trim"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct TextDecorationTrimStyleValue;

// /// Represents the style value for `text-emphasis` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-emphasis).
// ///
// /// The text-emphasis CSS property sets position and style for text emphasis marks, especially for East Asian languages.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'text-emphasis-style'> || <'text-emphasis-color'>
// /// ```
// ///
// /// https://drafts.csswg.org/css-text-decor-4/#text-emphasis
// #[syntax(" <'text-emphasis-style'> || <'text-emphasis-color'> ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "see individual properties",
//     inherits = Unknown,
//     applies_to = Unknown,
//     percentages = Unknown,
//     animation_type = Unknown,
//     property_group = TextDecor,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.text-emphasis")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct TextEmphasisStyleValue;

/// Represents the style value for `text-emphasis-color` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-emphasis-color).
///
/// The text-emphasis CSS property sets position and style for text emphasis marks, especially for East Asian languages.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color>
/// ```
///
/// https://drafts.csswg.org/css-text-decor-4/#text-emphasis-color
#[syntax(" <color> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "currentcolor",
    inherits,
    applies_to = Text,
    animation_type = ByComputedValue,
    property_group = TextDecor,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-emphasis-color"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct TextEmphasisColorStyleValue;

// /// Represents the style value for `text-emphasis-position` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-emphasis-position).
// ///
// /// The text-emphasis CSS property sets position and style for text emphasis marks, especially for East Asian languages.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ over | under ] && [ right | left ]?
// /// ```
// ///
// /// https://drafts.csswg.org/css-text-decor-4/#text-emphasis-position
// #[syntax(" [ over | under ] && [ right | left ]? ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "over right",
//     inherits,
//     applies_to = Text,
//     animation_type = Discrete,
//     property_group = TextDecor,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.text-emphasis-position")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct TextEmphasisPositionStyleValue;

/// Represents the style value for `text-emphasis-skip` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-emphasis-skip).
///
/// The grammar is defined as:
///
/// ```text,ignore
/// spaces || punctuation || symbols || narrow
/// ```
///
/// https://drafts.csswg.org/css-text-decor-4/#text-emphasis-skip
#[syntax(" spaces || punctuation || symbols || narrow ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "spaces punctuation",
    inherits,
    applies_to = Text,
    animation_type = Discrete,
    property_group = TextDecor,
    computed_value_type = Unknown,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-emphasis-skip"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct TextEmphasisSkipStyleValue;

// /// Represents the style value for `text-emphasis-style` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-emphasis-style).
// ///
// /// The text-emphasis CSS property sets position and style for text emphasis marks, especially for East Asian languages.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ [ filled | open ] || [ dot | circle | double-circle | triangle | sesame ] ] | <string>
// /// ```
// ///
// /// https://drafts.csswg.org/css-text-decor-4/#text-emphasis-style
// #[syntax(
//     " none | [ [ filled | open ] || [ dot | circle | double-circle | triangle | sesame ] ] | <string> "
// )]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "none",
//     inherits,
//     applies_to = Text,
//     animation_type = Discrete,
//     property_group = TextDecor,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.text-emphasis-style")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub enum TextEmphasisStyleStyleValue {}

/// Represents the style value for `text-shadow` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-shadow).
///
/// The text-shadow CSS property sets the position and styles of shadow on text.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <shadow>#
/// ```
///
/// https://drafts.csswg.org/css-text-decor-4/#text-shadow
#[syntax(" none | <shadow># ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "none",
    inherits,
    applies_to = Text,
    animation_type = ShadowList,
    property_group = TextDecor,
    computed_value_type = AbsoluteLengthOrNone,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-shadow"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct TextShadowStyleValue<'a>;

/// Represents the style value for `text-underline-offset` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-underline-offset).
///
/// The text-underline-offset CSS property shifts underlines on text from the initial position by a given distance. The initial position is affected by the text-underline-position property.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
/// https://drafts.csswg.org/css-text-decor-4/#text-underline-offset
#[syntax(" auto | <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, DeclarationMetadata, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[declaration_metadata(
    initial = "auto",
    inherits,
    applies_to = Elements,
    animation_type = ByComputedValue,
    property_group = TextDecor,
    computed_value_type = AsSpecified,
    canonical_order = "per grammar",
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("css.properties.text-underline-offset"))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
pub struct TextUnderlineOffsetStyleValue;

// /// Represents the style value for `text-underline-position` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-underline-position).
// ///
// /// The text-underline-position CSS property sets the position of underlines on text. For example, text-underline-position: under places the underline below the text, avoiding crossing descenders. The underline may be further adjusted by the text-underline-offset property.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | [ from-font | under ] || [ left | right ]
// /// ```
// ///
// /// https://drafts.csswg.org/css-text-decor-4/#text-underline-position
// #[syntax(" auto | [ from-font | under ] || [ left | right ] ")]
// #[derive(
//     Parse,
//     Peek,
//     ToSpan,
//     ToCursors,
//     DeclarationMetadata,
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     Hash,
// )]
// #[declaration_metadata(
//     initial = "auto",
//     inherits,
//     applies_to = Elements,
//     animation_type = Discrete,
//     property_group = TextDecor,
//     computed_value_type = Unknown,
//     canonical_order = "per grammar",
// )]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(
//     feature = "css_feature_data",
//     derive(ToCSSFeature),
//     css_feature("css.properties.text-underline-position")
// )]
// #[cfg_attr(feature = "visitable", derive(Visitable), visit)]
// pub struct TextUnderlinePositionStyleValue;
