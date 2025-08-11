#![allow(warnings)]
//! CSS Overflow Module Level 5
//! https://drafts.csswg.org/css-overflow-5/

mod impls;
use impls::*;

/// Represents the style value for `overflow-x` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-x).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// visible | hidden | clip | scroll | auto
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-x
#[value(" visible | hidden | clip | scroll | auto ")]
#[initial("visible")]
#[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], grid containers [CSS3-GRID-LAYOUT]")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum OverflowXStyleValue {}

/// Represents the style value for `overflow-y` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-y).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// visible | hidden | clip | scroll | auto
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-y
#[value(" visible | hidden | clip | scroll | auto ")]
#[initial("visible")]
#[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], grid containers [CSS3-GRID-LAYOUT]")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum OverflowYStyleValue {}

/// Represents the style value for `overflow-block` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-block).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// visible | hidden | clip | scroll | auto
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-block
#[value(" visible | hidden | clip | scroll | auto ")]
#[initial("visible")]
#[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], grid containers [CSS3-GRID-LAYOUT]")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum OverflowBlockStyleValue {}

/// Represents the style value for `overflow-inline` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-inline).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// visible | hidden | clip | scroll | auto
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-inline
#[value(" visible | hidden | clip | scroll | auto ")]
#[initial("visible")]
#[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], grid containers [CSS3-GRID-LAYOUT]")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum OverflowInlineStyleValue {}

/// Represents the style value for `overflow` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow).
///
/// The overflow-block and overflow-inline CSS media queries set styles based on the way a device displays content that's larger than the viewport or page area. For example, a laptop lets users scroll to reveal content, while a printer displays overflowing content on additional pages.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <'overflow-block'>{1,2}
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow
#[value(" <'overflow-block'>{1,2} ")]
#[initial("visible")]
#[applies_to("block containers [CSS2], flex containers [CSS3-FLEXBOX], and grid containers [CSS3-GRID-LAYOUT]")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(newly)]
#[versions(chrome:113,chrome_android:113,edge:113,firefox:66,firefox_android:66,safari:17,safari_ios:17)]
pub struct OverflowStyleValue;

/// Represents the style value for `overflow-clip-margin` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin).
///
/// The overflow-clip-margin CSS property sets how far overflow content may appear outside the bounds of an element before it's clipped by effects such as overflow: clip.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box> || <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(limited)]
#[versions(Unknown)]
pub struct OverflowClipMarginStyleValue;

/// Represents the style value for `scroll-behavior` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#scroll-behavior).
///
/// The scroll-behavior CSS property controls whether scrolling is smooth or snaps, for scroll actions not performed by the user such as those triggered by navigation.
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | smooth
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#scroll-behavior
#[value(" auto | smooth ")]
#[initial("auto")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
#[popularity(86.707)]
#[caniuse("https://caniuse.com/css-scroll-behavior")]
#[baseline(widely)]
#[versions(chrome:61,chrome_android:61,edge:79,firefox:36,firefox_android:36,safari:15.4,safari_ios:15.4)]
pub enum ScrollBehaviorStyleValue {}

// /// Represents the style value for `scrollbar-gutter` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#scrollbar-gutter).
// ///
// /// The scrollbar-gutter CSS property reserves space for the scrollbar, preventing unwanted layout changes as the scrollbar appears and disappears.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// auto | stable && both-edges?
// /// ```
// ///
// // https://drafts.csswg.org/css-overflow-5/#scrollbar-gutter
// #[value(" auto | stable && both-edges? ")]
// #[initial("auto")]
// #[applies_to("scroll containers")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// #[popularity(86.707)]
// #[caniuse(Unknown)]
// #[baseline(newly)]
// #[versions(chrome:94,chrome_android:94,edge:94,firefox:97,firefox_android:97,safari:18.2,safari_ios:18.2)]
// pub enum ScrollbarGutterStyleValue {}

// /// Represents the style value for `text-overflow` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#text-overflow).
// ///
// /// The text-overflow CSS property sets how hidden overflow content appears to users. The property can clip content, truncate content with an ellipsis (…), or truncate with a custom string.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// [ clip | ellipsis | <string> | fade | <fade()> ]{1,2}
// /// ```
// ///
// // https://drafts.csswg.org/css-overflow-5/#text-overflow
// #[value(" [ clip | ellipsis | <string> | fade | <fade()> ]{1,2} ")]
// #[initial("clip")]
// #[applies_to("block containers")]
// #[inherited("no")]
// #[percentages("refer to the width of the line box")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value type")]
// #[popularity(86.707)]
// #[caniuse("https://caniuse.com/text-overflow")]
// #[baseline(widely)]
// #[versions(chrome:1,chrome_android:18,edge:12,firefox:7,firefox_android:7,safari:1.3,safari_ios:1)]
// pub struct TextOverflowStyleValue;

/// Represents the style value for `overflow-clip-margin-top` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-top).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box> || <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-top
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct OverflowClipMarginTopStyleValue;

/// Represents the style value for `overflow-clip-margin-right` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-right).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box> || <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-right
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct OverflowClipMarginRightStyleValue;

/// Represents the style value for `overflow-clip-margin-bottom` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-bottom).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box> || <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-bottom
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct OverflowClipMarginBottomStyleValue;

/// Represents the style value for `overflow-clip-margin-left` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-left).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box> || <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-left
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct OverflowClipMarginLeftStyleValue;

/// Represents the style value for `overflow-clip-margin-block-start` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-block-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box> || <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-block-start
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct OverflowClipMarginBlockStartStyleValue;

/// Represents the style value for `overflow-clip-margin-inline-start` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-inline-start).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box> || <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-inline-start
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct OverflowClipMarginInlineStartStyleValue;

/// Represents the style value for `overflow-clip-margin-block-end` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-block-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box> || <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-block-end
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct OverflowClipMarginBlockEndStyleValue;

/// Represents the style value for `overflow-clip-margin-inline-end` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-inline-end).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box> || <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-inline-end
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("per computed value if the <visual-box> values match; otherwise discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct OverflowClipMarginInlineEndStyleValue;

/// Represents the style value for `overflow-clip-margin-inline` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-inline).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box> || <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-inline
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct OverflowClipMarginInlineStyleValue;

/// Represents the style value for `overflow-clip-margin-block` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-block).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// <visual-box> || <length [0,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#overflow-clip-margin-block
#[value(" <visual-box> || <length [0,∞]> ")]
#[initial("0px")]
#[applies_to("boxes to which overflow applies")]
#[inherited("no")]
#[percentages("see individual properties")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub struct OverflowClipMarginBlockStyleValue;

/// Represents the style value for `block-ellipsis` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#block-ellipsis).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | auto | <string>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#block-ellipsis
#[value(" none | auto | <string> ")]
#[initial("none")]
#[applies_to("block containers")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum BlockEllipsisStyleValue {}

// /// Represents the style value for `line-clamp` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#line-clamp).
// ///
// /// The line-clamp CSS property limits the text in a block container to a certain number of lines. The prefixed -webkit-line-clamp is widely supported but only works with -webkit-box-orient: vertical in combination with display: -webkit-box or display: -webkit-inline-box.
// ///
// /// The grammar is defined as:
// ///
// /// ```text,ignore
// /// none | [<integer [1,∞]> || <'block-ellipsis'>] -webkit-legacy?
// /// ```
// ///
// // https://drafts.csswg.org/css-overflow-5/#line-clamp
// #[value(" none | [<integer [1,∞]> || <'block-ellipsis'>] -webkit-legacy? ")]
// #[initial("none")]
// #[applies_to("see individual properties")]
// #[inherited("see individual properties")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// #[popularity(86.707)]
// #[caniuse("https://caniuse.com/css-line-clamp")]
// #[baseline(limited)]
// #[versions(safari:18.2,safari_ios:18.2)]
// pub enum LineClampStyleValue {}

/// Represents the style value for `-webkit-line-clamp` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#-webkit-line-clamp).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <integer [1,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#-webkit-line-clamp
#[value(" none | <integer [1,∞]> ")]
#[initial("none")]
#[applies_to("see individual properties")]
#[inherited("see individual properties")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum WebkitLineClampStyleValue {}

/// Represents the style value for `max-lines` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#max-lines).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | <integer [1,∞]>
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#max-lines
#[value(" none | <integer [1,∞]> ")]
#[initial("none")]
#[applies_to(
	"block containers which are also either fragmentation containers that capture region breaks or line-clamp containers"
)]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum MaxLinesStyleValue {}

/// Represents the style value for `continue` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#continue).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// auto | discard | collapse
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#continue
#[value(" auto | discard | collapse ")]
#[initial("auto")]
#[applies_to("block containers and multicol containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum ContinueStyleValue {}

/// Represents the style value for `scroll-target-group` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#scroll-target-group).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | auto
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#scroll-target-group
#[value(" none | auto ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum ScrollTargetGroupStyleValue {}

/// Represents the style value for `scroll-marker-group` as defined in [css-overflow-5](https://drafts.csswg.org/css-overflow-5/#scroll-marker-group).
///
///
/// The grammar is defined as:
///
/// ```text,ignore
/// none | before | after
/// ```
///
// https://drafts.csswg.org/css-overflow-5/#scroll-marker-group
#[value(" none | before | after ")]
#[initial("none")]
#[applies_to("scroll containers")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
#[popularity(86.707)]
#[caniuse(Unknown)]
#[baseline(Unknown)]
#[versions(Unknown)]
pub enum ScrollMarkerGroupStyleValue {}
