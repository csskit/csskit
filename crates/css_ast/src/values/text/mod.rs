#![allow(warnings)]
//! CSS Text Module Level 4
//! https://drafts.csswg.org/css-text-4/

mod impls;
use impls::*;

// /// Represents the style value for `text-transform` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-transform).
// ///
// /// The text-transform CSS property sets text case and capitalization.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [capitalize | uppercase | lowercase ] || full-width || full-size-kana | math-auto
// /// ```
// ///
// // https://drafts.csswg.org/css-text-4/#text-transform
// #[value(" none | [capitalize | uppercase | lowercase ] || full-width || full-size-kana | math-auto ")]
// #[initial("none")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("n/a")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
// pub enum TextTransformStyleValue {}

// /// Represents the style value for `white-space` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#white-space).
// ///
// /// The white-space CSS property sets how white space is collapsed and how lines wrap. It is a shorthand for white-space-collapse and text-wrap-mode.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | pre | pre-wrap | pre-line | <'white-space-collapse'> || <'text-wrap-mode'> || <'white-space-trim'>
// /// ```
// ///
// // https://drafts.csswg.org/css-text-4/#white-space
// #[value(
// 	" normal | pre | pre-wrap | pre-line | <'white-space-collapse'> || <'text-wrap-mode'> || <'white-space-trim'> "
// )]
// #[initial("normal")]
// #[applies_to("text")]
// #[inherited("see individual properties")]
// #[percentages("n/a")]
// #[canonical_order("n/a")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
// pub enum WhiteSpaceStyleValue {}

/// Represents the style value for `tab-size` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#tab-size).
///
/// The tab-size CSS property sets the width of the tab character.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <number [0,∞]> | <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-text-4/#tab-size
#[value(" <number [0,∞]> | <length [0,∞]> ")]
#[initial("8")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css3-tabsize")]
#[baseline(widely)]
#[versions(chrome:42,chrome_android:42,edge:79,firefox:91,firefox_android:91,safari:13.1,safari_ios:13.4)]
pub struct TabSizeStyleValue;

/// Represents the style value for `word-break` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#word-break).
///
/// The word-break CSS property sets how lines break within words.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | break-all | keep-all | manual | auto-phrase | break-word
/// ```
///
// https://drafts.csswg.org/css-text-4/#word-break
#[value(" normal | break-all | keep-all | manual | auto-phrase | break-word ")]
#[initial("normal")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/word-break")]
#[baseline(widely)]
#[versions(chrome:44,chrome_android:44,edge:12,firefox:15,firefox_android:15,safari:9,safari_ios:9)]
pub enum WordBreakStyleValue {}

/// Represents the style value for `line-break` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#line-break).
///
/// The line-break CSS property sets how strictly to apply rules for wrapping text to new lines, especially for symbols and punctuation.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | loose | normal | strict | anywhere
/// ```
///
// https://drafts.csswg.org/css-text-4/#line-break
#[value(" auto | loose | normal | strict | anywhere ")]
#[initial("auto")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:83,chrome_android:83,edge:83,firefox:69,firefox_android:79,safari:13,safari_ios:13)]
pub enum LineBreakStyleValue {}

/// Represents the style value for `hyphens` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#hyphens).
///
/// The hyphens CSS property controls when long words are broken by line wrapping. Although called hyphens, the property applies to word-splitting behavior across languages, such as customary spelling changes or the use of other characters. Support for non-English languages varies significantly.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | manual | auto
/// ```
///
// https://drafts.csswg.org/css-text-4/#hyphens
#[value(" none | manual | auto ")]
#[initial("manual")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-hyphens")]
#[baseline(newly)]
#[versions(chrome:88,chrome_android:55,edge:88,firefox:43,firefox_android:43,safari:17,safari_ios:17)]
pub enum HyphensStyleValue {}

/// Represents the style value for `overflow-wrap` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#overflow-wrap).
///
/// The overflow-wrap CSS property breaks a line of text onto multiple lines inside the targeted element in an otherwise unbreakable place to prevent overflow. The legacy property is word-wrap.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | break-word | anywhere
/// ```
///
// https://drafts.csswg.org/css-text-4/#overflow-wrap
#[value(" normal | break-word | anywhere ")]
#[initial("normal")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/wordwrap")]
#[baseline(widely)]
#[versions(chrome:23,chrome_android:25,edge:18,firefox:49,firefox_android:49,safari:7,safari_ios:7)]
pub enum OverflowWrapStyleValue {}

/// Represents the style value for `word-wrap` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#word-wrap).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | break-word | anywhere
/// ```
///
// https://drafts.csswg.org/css-text-4/#word-wrap
#[value(" normal | break-word | anywhere ")]
#[initial("normal")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum WordWrapStyleValue {}

/// Represents the style value for `text-align` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-align).
///
/// The text-align CSS property sets the horizontal placement of the inner content of a block element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// start | end | left | right | center | <string> | justify | match-parent | justify-all
/// ```
///
// https://drafts.csswg.org/css-text-4/#text-align
#[value(" start | end | left | right | center | <string> | justify | match-parent | justify-all ")]
#[initial("start")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("see individual properties")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
pub enum TextAlignStyleValue {}

/// Represents the style value for `text-align-all` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-align-all).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// start | end | left | right | center | <string> | justify | match-parent
/// ```
///
// https://drafts.csswg.org/css-text-4/#text-align-all
#[value(" start | end | left | right | center | <string> | justify | match-parent ")]
#[initial("start")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum TextAlignAllStyleValue {}

/// Represents the style value for `text-align-last` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-align-last).
///
/// The text-align-last CSS property sets the alignment of the last line of text before a forced line break.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | start | end | left | right | center | justify | match-parent
/// ```
///
// https://drafts.csswg.org/css-text-4/#text-align-last
#[value(" auto | start | end | left | right | center | justify | match-parent ")]
#[initial("auto")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("n/a")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-text-align-last")]
#[baseline(widely)]
#[versions(chrome:47,chrome_android:47,edge:12,firefox:49,firefox_android:49,safari:16,safari_ios:16)]
pub enum TextAlignLastStyleValue {}

// /// Represents the style value for `text-justify` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-justify).
// ///
// /// The text-justify CSS property sets the justification method of text when text-align: justify is set.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ auto | none | inter-word | inter-character | ruby ] || no-compress
// /// ```
// ///
// // https://drafts.csswg.org/css-text-4/#text-justify
// #[value(" [ auto | none | inter-word | inter-character | ruby ] || no-compress ")]
// #[initial("auto")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("n/a")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/css-text-justify")]
// #[baseline(limited)]
// #[versions(firefox:55,firefox_android:55)]
// pub struct TextJustifyStyleValue;

/// Represents the style value for `word-spacing` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#word-spacing).
///
/// The word-spacing CSS property sets the amount of white space between words.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-text-4/#word-spacing
#[value(" normal | <length-percentage> ")]
#[initial("normal")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("relative to computed font-size, i.e. 1em")]
#[canonical_order("n/a")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
pub enum WordSpacingStyleValue {}

/// Represents the style value for `letter-spacing` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#letter-spacing).
///
/// The letter-spacing CSS property controls the amount of space between each letter in an element or block of text.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-text-4/#letter-spacing
#[value(" normal | <length-percentage> ")]
#[initial("normal")]
#[applies_to("inline boxes and text")]
#[inherited("yes")]
#[percentages("relative to computed font-size, i.e. 1em")]
#[canonical_order("n/a")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-letter-spacing")]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
pub enum LetterSpacingStyleValue {}

// /// Represents the style value for `text-indent` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-indent).
// ///
// /// The text-indent CSS property sets the size of the empty space (indentation) at the beginning of lines in a text.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage> ] && hanging? && each-line?
// /// ```
// ///
// // https://drafts.csswg.org/css-text-4/#text-indent
// #[value(" [ <length-percentage> ] && hanging? && each-line? ")]
// #[initial("0")]
// #[applies_to("block containers")]
// #[inherited("yes")]
// #[percentages("refers to block container’s own inline-axis inner size")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/css-text-indent")]
// #[baseline(widely)]
// #[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
// pub struct TextIndentStyleValue;

// /// Represents the style value for `hanging-punctuation` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#hanging-punctuation).
// ///
// /// The hanging-punctuation CSS property puts punctuation characters outside of the box to align the text with the rest of the document.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ first || [ force-end | allow-end ] || last ]
// /// ```
// ///
// // https://drafts.csswg.org/css-text-4/#hanging-punctuation
// #[value(" none | [ first || [ force-end | allow-end ] || last ] ")]
// #[initial("none")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/css-hanging-punctuation")]
// #[baseline(limited)]
// #[versions(Unknown)]
// pub enum HangingPunctuationStyleValue {}

// /// Represents the style value for `word-space-transform` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#word-space-transform).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ space | ideographic-space ] && auto-phrase?
// /// ```
// ///
// // https://drafts.csswg.org/css-text-4/#word-space-transform
// #[value(" none | [ space | ideographic-space ] && auto-phrase? ")]
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
// pub enum WordSpaceTransformStyleValue {}

/// Represents the style value for `white-space-collapse` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#white-space-collapse).
///
/// The white-space-collapse CSS property sets whether new line characters are shown as line breaks, and whether multiple consecutive spaces are all displayed or combined.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// collapse | discard | preserve | preserve-breaks | preserve-spaces | break-spaces
/// ```
///
// https://drafts.csswg.org/css-text-4/#white-space-collapse
#[value(" collapse | discard | preserve | preserve-breaks | preserve-spaces | break-spaces ")]
#[initial("collapse")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:114,chrome_android:114,edge:114,firefox:124,firefox_android:124,safari:17.4,safari_ios:17.4)]
pub enum WhiteSpaceCollapseStyleValue {}

// /// Represents the style value for `white-space-trim` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#white-space-trim).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | discard-before || discard-after || discard-inner
// /// ```
// ///
// // https://drafts.csswg.org/css-text-4/#white-space-trim
// #[value(" none | discard-before || discard-after || discard-inner ")]
// #[initial("none")]
// #[applies_to("inline boxes and block containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum WhiteSpaceTrimStyleValue {}

/// Represents the style value for `text-wrap-mode` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-wrap-mode).
///
/// The text-wrap-mode CSS property sets whether lines may wrap with the values wrap and nowrap. It is a longhand property for both white-space and text-wrap.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// wrap | nowrap
/// ```
///
// https://drafts.csswg.org/css-text-4/#text-wrap-mode
#[value(" wrap | nowrap ")]
#[initial("wrap")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:130,chrome_android:130,edge:130,firefox:124,firefox_android:124,safari:17.4,safari_ios:17.4)]
pub enum TextWrapModeStyleValue {}

/// Represents the style value for `wrap-inside` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#wrap-inside).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | avoid
/// ```
///
// https://drafts.csswg.org/css-text-4/#wrap-inside
#[value(" auto | avoid ")]
#[initial("auto")]
#[applies_to("inline boxes")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum WrapInsideStyleValue {}

/// Represents the style value for `wrap-before` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#wrap-before).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | avoid | avoid-line | avoid-flex | line | flex
/// ```
///
// https://drafts.csswg.org/css-text-4/#wrap-before
#[value(" auto | avoid | avoid-line | avoid-flex | line | flex ")]
#[initial("auto")]
#[applies_to("inline-level boxes and flex items")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum WrapBeforeStyleValue {}

/// Represents the style value for `wrap-after` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#wrap-after).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | avoid | avoid-line | avoid-flex | line | flex
/// ```
///
// https://drafts.csswg.org/css-text-4/#wrap-after
#[value(" auto | avoid | avoid-line | avoid-flex | line | flex ")]
#[initial("auto")]
#[applies_to("inline-level boxes and flex items")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum WrapAfterStyleValue {}

/// Represents the style value for `text-wrap-style` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-wrap-style).
///
/// The text-wrap-style CSS property sets how lines break in text that overflows the container. It can also be set with the text-wrap shorthand.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | balance | stable | pretty | avoid-orphans
/// ```
///
// https://drafts.csswg.org/css-text-4/#text-wrap-style
#[value(" auto | balance | stable | pretty | avoid-orphans ")]
#[initial("auto")]
#[applies_to("block containers hat establish an inline formatting context")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:130,chrome_android:130,edge:130,firefox:124,safari:17.5)]
pub enum TextWrapStyleStyleValue {}

/// Represents the style value for `text-wrap` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-wrap).
///
/// The text-wrap CSS property sets how lines break in text that overflows the container. It is a shorthand for text-wrap-style and text-wrap-mode.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'text-wrap-mode'> || <'text-wrap-style'>
/// ```
///
// https://drafts.csswg.org/css-text-4/#text-wrap
#[value(" <'text-wrap-mode'> || <'text-wrap-style'> ")]
#[initial("wrap")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:114,chrome_android:114,edge:114,firefox:124,firefox_android:124,safari:17.4,safari_ios:17.4)]
pub struct TextWrapStyleValue;

/// Represents the style value for `hyphenate-character` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#hyphenate-character).
///
/// The hyphenate-character CSS property sets the character or string to use at the end of a line before a line break.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <string>
/// ```
///
// https://drafts.csswg.org/css-text-4/#hyphenate-character
#[value(" auto | <string> ")]
#[initial("auto")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:106,chrome_android:106,edge:106,firefox:98,firefox_android:98,safari:17,safari_ios:17)]
pub struct HyphenateCharacterStyleValue;

/// Represents the style value for `hyphenate-limit-zone` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#hyphenate-limit-zone).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-text-4/#hyphenate-limit-zone
#[value(" <length-percentage> ")]
#[initial("0")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("refers to length of the line box")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct HyphenateLimitZoneStyleValue;

// /// Represents the style value for `hyphenate-limit-chars` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#hyphenate-limit-chars).
// ///
// /// The hyphenate-limit-chars CSS property sets the number of characters in a word before it is hyphenated and the minimum number of characters on either side of the hyphen.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ auto | <integer> ]{1,3}
// /// ```
// ///
// // https://drafts.csswg.org/css-text-4/#hyphenate-limit-chars
// #[value(" [ auto | <integer> ]{1,3} ")]
// #[initial("auto")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(limited)]
// #[versions(chrome:109,chrome_android:109,edge:109,firefox:137,firefox_android:137)]
// pub struct HyphenateLimitCharsStyleValue;

/// Represents the style value for `hyphenate-limit-lines` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#hyphenate-limit-lines).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// no-limit | <integer>
/// ```
///
// https://drafts.csswg.org/css-text-4/#hyphenate-limit-lines
#[value(" no-limit | <integer> ")]
#[initial("no-limit")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum HyphenateLimitLinesStyleValue {}

/// Represents the style value for `hyphenate-limit-last` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#hyphenate-limit-last).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | always | column | page | spread
/// ```
///
// https://drafts.csswg.org/css-text-4/#hyphenate-limit-last
#[value(" none | always | column | page | spread ")]
#[initial("none")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum HyphenateLimitLastStyleValue {}

/// Represents the style value for `text-group-align` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-group-align).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | start | end | left | right | center
/// ```
///
// https://drafts.csswg.org/css-text-4/#text-group-align
#[value(" none | start | end | left | right | center ")]
#[initial("none")]
#[applies_to("block containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum TextGroupAlignStyleValue {}

/// Represents the style value for `line-padding` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#line-padding).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-text-4/#line-padding
#[value(" <length> ")]
#[initial("0")]
#[applies_to("inline boxes")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct LinePaddingStyleValue;

/// Represents the style value for `text-autospace` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-autospace).
///
/// The text-autospace CSS property sets whether and how to insert spaces in inter-script text (such as when mixing Latin and Chinese characters) and around punctuation.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <autospace> | auto
/// ```
///
// https://drafts.csswg.org/css-text-4/#text-autospace
#[value(" normal | <autospace> | auto ")]
#[initial("normal")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(safari:18.4,safari_ios:18.4)]
pub enum TextAutospaceStyleValue {}

/// Represents the style value for `text-spacing-trim` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-spacing-trim).
///
/// The text-spacing-trim CSS property controls spacing around CJK characters, avoiding excessive whitespace when using full-width punctuation characters.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <spacing-trim> | auto
/// ```
///
// https://drafts.csswg.org/css-text-4/#text-spacing-trim
#[value(" <spacing-trim> | auto ")]
#[initial("normal")]
#[applies_to("text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:123,chrome_android:123,edge:123)]
pub struct TextSpacingTrimStyleValue;

// /// Represents the style value for `text-spacing` as defined in [css-text-4](https://drafts.csswg.org/css-text-4/#text-spacing).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | auto | <spacing-trim> || <autospace>
// /// ```
// ///
// // https://drafts.csswg.org/css-text-4/#text-spacing
// #[value(" none | auto | <spacing-trim> || <autospace> ")]
// #[initial("see individual properties")]
// #[applies_to("text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum TextSpacingStyleValue {}
