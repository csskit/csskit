#![allow(warnings)]
//! CSS Basic User Interface Module Level 4
//! https://drafts.csswg.org/css-ui-4/

mod impls;
use impls::*;

/// Represents the style value for `outline` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#outline).
///
/// The outline CSS shorthand sets the color, style, and width of a line around an element, outside of the border.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'outline-width'> || <'outline-style'> || <'outline-color'>
/// ```
///
// https://drafts.csswg.org/css-ui-4/#outline
#[syntax(" <'outline-width'> || <'outline-style'> || <'outline-color'> ")]
#[initial("see individual properties")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:94,chrome_android:94,edge:94,firefox:88,firefox_android:88,safari:16.4,safari_ios:16.4)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct OutlineStyleValue<'a>;

/// Represents the style value for `outline-width` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#outline-width).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <line-width>
/// ```
///
// https://drafts.csswg.org/css-ui-4/#outline-width
#[syntax(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct OutlineWidthStyleValue;

/// Represents the style value for `outline-style` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#outline-style).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <outline-line-style>
/// ```
///
// https://drafts.csswg.org/css-ui-4/#outline-style
#[syntax(" auto | <outline-line-style> ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct OutlineStyleStyleValue;

/// Represents the style value for `outline-color` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#outline-color).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <color> | <image-1D>
/// ```
///
// https://drafts.csswg.org/css-ui-4/#outline-color
#[syntax(" auto | <color> | <image-1D> ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum OutlineColorStyleValue<'a> {}

/// Represents the style value for `outline-offset` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#outline-offset).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <length>
/// ```
///
// https://drafts.csswg.org/css-ui-4/#outline-offset
#[syntax(" <length> ")]
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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct OutlineOffsetStyleValue;

/// Represents the style value for `resize` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#resize).
///
/// The resize CSS property sets whether an element can be resized by the user, and on which axes.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | both | horizontal | vertical | block | inline
/// ```
///
// https://drafts.csswg.org/css-ui-4/#resize
#[syntax(" none | both | horizontal | vertical | block | inline ")]
#[initial("none")]
#[applies_to(
	"elements that are scroll containers and optionally replaced elements such as images, videos, and iframes"
)]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-resize")]
#[baseline(limited)]
#[versions(chrome:4,chrome_android:18,edge:79,firefox:5,firefox_android:5,safari:4)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum ResizeStyleValue {}

/// Represents the style value for `cursor` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#cursor).
///
/// The cursor CSS property styles the pointer, allowing you to provide hints to the user on how to interact with the hovered element.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <cursor-image>#? <cursor-predefined>
/// ```
///
// https://drafts.csswg.org/css-ui-4/#cursor
#[syntax(" <cursor-image>#? <cursor-predefined> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css3-cursors")]
#[baseline(limited)]
#[versions(chrome:68,chrome_android:68,edge:79,firefox:27,firefox_android:95,safari:11)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CursorStyleValue<'a>;

/// Represents the style value for `caret-color` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#caret-color).
///
/// The caret-color CSS property sets the color of the text insertion pointer in a text input.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <color>
/// ```
///
// https://drafts.csswg.org/css-ui-4/#caret-color
#[syntax(" auto | <color> ")]
#[initial("auto")]
#[applies_to("text or elements that accept text input")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-caret-color")]
#[baseline(widely)]
#[versions(chrome:57,chrome_android:57,edge:79,firefox:53,firefox_android:53,safari:11.1,safari_ios:11.3)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CaretColorStyleValue;

/// Represents the style value for `caret-animation` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#caret-animation).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | manual
/// ```
///
// https://drafts.csswg.org/css-ui-4/#caret-animation
#[syntax(" auto | manual ")]
#[initial("auto")]
#[applies_to("text or elements that accept text input")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum CaretAnimationStyleValue {}

/// Represents the style value for `caret-shape` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#caret-shape).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | bar | block | underscore
/// ```
///
// https://drafts.csswg.org/css-ui-4/#caret-shape
#[syntax(" auto | bar | block | underscore ")]
#[initial("auto")]
#[applies_to("text or elements that accept text input")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum CaretShapeStyleValue {}

/// Represents the style value for `caret` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#caret).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'caret-color'> || <'caret-animation'> || <'caret-shape'>
/// ```
///
// https://drafts.csswg.org/css-ui-4/#caret
#[syntax(" <'caret-color'> || <'caret-animation'> || <'caret-shape'> ")]
#[initial("auto")]
#[applies_to("text or elements that accept text input")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CaretStyleValue;

// /// Represents the style value for `nav-up` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#nav-up).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | <id> [ current | root | <target-name> ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-ui-4/#nav-up
// #[syntax(" auto | <id> [ current | root | <target-name> ]? ")]
// #[initial("auto")]
// #[applies_to("all enabled elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum NavUpStyleValue {}

// /// Represents the style value for `nav-right` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#nav-right).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | <id> [ current | root | <target-name> ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-ui-4/#nav-right
// #[syntax(" auto | <id> [ current | root | <target-name> ]? ")]
// #[initial("auto")]
// #[applies_to("all enabled elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum NavRightStyleValue {}

// /// Represents the style value for `nav-down` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#nav-down).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | <id> [ current | root | <target-name> ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-ui-4/#nav-down
// #[syntax(" auto | <id> [ current | root | <target-name> ]? ")]
// #[initial("auto")]
// #[applies_to("all enabled elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum NavDownStyleValue {}

// /// Represents the style value for `nav-left` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#nav-left).
// ///
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | <id> [ current | root | <target-name> ]?
// /// ```
// ///
// // https://drafts.csswg.org/css-ui-4/#nav-left
// #[syntax(" auto | <id> [ current | root | <target-name> ]? ")]
// #[initial("auto")]
// #[applies_to("all enabled elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(Unknown)]
// #[caniuse(Unknown)]
// #[baseline(Unknown)]
// #[versions(Unknown)]
// #[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[visit]
// pub enum NavLeftStyleValue {}

/// Represents the style value for `user-select` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#user-select).
///
/// The user-select CSS property controls which elements can be selected by the user.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | text | none | contain | all
/// ```
///
// https://drafts.csswg.org/css-ui-4/#user-select
#[syntax(" auto | text | none | contain | all ")]
#[initial("auto")]
#[applies_to("all elements, and optionally to the ::before and ::after pseudo-elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/user-select-none")]
#[baseline(limited)]
#[versions(chrome:54,chrome_android:54,edge:79,firefox:69,firefox_android:79)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum UserSelectStyleValue {}

/// Represents the style value for `pointer-events` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#pointer-events).
///
/// The pointer-events CSS property sets whether a user can interact with an element using a mouse, touch, or other pointing input device.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | none
/// ```
///
// https://drafts.csswg.org/css-ui-4/#pointer-events
#[syntax(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/pointer-events")]
#[baseline(widely)]
#[versions(chrome:2,chrome_android:18,edge:12,firefox:3.6,firefox_android:4,safari:4,safari_ios:3.2)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum PointerEventsStyleValue {}

/// Represents the style value for `interactivity` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#interactivity).
///
/// The interactivity: inert CSS declaration makes an element and its descendants inert, like when using the inert HTML attribute. Inert elements can't be focused or clicked, their text can't be selected or found using the browser's find-in-page feature.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | inert
/// ```
///
// https://drafts.csswg.org/css-ui-4/#interactivity
#[syntax(" auto | inert ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:135,edge:135)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum InteractivityStyleValue {}

/// Represents the style value for `interest-delay-start` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#interest-delay-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <time>
/// ```
///
// https://drafts.csswg.org/css-ui-4/#interest-delay-start
#[syntax(" normal | <time> ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum InterestDelayStartStyleValue {}

/// Represents the style value for `interest-delay-end` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#interest-delay-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// normal | <time>
/// ```
///
// https://drafts.csswg.org/css-ui-4/#interest-delay-end
#[syntax(" normal | <time> ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum InterestDelayEndStyleValue {}

/// Represents the style value for `interest-delay` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#interest-delay).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'interest-delay-start'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-ui-4/#interest-delay
#[syntax(" <'interest-delay-start'>{1,2} ")]
#[initial("see individual properties")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct InterestDelayStyleValue;

/// Represents the style value for `accent-color` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#accent-color).
///
/// The accent-color CSS property sets a color for checkboxes, radio buttons, and other form controls.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <color>
/// ```
///
// https://drafts.csswg.org/css-ui-4/#accent-color
#[syntax(" auto | <color> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:93,edge:93,firefox:92,firefox_android:92,safari:15.4)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct AccentColorStyleValue;

/// Represents the style value for `appearance` as defined in [css-ui-4](https://drafts.csswg.org/css-ui-4/#appearance).
///
/// The appearance CSS property controls the appearance of form controls. Using appearance: none disables any default native appearance and allows the elements to be styled with CSS.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | auto | base | base-select | <compat-auto> | <compat-special>
/// ```
///
// https://drafts.csswg.org/css-ui-4/#appearance
#[syntax(" none | auto | base | base-select | <compat-auto> | <compat-special> ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:84,chrome_android:84,edge:84,firefox:80,firefox_android:80,safari:15.4,safari_ios:15.4)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum AppearanceStyleValue {}
