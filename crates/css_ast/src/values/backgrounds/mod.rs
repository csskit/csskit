#![allow(warnings)]
//! CSS Backgrounds Module Level 4
//! https://drafts.csswg.org/css-backgrounds-4/

mod impls;
use impls::*;

/// Represents the style value for `background-color` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-color).
///
/// The background-color CSS property sets the fill color of an element, behind any content and background images or gradients.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <color>
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-color
#[value(" <color> ")]
#[initial("transparent")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
pub struct BackgroundColorStyleValue;

/// Represents the style value for `background-image` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-image).
///
/// The background-image CSS property sets the graphics to display behind the content of an element and in front of the background color. Graphics may be any combination of images or gradients.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <bg-image>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-image
#[value(" <bg-image># ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:3.6,firefox_android:4,safari:1.3,safari_ios:1)]
pub struct BackgroundImageStyleValue<'a>;

/// Represents the style value for `background-repeat` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-repeat).
///
/// The background-repeat CSS property sets how a background image is tiled.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <repeat-style>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-repeat
#[value(" <repeat-style># ")]
#[initial("repeat")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/background-repeat-round-space")]
#[baseline(widely)]
#[versions(chrome:30,chrome_android:30,edge:12,firefox:49,firefox_android:49,safari:8,safari_ios:8)]
pub struct BackgroundRepeatStyleValue<'a>;

/// Represents the style value for `background-attachment` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-attachment).
///
/// The background-attachment CSS property sets whether an element's background image or gradient moves as the element scrolls.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <attachment>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-attachment
#[value(" <attachment># ")]
#[initial("scroll")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/background-attachment")]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:25,firefox_android:25,safari:15.4,safari_ios:15.4)]
pub struct BackgroundAttachmentStyleValue<'a>;

// /// Represents the style value for `background-position` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-position).
// ///
// /// The background-position CSS property offsets the initial position of background images relative to the background origin.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <bg-position>#
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#background-position
// #[value(" <bg-position># ")]
// #[initial("0% 0%")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to size of background positioning area minus size of background image; see text")]
// #[canonical_order("per grammar")]
// #[animation_type("repeatable list")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/css-background-offsets")]
// #[baseline(widely)]
// #[versions(chrome:25,chrome_android:25,edge:12,firefox:13,firefox_android:14,safari:7,safari_ios:7)]
// pub struct BackgroundPositionStyleValue<'a>;

/// Represents the style value for `background-clip` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-clip).
///
/// The background-clip CSS property sets the extent of the background: the padding box, the content box, or the default border box.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <bg-clip>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-clip
#[value(" <bg-clip># ")]
#[initial("border-box")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("repeatable list")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:21,chrome_android:25,edge:12,firefox:22,firefox_android:22,safari:5.1,safari_ios:5)]
pub struct BackgroundClipStyleValue<'a>;

/// Represents the style value for `background-origin` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-origin).
///
/// The background-origin CSS property sets the background starting position relative to the border and padding of an element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-origin
#[value(" <visual-box># ")]
#[initial("padding-box")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("repeatable list")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:21,chrome_android:25,edge:12,firefox:22,firefox_android:22,safari:5.1,safari_ios:4)]
pub struct BackgroundOriginStyleValue<'a>;

/// Represents the style value for `background-size` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-size).
///
/// The background-size CSS property scales or stretches a background based on the size of the element (with the contain and cover keywords), a length, or percentage.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <bg-size>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-size
#[value(" <bg-size># ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("see text")]
#[canonical_order("per grammar")]
#[animation_type("repeatable list")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:21,chrome_android:25,edge:12,firefox:9,firefox_android:18,safari:5.1,safari_ios:4.2)]
pub struct BackgroundSizeStyleValue<'a>;

// /// Represents the style value for `background` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background).
// ///
// /// The background CSS property is a shorthand that sets several background properties at once.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <bg-layer>#? , <final-bg-layer>
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#background
// #[value(" <bg-layer>#? , <final-bg-layer> ")]
// #[initial("see individual properties")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("see individual properties")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/multibackgrounds")]
// #[baseline(widely)]
// #[versions(chrome:1,chrome_android:18,edge:12,firefox:3.6,firefox_android:4,safari:1.3,safari_ios:1)]
// pub struct BackgroundStyleValue<'a>;

/// Represents the style value for `border-image-source` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#border-image-source).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <image>
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#border-image-source
#[value(" none | <image> ")]
#[initial("none")]
#[applies_to("All elements, except internal table elements when border-collapse is collapse")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum BorderImageSourceStyleValue<'a> {}

// /// Represents the style value for `border-image-slice` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#border-image-slice).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [<number [0,∞]> | <percentage [0,∞]>]{1,4} && fill?
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#border-image-slice
// #[value(" [<number [0,∞]> | <percentage [0,∞]>]{1,4} && fill? ")]
// #[initial("100%")]
// #[applies_to("All elements, except internal table elements when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("refer to size of the border image")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct BorderImageSliceStyleValue;

// /// Represents the style value for `border-image-width` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#border-image-width).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length-percentage [0,∞]> | <number [0,∞]> | auto ]{1,4}
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#border-image-width
// #[value(" [ <length-percentage [0,∞]> | <number [0,∞]> | auto ]{1,4} ")]
// #[initial("1")]
// #[applies_to("All elements, except internal table elements when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("relative to width/height of the border image area")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct BorderImageWidthStyleValue;

// /// Represents the style value for `border-image-outset` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#border-image-outset).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ <length [0,∞]> | <number [0,∞]> ]{1,4}
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#border-image-outset
// #[value(" [ <length [0,∞]> | <number [0,∞]> ]{1,4} ")]
// #[initial("0")]
// #[applies_to("All elements, except internal table elements when border-collapse is collapse")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct BorderImageOutsetStyleValue;

/// Represents the style value for `border-image-repeat` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#border-image-repeat).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// [ stretch | repeat | round | space ]{1,2}
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#border-image-repeat
#[value(" [ stretch | repeat | round | space ]{1,2} ")]
#[initial("stretch")]
#[applies_to("All elements, except internal table elements when border-collapse is collapse")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct BorderImageRepeatStyleValue;

// /// Represents the style value for `border-image` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#border-image).
// ///
// /// The border-image CSS property draws an image around an element.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// <'border-image-source'> || <'border-image-slice'> [ / <'border-image-width'> | / <'border-image-width'>? / <'border-image-outset'> ]? || <'border-image-repeat'>
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#border-image
// #[value(
// 	" <'border-image-source'> || <'border-image-slice'> [ / <'border-image-width'> | / <'border-image-width'>? / <'border-image-outset'> ]? || <'border-image-repeat'> "
// )]
// #[initial("See individual properties")]
// #[applies_to("See individual properties")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/border-image")]
// #[baseline(widely)]
// #[versions(chrome:56,chrome_android:56,edge:12,firefox:50,firefox_android:50,safari:9.1,safari_ios:9.3)]
// pub struct BorderImageStyleValue;

/// Represents the style value for `background-repeat-x` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-repeat-x).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <repetition>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-repeat-x
#[value(" <repetition># ")]
#[initial("repeat")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct BackgroundRepeatXStyleValue<'a>;

/// Represents the style value for `background-repeat-y` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-repeat-y).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <repetition>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-repeat-y
#[value(" <repetition># ")]
#[initial("repeat")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct BackgroundRepeatYStyleValue<'a>;

/// Represents the style value for `background-repeat-block` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-repeat-block).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <repetition>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-repeat-block
#[value(" <repetition># ")]
#[initial("repeat")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct BackgroundRepeatBlockStyleValue<'a>;

/// Represents the style value for `background-repeat-inline` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-repeat-inline).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <repetition>#
/// ```
///
// https://drafts.csswg.org/css-backgrounds-4/#background-repeat-inline
#[value(" <repetition># ")]
#[initial("repeat")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct BackgroundRepeatInlineStyleValue<'a>;

// /// Represents the style value for `background-position-x` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-position-x).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ center | [ [ left | right | x-start | x-end ]? <length-percentage>? ]! ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#background-position-x
// #[value(" [ center | [ [ left | right | x-start | x-end ]? <length-percentage>? ]! ]# ")]
// #[initial("0%")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to width of background positioning area minus width of background image")]
// #[canonical_order("per grammar")]
// #[animation_type("repeatable list")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum BackgroundPositionXStyleValue<'a> {}

// /// Represents the style value for `background-position-y` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-position-y).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ center | [ [ top | bottom | y-start | y-end ]? <length-percentage>? ]! ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#background-position-y
// #[value(" [ center | [ [ top | bottom | y-start | y-end ]? <length-percentage>? ]! ]# ")]
// #[initial("0%")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to height of background positioning area minus height of background image")]
// #[canonical_order("per grammar")]
// #[animation_type("repeatable list")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum BackgroundPositionYStyleValue<'a> {}

// /// Represents the style value for `background-position-inline` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-position-inline).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ center | [ [ start | end ]? <length-percentage>? ]! ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#background-position-inline
// #[value(" [ center | [ [ start | end ]? <length-percentage>? ]! ]# ")]
// #[initial("0%")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to inline-size of background positioning area minus inline-size of background image")]
// #[canonical_order("per grammar")]
// #[animation_type("repeatable list")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum BackgroundPositionInlineStyleValue<'a> {}

// /// Represents the style value for `background-position-block` as defined in [css-backgrounds-4](https://drafts.csswg.org/css-backgrounds-4/#background-position-block).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ center | [ [ start | end ]? <length-percentage>? ]! ]#
// /// ```
// ///
// // https://drafts.csswg.org/css-backgrounds-4/#background-position-block
// #[value(" [ center | [ [ start | end ]? <length-percentage>? ]! ]# ")]
// #[initial("0%")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("refer to size of background positioning area minus size of background image")]
// #[canonical_order("per grammar")]
// #[animation_type("repeatable list")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub enum BackgroundPositionBlockStyleValue<'a> {}
