#![allow(warnings)]
//! CSS Fragmentation Module Level 4  Breaking the Web, one fragment at a time
//! https://drafts.csswg.org/css-break-4/

mod impls;
use impls::*;

/// Represents the style value for `break-before` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#break-before).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | avoid | always | all | avoid-page | page | left | right | recto | verso | avoid-column | column | avoid-region | region
/// ```
///
// https://drafts.csswg.org/css-break-4/#break-before
#[value(
	" auto | avoid | always | all | avoid-page | page | left | right | recto | verso | avoid-column | column | avoid-region | region "
)]
#[initial("auto")]
#[applies_to("block-level boxes, grid items, flex items, table row groups, table rows (but see prose)")]
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
pub enum BreakBeforeStyleValue {}

/// Represents the style value for `break-after` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#break-after).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | avoid | always | all | avoid-page | page | left | right | recto | verso | avoid-column | column | avoid-region | region
/// ```
///
// https://drafts.csswg.org/css-break-4/#break-after
#[value(
	" auto | avoid | always | all | avoid-page | page | left | right | recto | verso | avoid-column | column | avoid-region | region "
)]
#[initial("auto")]
#[applies_to("block-level boxes, grid items, flex items, table row groups, table rows (but see prose)")]
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
pub enum BreakAfterStyleValue {}

/// Represents the style value for `break-inside` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#break-inside).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | avoid | avoid-page | avoid-column | avoid-region
/// ```
///
// https://drafts.csswg.org/css-break-4/#break-inside
#[value(" auto | avoid | avoid-page | avoid-column | avoid-region ")]
#[initial("auto")]
#[applies_to(
	"all elements except inline-level boxes, internal ruby boxes, table column boxes, table column group boxes, absolutely-positioned boxes"
)]
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
pub enum BreakInsideStyleValue {}

/// Represents the style value for `orphans` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#orphans).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <integer [1,∞]>
/// ```
///
// https://drafts.csswg.org/css-break-4/#orphans
#[value(" <integer [1,∞]> ")]
#[initial("2")]
#[applies_to("block containers that establish an inline formatting context")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct OrphansStyleValue;

/// Represents the style value for `widows` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#widows).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <integer [1,∞]>
/// ```
///
// https://drafts.csswg.org/css-break-4/#widows
#[value(" <integer [1,∞]> ")]
#[initial("2")]
#[applies_to("block containers that establish an inline formatting context")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(Unknown)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct WidowsStyleValue;

/// Represents the style value for `box-decoration-break` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#box-decoration-break).
///
/// The box-decoration-break CSS property sets whether box decorations, such as borders or backgrounds, of an element divided across a page, column, or region wraps each fragment or splits across the break.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// slice | clone
/// ```
///
// https://drafts.csswg.org/css-break-4/#box-decoration-break
#[value(" slice | clone ")]
#[initial("slice")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(Unknown)]
#[caniuse("https://caniuse.com/css-boxdecorationbreak")]
#[baseline(limited)]
#[versions(chrome:130,chrome_android:130,edge:130,firefox:32,firefox_android:32)]
#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum BoxDecorationBreakStyleValue {}

/// Represents the style value for `margin-break` as defined in [css-break-4](https://drafts.csswg.org/css-break-4/#margin-break).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | keep | discard
/// ```
///
// https://drafts.csswg.org/css-break-4/#margin-break
#[value(" auto | keep | discard ")]
#[initial("auto")]
#[applies_to("all elements")]
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
pub enum MarginBreakStyleValue {}
