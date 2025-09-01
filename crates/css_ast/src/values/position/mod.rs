#![allow(warnings)]
//! CSS Positioned Layout Module Level 4
//! https://drafts.csswg.org/css-position-4/

mod impls;
use impls::*;

/// Represents the style value for `position` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#position).
///
/// The position CSS property sets the origin position of an element to an element, the element's scrollport, or the viewport.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// static | relative | absolute | sticky | fixed
/// ```
///
// https://drafts.csswg.org/css-position-4/#position
#[syntax(" static | relative | absolute | sticky | fixed ")]
#[initial("static")]
#[applies_to("all elements except table-column-group and table-column")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(widely)]
#[versions(chrome:1,chrome_android:18,edge:12,firefox:1,firefox_android:4,safari:1,safari_ios:1)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum PositionStyleValue {}

/// Represents the style value for `top` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#top).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#top
#[syntax(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct TopStyleValue;

/// Represents the style value for `right` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#right).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#right
#[syntax(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct RightStyleValue;

/// Represents the style value for `bottom` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#bottom).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#bottom
#[syntax(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct BottomStyleValue;

/// Represents the style value for `left` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#left).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#left
#[syntax(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct LeftStyleValue;

/// Represents the style value for `inset-block-start` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset-block-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset-block-start
#[syntax(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct InsetBlockStartStyleValue;

/// Represents the style value for `inset-inline-start` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset-inline-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset-inline-start
#[syntax(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct InsetInlineStartStyleValue;

/// Represents the style value for `inset-block-end` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset-block-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset-block-end
#[syntax(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct InsetBlockEndStyleValue;

/// Represents the style value for `inset-inline-end` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset-inline-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | <length-percentage>
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset-inline-end
#[syntax(" auto | <length-percentage> ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("refer to size of containing block; see prose")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct InsetInlineEndStyleValue;

/// Represents the style value for `inset-block` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset-block).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'top'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset-block
#[syntax(" <'top'>{1,2} ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct InsetBlockStyleValue;

/// Represents the style value for `inset-inline` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset-inline).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'top'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset-inline
#[syntax(" <'top'>{1,2} ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct InsetInlineStyleValue;

/// Represents the style value for `inset` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#inset).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'top'>{1,4}
/// ```
///
// https://drafts.csswg.org/css-position-4/#inset
#[syntax(" <'top'>{1,4} ")]
#[initial("auto")]
#[applies_to("positioned elements")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct InsetStyleValue;

/// Represents the style value for `overlay` as defined in [css-position-4](https://drafts.csswg.org/css-position-4/#overlay).
///
/// The overlay CSS property, used as an allow-discrete CSS transition, prevents a top layer element, such as a popover or a <dialog>, from being removed from the top layer before it has finished animating. You can't set the value of the overlay property; only use it as transition property.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | auto
/// ```
///
// https://drafts.csswg.org/css-position-4/#overlay
#[syntax(" none | auto ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see prose")]
#[popularity(90.749)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(chrome:117,chrome_android:117,edge:117)]
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum OverlayStyleValue {}
