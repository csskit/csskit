use super::prelude::*;

/// <https://drafts.csswg.org/scroll-animations-1/#typedef-timeline-range-name>
///
/// ```text,ignore
/// <timeline-range-name> = <ident>
/// ```
#[derive(IntoCursor, Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct TimelineRangeName(T![Ident]);
