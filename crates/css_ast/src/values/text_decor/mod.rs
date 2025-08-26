#![allow(warnings)]
//! CSS Text Decoration Module Level 4
//! https://drafts.csswg.org/css-text-decor-4/

mod impls;
use impls::*;

// /// Represents the style value for `text-decoration-line` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-line).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ underline || overline || line-through || blink ] | spelling-error | grammar-error
// /// ```
// ///
// // https://drafts.csswg.org/css-text-decor-4/#text-decoration-line
// #[value(" none | [ underline || overline || line-through || blink ] | spelling-error | grammar-error ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no (but see prose, above)")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum TextDecorationLineStyleValue {}

/// Represents the style value for `text-decoration-style` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// solid | double | dotted | dashed | wavy
/// ```
///
// https://drafts.csswg.org/css-text-decor-4/#text-decoration-style
#[value(" solid | double | dotted | dashed | wavy ")]
#[initial("solid")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum TextDecorationStyleStyleValue {}

/// Represents the style value for `text-decoration-color` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color>
/// ```
///
// https://drafts.csswg.org/css-text-decor-4/#text-decoration-color
#[value(" <color> ")]
#[initial("currentcolor")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct TextDecorationColorStyleValue;

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
// // https://drafts.csswg.org/css-text-decor-4/#text-decoration
// #[value(
// 	" <'text-decoration-line'> || <'text-decoration-thickness'> || <'text-decoration-style'> || <'text-decoration-color'> "
// )]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/text-decoration")]
// #[baseline(widely)]
// #[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
// pub struct TextDecorationStyleValue;

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
// // https://drafts.csswg.org/css-text-decor-4/#text-underline-position
// #[value(" auto | [ from-font | under ] || [ left | right ] ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:33,chrome_android:33,edge:12,firefox:74,firefox_android:79,safari:12.1,safari_ios:12.2)]
// pub enum TextUnderlinePositionStyleValue {}

// /// Represents the style value for `text-emphasis-style` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-emphasis-style).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ [ filled | open ] || [ dot | circle | double-circle | triangle | sesame ] ] | <string>
// /// ```
// ///
// // https://drafts.csswg.org/css-text-decor-4/#text-emphasis-style
// #[value(" none | [ [ filled | open ] || [ dot | circle | double-circle | triangle | sesame ] ] | <string> ")]
// #[initial("none")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum TextEmphasisStyleStyleValue {}

/// Represents the style value for `text-emphasis-color` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-emphasis-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color>
/// ```
///
// https://drafts.csswg.org/css-text-decor-4/#text-emphasis-color
#[value(" <color> ")]
#[initial("currentcolor")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct TextEmphasisColorStyleValue;

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
// // https://drafts.csswg.org/css-text-decor-4/#text-emphasis
// #[value(" <'text-emphasis-style'> || <'text-emphasis-color'> ")]
// #[initial("see individual properties")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/text-emphasis")]
// #[baseline(widely)]
// #[versions(chrome:99,chrome_android:99,edge:99,firefox:46,firefox_android:46,safari:7,safari_ios:7)]
// pub struct TextEmphasisStyleValue;

// /// Represents the style value for `text-emphasis-position` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-emphasis-position).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ over | under ] && [ right | left ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-text-decor-4/#text-emphasis-position
// #[value(" [ over | under ] && [ right | left ]? ")]
// #[initial("over right")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct TextEmphasisPositionStyleValue;

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
// https://drafts.csswg.org/css-text-decor-4/#text-shadow
#[value(" none | <shadow># ")]
#[initial("none")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("as shadow list")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-textshadow")]
#[baseline(widely)]
#[versions(chrome:2,chrome_android:18,edge:12,firefox:3.5,firefox_android:4,safari:1.1,safari_ios:1)]
pub struct TextShadowStyleValue<'a>;

/// Represents the style value for `text-decoration-thickness` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-thickness).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | from-font | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-text-decor-4/#text-decoration-thickness
#[value(" auto | from-font | <length-percentage> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum TextDecorationThicknessStyleValue {}

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
// https://drafts.csswg.org/css-text-decor-4/#text-underline-offset
#[value(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:87,chrome_android:87,edge:87,firefox:70,firefox_android:79,safari:12.1,safari_ios:12.2)]
pub struct TextUnderlineOffsetStyleValue;

/// Represents the style value for `text-decoration-trim` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-trim).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>{1,2} | auto
/// ```
///
// https://drafts.csswg.org/css-text-decor-4/#text-decoration-trim
#[value(" <length>{1,2} | auto ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct TextDecorationTrimStyleValue;

/// Represents the style value for `text-decoration-skip` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | auto
/// ```
///
// https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip
#[value(" none | auto ")]
#[initial("See individual properties")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum TextDecorationSkipStyleValue {}

// /// Represents the style value for `text-decoration-skip-self` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-self).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | skip-all | [ skip-underline || skip-overline || skip-line-through ] | no-skip
// /// ```
// ///
// // https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-self
// #[value(" auto | skip-all | [ skip-underline || skip-overline || skip-line-through ] | no-skip ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum TextDecorationSkipSelfStyleValue {}

/// Represents the style value for `text-decoration-skip-box` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-box).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | all
/// ```
///
// https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-box
#[value(" none | all ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum TextDecorationSkipBoxStyleValue {}

// /// Represents the style value for `text-decoration-skip-spaces` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-spaces).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | all | [ start || end ]
// /// ```
// ///
// // https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-spaces
// #[value(" none | all | [ start || end ] ")]
// #[initial("start end")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum TextDecorationSkipSpacesStyleValue {}

/// Represents the style value for `text-decoration-skip-ink` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-ink).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none | all
/// ```
///
// https://drafts.csswg.org/css-text-decor-4/#text-decoration-skip-ink
#[value(" auto | none | all ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum TextDecorationSkipInkStyleValue {}

/// Represents the style value for `text-emphasis-skip` as defined in [css-text-decor-4](https://drafts.csswg.org/css-text-decor-4/#text-emphasis-skip).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// spaces || punctuation || symbols || narrow
/// ```
///
// https://drafts.csswg.org/css-text-decor-4/#text-emphasis-skip
#[value(" spaces || punctuation || symbols || narrow ")]
#[initial("spaces punctuation")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct TextEmphasisSkipStyleValue;
