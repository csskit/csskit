#![allow(warnings)]
//! CSS Images Module Level 5
//! https://drafts.csswg.org/css-images-5/

mod impls;
use impls::*;

// /// Represents the style value for `object-fit` as defined in [css-images-5](https://drafts.csswg.org/css-images-5/#object-fit).
// ///
// /// The object-fit CSS property sets how images, videos, and other replaced elements are scaled within their container.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// fill | none | [contain | cover] || scale-down
// /// ```
// ///
// // https://drafts.csswg.org/css-images-5/#object-fit
// #[value(" fill | none | [contain | cover] || scale-down ")]
// #[initial("fill")]
// #[applies_to("replaced elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/object-fit")]
// #[baseline(widely)]
// #[versions(chrome:32,chrome_android:32,edge:79,firefox:36,firefox_android:36,safari:10,safari_ios:10)]
// pub enum ObjectFitStyleValue {}

/// Represents the style value for `object-position` as defined in [css-images-5](https://drafts.csswg.org/css-images-5/#object-position).
///
/// The object-position CSS property places images, videos, and other replaced elements within their boxes.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <position>
/// ```
///
// https://drafts.csswg.org/css-images-5/#object-position
#[value(" <position> ")]
#[initial("50% 50%")]
#[applies_to("replaced elements")]
#[inherited("no")]
#[percentages("refer to width and height of element itself")]
#[canonical_order("the horizontal component of the <position>, followed by the vertical component")]
#[animation_type("as for background-position")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:32,chrome_android:32,edge:79,firefox:36,firefox_android:36,safari:10,safari_ios:10)]
pub struct ObjectPositionStyleValue;

// /// Represents the style value for `image-orientation` as defined in [css-images-5](https://drafts.csswg.org/css-images-5/#image-orientation).
// ///
// /// The image-orientation CSS property corrects the rotation of an image using the image's metadata, such as EXIF.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// from-image | none | [ <angle> || flip ]
// /// ```
// ///
// // https://drafts.csswg.org/css-images-5/#image-orientation
// #[value(" from-image | none | [ <angle> || flip ] ")]
// #[initial("from-image")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse("https://caniuse.com/css-image-orientation")]
// #[baseline(widely)]
// #[versions(chrome:81,chrome_android:81,edge:81,firefox:26,firefox_android:26,safari:13.1,safari_ios:13.4)]
// pub enum ImageOrientationStyleValue {}

/// Represents the style value for `image-rendering` as defined in [css-images-5](https://drafts.csswg.org/css-images-5/#image-rendering).
///
/// The image-rendering CSS property sets how images are scaled, retaining smoothness for photos, or hard edges for pixel art and QR codes.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | smooth | high-quality | pixelated | crisp-edges
/// ```
///
// https://drafts.csswg.org/css-images-5/#image-rendering
#[value(" auto | smooth | high-quality | pixelated | crisp-edges ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-crisp-edges")]
#[baseline(widely)]
#[versions(chrome:41,chrome_android:41,edge:79,firefox:93,firefox_android:93,safari:10,safari_ios:10)]
pub enum ImageRenderingStyleValue {}

// /// Represents the style value for `image-resolution` as defined in [css-images-5](https://drafts.csswg.org/css-images-5/#image-resolution).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ from-image || <resolution> ] && snap?
// /// ```
// ///
// // https://drafts.csswg.org/css-images-5/#image-resolution
// #[value(" [ from-image || <resolution> ] && snap? ")]
// #[initial("1dppx")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// pub struct ImageResolutionStyleValue;

/// Represents the style value for `object-view-box` as defined in [css-images-5](https://drafts.csswg.org/css-images-5/#object-view-box).
///
/// The object-view-box CSS property crops and zooms to an inset area of an image.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <basic-shape-rect>
/// ```
///
// https://drafts.csswg.org/css-images-5/#object-view-box
#[value(" none | <basic-shape-rect> ")]
#[initial("none")]
#[applies_to("replaced elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("as  if possible, otherwise discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:104,chrome_android:104,edge:104)]
pub struct ObjectViewBoxStyleValue;
