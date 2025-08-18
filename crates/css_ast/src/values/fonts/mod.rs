#![allow(warnings)]
//! CSS Fonts Module Level 5
//! https://drafts.csswg.org/css-fonts-5/

mod impls;
use impls::*;

// /// Represents the style value for `font-family` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-family).
// ///
// /// The font-family CSS property sets the desired font face for text, along with optional fallback font faces.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <family-name> | <generic-family> ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-fonts-5/#font-family
// #[value(" [ <family-name> | <generic-family> ]# ")]
// #[initial("depends on user agent")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
// pub struct FontFamilyStyleValue<'a>;

/// Represents the style value for `font-weight` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-weight).
///
/// The font-weight CSS property controls the thickness of a font. It is set explicitly with the keyword bold or a number, or relative to the inherited thickness with the keywords bolder or lighter.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <font-weight-absolute> | bolder | lighter
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-weight
#[value(" <font-weight-absolute> | bolder | lighter ")]
#[initial("normal")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:2,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
pub enum FontWeightStyleValue {}

/// Represents the style value for `font-width` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-width).
///
/// The font-width CSS property selects a font face from a font family based on width, either by a keyword such as condensed or a percentage.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <percentage [0,∞]> | ultra-condensed | extra-condensed | condensed | semi-condensed | semi-expanded | expanded | extra-expanded | ultra-expanded
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-width
#[value(
	" normal | <percentage [0,∞]> | ultra-condensed | extra-condensed | condensed | semi-condensed | semi-expanded | expanded | extra-expanded | ultra-expanded "
)]
#[initial("normal")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("not resolved")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(safari:18.4)]
pub enum FontWidthStyleValue {}

/// Represents the style value for `font-style` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-style).
///
/// The font-style CSS property sets the text style, with normal, italic, and oblique options.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | italic | left | right | oblique <angle [-90deg,90deg]>?
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-style
#[value(" normal | italic | left | right | oblique <angle [-90deg,90deg]>? ")]
#[initial("normal")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type;normal animates as oblique 0deg")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
pub enum FontStyleStyleValue {}

/// Represents the style value for `font-size` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-size).
///
/// The font-size CSS property sets the text height.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <absolute-size> | <relative-size> | <length-percentage [0,∞]> | math
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-size
#[value(" <absolute-size> | <relative-size> | <length-percentage [0,∞]> | math ")]
#[initial("medium")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("refer to parent element’s font size")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
pub enum FontSizeStyleValue {}

// /// Represents the style value for `font-size-adjust` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-size-adjust).
// ///
// /// The font-size-adjust CSS property preserves apparent text size, regardless of the font used, by scaling fonts to the same size with respect to a specific metric, such as x-height. This can help make fallback fonts look the same size.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ ex-height | cap-height | ch-width | ic-width | ic-height ]? [ from-font | <number [0,∞]> ]
// /// ```
// ///
// // https://drafts.csswg.org/css-fonts-5/#font-size-adjust
// #[value(" none | [ ex-height | cap-height | ch-width | ic-width | ic-height ]? [ from-font | <number [0,∞]> ] ")]
// #[initial("none")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete if the keywords differ, otherwise by computed value type")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/font-size-adjust")]
// #[baseline(newly)]
// #[versions(chrome:127,chrome_android:127,edge:127,firefox:118,firefox_android:118,safari:17,safari_ios:17)]
// pub enum FontSizeAdjustStyleValue {}

// /// Represents the style value for `font` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ [ <'font-style'> || <font-variant-css2> || <'font-weight'> || <font-width-css3> ]? <'font-size'> [ / <'line-height'> ]? <'font-family'># ] | <system-family-name>
// /// ```
// ///
// // https://drafts.csswg.org/css-fonts-5/#font
// #[value(
// 	" [ [ <'font-style'> || <font-variant-css2> || <'font-weight'> || <font-width-css3> ]? <'font-size'> [ / <'line-height'> ]? <'font-family'># ] | <system-family-name> "
// )]
// #[initial("see individual properties")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum FontStyleValue<'a> {}

/// Represents the style value for `font-synthesis-weight` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-synthesis-weight).
///
/// The font-synthesis-weight CSS property sets whether or not the browser should synthesize bold typefaces when they're missing from the font.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-synthesis-weight
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:97,chrome_android:97,edge:97,firefox:111,firefox_android:111,safari:16.4,safari_ios:16.4)]
pub enum FontSynthesisWeightStyleValue {}

/// Represents the style value for `font-synthesis-style` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-synthesis-style).
///
/// The font-synthesis-style CSS property sets whether or not the browser should synthesize italic and oblique typefaces when they're missing from the font.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none | oblique-only
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-synthesis-style
#[value(" auto | none | oblique-only ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:97,chrome_android:97,edge:97,firefox:111,firefox_android:111,safari:16.4,safari_ios:16.4)]
pub enum FontSynthesisStyleStyleValue {}

/// Represents the style value for `font-synthesis-small-caps` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-synthesis-small-caps).
///
/// The font-synthesis-small-caps CSS property sets whether or not the browser should synthesize small caps typefaces when they're missing from the font.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-synthesis-small-caps
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:97,chrome_android:97,edge:97,firefox:111,firefox_android:111,safari:16.4,safari_ios:16.4)]
pub enum FontSynthesisSmallCapsStyleValue {}

/// Represents the style value for `font-synthesis-position` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-synthesis-position).
///
/// The font-synthesis-position CSS property sets whether or not the browser should synthesize subscript and superscript typefaces when they're missing from the font.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-synthesis-position
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(firefox:118,firefox_android:118)]
pub enum FontSynthesisPositionStyleValue {}

// /// Represents the style value for `font-synthesis` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-synthesis).
// ///
// /// The font-synthesis CSS shorthand property disables all font synthesis except the given kinds. To disable a specific kind of font synthesis, instead use the longhand properties such as font-synthesis-style and font-synthesis-weight.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [ weight || style || small-caps || position]
// /// ```
// ///
// // https://drafts.csswg.org/css-fonts-5/#font-synthesis
// #[value(" none | [ weight || style || small-caps || position] ")]
// #[initial("weight style small-caps position")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:97,chrome_android:97,edge:97,firefox:34,firefox_android:34,safari:9,safari_ios:9)]
// pub enum FontSynthesisStyleValue {}

/// Represents the style value for `font-kerning` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-kerning).
///
/// The font-kerning CSS property sets whether kerning data from a font is used to adjust the space between letters.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | normal | none
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-kerning
#[value(" auto | normal | none ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/font-kerning")]
#[baseline(widely)]
#[versions(chrome:33,chrome_android:33,edge:79,firefox:32,firefox_android:32,safari:9,safari_ios:9)]
pub enum FontKerningStyleValue {}

// /// Represents the style value for `font-variant-ligatures` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-variant-ligatures).
// ///
// /// The font-variant-ligatures CSS property sets how characters can be visually combined for readability or stylistic reasons.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | none | [ <common-lig-values> || <discretionary-lig-values> || <historical-lig-values> || <contextual-alt-values> ]
// /// ```
// ///
// // https://drafts.csswg.org/css-fonts-5/#font-variant-ligatures
// #[value(
// 	" normal | none | [ <common-lig-values> || <discretionary-lig-values> || <historical-lig-values> || <contextual-alt-values> ] "
// )]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:34,chrome_android:34,edge:79,firefox:34,firefox_android:34,safari:9.1,safari_ios:9.3)]
// pub enum FontVariantLigaturesStyleValue {}

/// Represents the style value for `font-variant-position` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-variant-position).
///
/// The font-variant-position CSS property sets whether to use alternate glyphs for subscript and superscript text.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | sub | super
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-variant-position
#[value(" normal | sub | super ")]
#[initial("normal")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:117,edge:117,firefox:34,firefox_android:34,safari:9.1,safari_ios:9.3)]
pub enum FontVariantPositionStyleValue {}

/// Represents the style value for `font-variant-caps` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-variant-caps).
///
/// The font-variant-caps CSS property sets whether text should be displayed in small caps, petite caps, or with capital letters designed for titles.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | small-caps | all-small-caps | petite-caps | all-petite-caps | unicase | titling-caps
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-variant-caps
#[value(" normal | small-caps | all-small-caps | petite-caps | all-petite-caps | unicase | titling-caps ")]
#[initial("normal")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:52,chrome_android:52,edge:79,firefox:34,firefox_android:34,safari:9.1,safari_ios:9.3)]
pub enum FontVariantCapsStyleValue {}

// /// Represents the style value for `font-variant-numeric` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-variant-numeric).
// ///
// /// The font-variant-numeric CSS property sets how numeric characters are displayed. For example, you can align columns of numbers or use zeroes that have a slash.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | [ <numeric-figure-values> || <numeric-spacing-values> || <numeric-fraction-values> || ordinal || slashed-zero ]
// /// ```
// ///
// // https://drafts.csswg.org/css-fonts-5/#font-variant-numeric
// #[value(
// 	" normal | [ <numeric-figure-values> || <numeric-spacing-values> || <numeric-fraction-values> || ordinal || slashed-zero ] "
// )]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/font-variant-numeric")]
// #[baseline(widely)]
// #[versions(chrome:52,chrome_android:52,edge:79,firefox:34,firefox_android:34,safari:9.1,safari_ios:9.3)]
// pub enum FontVariantNumericStyleValue {}

// /// Represents the style value for `font-variant-alternates` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-variant-alternates).
// ///
// /// The font-variant-alternates CSS property, along with the @font-feature-values at-rule, chooses when to use a font's alternate glyphs.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | [ stylistic(<feature-value-name>) || historical-forms || styleset(<feature-value-name>#) || character-variant(<feature-value-name>#) || swash(<feature-value-name>) || ornaments(<feature-value-name>) || annotation(<feature-value-name>) ]
// /// ```
// ///
// // https://drafts.csswg.org/css-fonts-5/#font-variant-alternates
// #[value(
// 	" normal | [ stylistic(<feature-value-name>) || historical-forms || styleset(<feature-value-name>#) || character-variant(<feature-value-name>#) || swash(<feature-value-name>) || ornaments(<feature-value-name>) || annotation(<feature-value-name>) ] "
// )]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/font-variant-alternates")]
// #[baseline(newly)]
// #[versions(chrome:111,chrome_android:111,edge:111,firefox:34,firefox_android:34,safari:9.1,safari_ios:9.3)]
// pub enum FontVariantAlternatesStyleValue<'a> {}

// /// Represents the style value for `font-variant-east-asian` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-variant-east-asian).
// ///
// /// The font-variant-east-asian CSS property controls glyph substitution and sizing in East Asian text.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | [ <east-asian-variant-values> || <east-asian-width-values> || ruby ]
// /// ```
// ///
// // https://drafts.csswg.org/css-fonts-5/#font-variant-east-asian
// #[value(" normal | [ <east-asian-variant-values> || <east-asian-width-values> || ruby ] ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:63,chrome_android:63,edge:79,firefox:34,firefox_android:34,safari:9.1,safari_ios:9.3)]
// pub enum FontVariantEastAsianStyleValue {}

// /// Represents the style value for `font-variant` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-variant).
// ///
// /// The font-variant CSS property is a shorthand for font-variant-alternates, font-variant-caps, font-variant-east-asian, font-variant-emoji, font-variant-ligatures, font-variant-numeric, and font-variant-position.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | none | [ [ <common-lig-values> || <discretionary-lig-values> || <historical-lig-values> || <contextual-alt-values> ] || [ small-caps | all-small-caps | petite-caps | all-petite-caps | unicase | titling-caps ] || [ stylistic(<feature-value-name>) || historical-forms || styleset(<feature-value-name>#) || character-variant(<feature-value-name>#) || swash(<feature-value-name>) || ornaments(<feature-value-name>) || annotation(<feature-value-name>) ] || [ <numeric-figure-values> || <numeric-spacing-values> || <numeric-fraction-values> || ordinal || slashed-zero ] || [ <east-asian-variant-values> || <east-asian-width-values> || ruby ] || [ sub | super ] || [ text | emoji | unicode ] ]
// /// ```
// ///
// // https://drafts.csswg.org/css-fonts-5/#font-variant
// #[value(
// 	" normal | none | [ [ <common-lig-values> || <discretionary-lig-values> || <historical-lig-values> || <contextual-alt-values> ] || [ small-caps | all-small-caps | petite-caps | all-petite-caps | unicase | titling-caps ] || [ stylistic(<feature-value-name>) || historical-forms || styleset(<feature-value-name>#) || character-variant(<feature-value-name>#) || swash(<feature-value-name>) || ornaments(<feature-value-name>) || annotation(<feature-value-name>) ] || [ <numeric-figure-values> || <numeric-spacing-values> || <numeric-fraction-values> || ordinal || slashed-zero ] || [ <east-asian-variant-values> || <east-asian-width-values> || ruby ] || [ sub | super ] || [ text | emoji | unicode ] ] "
// )]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(widely)]
// #[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
// pub enum FontVariantStyleValue<'a> {}

// /// Represents the style value for `font-feature-settings` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-feature-settings).
// ///
// /// The font-feature-settings CSS property sets low-level OpenType feature tags for a font. When possible, use font-variant instead.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | <feature-tag-value>#
// /// ```
// ///
// // https://drafts.csswg.org/css-fonts-5/#font-feature-settings
// #[value(" normal | <feature-tag-value># ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/font-feature")]
// #[baseline(widely)]
// #[versions(chrome:48,chrome_android:48,edge:15,firefox:34,firefox_android:34,safari:9.1,safari_ios:9.3)]
// pub enum FontFeatureSettingsStyleValue<'a> {}

/// Represents the style value for `font-language-override` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-language-override).
///
/// The font-language-override CSS property sets which language-specific glyphs are displayed.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <string>
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-language-override
#[value(" normal | <string> ")]
#[initial("normal")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(firefox:34,firefox_android:34)]
pub enum FontLanguageOverrideStyleValue {}

/// Represents the style value for `font-optical-sizing` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-optical-sizing).
///
/// The font-optical-sizing CSS property sets whether text rendering is optimized for viewing at different sizes.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-optical-sizing
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:79,chrome_android:79,edge:17,firefox:62,firefox_android:62,safari:13.1,safari_ios:13.4)]
pub enum FontOpticalSizingStyleValue {}

// /// Represents the style value for `font-variation-settings` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-variation-settings).
// ///
// /// The font-variation-settings CSS property sets an "axis of variability" on a variable font, such as weight, optical size, or a custom axis defined by the typeface designer. When possible, use other CSS font properties, such as font-weight: bold. Also known as variable fonts.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | [ <opentype-tag> <number> ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-fonts-5/#font-variation-settings
// #[value(" normal | [ <opentype-tag> <number> ]# ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see prose")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/variable-fonts")]
// #[baseline(widely)]
// #[versions(chrome:62,chrome_android:62,edge:17,firefox:62,firefox_android:62,safari:11,safari_ios:11)]
// pub enum FontVariationSettingsStyleValue<'a> {}

// /// Represents the style value for `font-palette` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-palette).
// ///
// /// The font-palette CSS property selects a color palette from the font, optionally overriding individual colors in the @font-palette-values at-rule.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// normal | light | dark | <palette-identifier> | <palette-mix()>
// /// ```
// ///
// // https://drafts.csswg.org/css-fonts-5/#font-palette
// #[value(" normal | light | dark | <palette-identifier> | <palette-mix()> ")]
// #[initial("normal")]
// #[applies_to("all elements and text")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/css-font-palette")]
// #[baseline(widely)]
// #[versions(chrome:101,chrome_android:101,edge:101,firefox:107,firefox_android:107,safari:15.4,safari_ios:15.4)]
// pub enum FontPaletteStyleValue<'a> {}

/// Represents the style value for `font-variant-emoji` as defined in [css-fonts-5](https://drafts.csswg.org/css-fonts-5/#font-variant-emoji).
///
/// The font-variant-emoji CSS property sets the default presentation for emoji characters.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | text | emoji | unicode
/// ```
///
// https://drafts.csswg.org/css-fonts-5/#font-variant-emoji
#[value(" normal | text | emoji | unicode ")]
#[initial("normal")]
#[applies_to("all elements and text")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:131,chrome_android:131,edge:131,firefox:141,firefox_android:141)]
pub enum FontVariantEmojiStyleValue {}
