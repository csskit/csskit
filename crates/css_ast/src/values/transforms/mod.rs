#![allow(warnings)]
//! CSS Transforms Module Level 2
//! https://drafts.csswg.org/css-transforms-2/

mod impls;
use impls::*;

/// Represents the style value for `transform` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#transform).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <transform-list>
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#transform
#[syntax(" none | <transform-list> ")]
#[initial("none")]
#[applies_to("transformable elements")]
#[inherited("no")]
#[percentages("refer to the size of reference box")]
#[canonical_order("per grammar")]
#[animation_type("transform list, see interpolation rules")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct TransformStyleValue<'a>;

// /// Represents the style value for `transform-origin` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#transform-origin).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ left | center | right | top | bottom | <length-percentage> ] |   [ left | center | right | <length-percentage> ]  [ top | center | bottom | <length-percentage> ] <length>? |  [ [ center | left | right ] && [ center | top | bottom ] ] <length>?
// /// ```
// ///
// // https://drafts.csswg.org/css-transforms-2/#transform-origin
// #[syntax(
// 	" [ left | center | right | top | bottom | <length-percentage> ] |   [ left | center | right | <length-percentage> ]  [ top | center | bottom | <length-percentage> ] <length>? |  [ [ center | left | right ] && [ center | top | bottom ] ] <length>? "
// )]
// #[initial("50% 50%")]
// #[applies_to("transformable elements")]
// #[inherited("no")]
// #[percentages("refer to the size of reference box")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum TransformOriginStyleValue {}

/// Represents the style value for `transform-box` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#transform-box).
///
/// The transform-box CSS property sets the position and dimensions of the reference box relative to which an element's transformations are calculated.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// content-box | border-box | fill-box | stroke-box | view-box
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#transform-box
#[syntax(" content-box | border-box | fill-box | stroke-box | view-box ")]
#[initial("view-box")]
#[applies_to("transformable elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:118,chrome_android:118,edge:118,firefox:125,firefox_android:125,safari:13.1,safari_ios:13.4)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum TransformBoxStyleValue {}

// /// Represents the style value for `translate` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#translate).
// ///
// /// The translate HTML attribute marks whether an element's text should be translated.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | <length-percentage> [ <length-percentage> <length>? ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-transforms-2/#translate
// #[syntax(" none | <length-percentage> [ <length-percentage> <length>? ]? ")]
// #[initial("none")]
// #[applies_to("transformable elements")]
// #[inherited("no")]
// #[percentages("relative to the width of the reference box (for the first value) or the height (for the second value)")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value, but see below for none")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(newly)]
// #[versions(chrome:19,chrome_android:25,edge:79,firefox:111,firefox_android:111,safari:6,safari_ios:6)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub struct TranslateStyleValue;

// /// Represents the style value for `rotate` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#rotate).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | <angle> | [ x | y | z | <number>{3} ] && <angle>
// /// ```
// ///
// // https://drafts.csswg.org/css-transforms-2/#rotate
// #[syntax(" none | <angle> | [ x | y | z | <number>{3} ] && <angle> ")]
// #[initial("none")]
// #[applies_to("transformable elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("as slerp, but see below for none")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum RotateStyleValue {}

/// Represents the style value for `scale` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#scale).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | [ <number> | <percentage> ]{1,3}
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#scale
#[syntax(" none | [ <number> | <percentage> ]{1,3} ")]
#[initial("none")]
#[applies_to("transformable elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value, but see below for none")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct ScaleStyleValue;

/// Represents the style value for `transform-style` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#transform-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// flat | preserve-3d
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#transform-style
#[syntax(" flat | preserve-3d ")]
#[initial("flat")]
#[applies_to("transformable elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum TransformStyleStyleValue {}

/// Represents the style value for `perspective` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#perspective).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#perspective
#[syntax(" none | <length [0,∞]> ")]
#[initial("none")]
#[applies_to("transformable elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct PerspectiveStyleValue;

/// Represents the style value for `perspective-origin` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#perspective-origin).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <position>
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#perspective-origin
#[syntax(" <position> ")]
#[initial("50% 50%")]
#[applies_to("transformable elements")]
#[inherited("no")]
#[percentages("refer to the size of the reference box")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct PerspectiveOriginStyleValue;

/// Represents the style value for `backface-visibility` as defined in [css-transforms-2](https://drafts.csswg.org/css-transforms-2/#backface-visibility).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// visible | hidden
/// ```
///
// https://drafts.csswg.org/css-transforms-2/#backface-visibility
#[syntax(" visible | hidden ")]
#[initial("visible")]
#[applies_to("transformable elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BackfaceVisibilityStyleValue {}
