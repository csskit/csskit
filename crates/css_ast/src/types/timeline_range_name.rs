use super::prelude::*;

/// <https://drafts.csswg.org/scroll-animations-1/#typedef-timeline-range-name>
///
/// ```text,ignore
/// <timeline-range-name> = cover | contain | entry | exit | entry-crossing | exit-crossing | scroll
/// ```
#[syntax(" cover | contain | entry | exit | entry-crossing | exit-crossing | scroll ")]
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum TimelineRangeName {}
