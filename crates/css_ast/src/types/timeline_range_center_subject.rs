use super::prelude::*;

/// <https://drafts.csswg.org/pointer-animations-1/#typedef-timeline-range-center-subject>
///
/// ```text,ignore
/// <timeline-range-center-subject> = source | target
/// ```
#[syntax(" source | target ")]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum TimelineRangeCenterSubject {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TimelineRangeCenterSubject, "source");
		assert_parse!(CssAtomSet::ATOMS, TimelineRangeCenterSubject, "target");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, TimelineRangeCenterSubject, "");
		assert_peek_false!(CssAtomSet::ATOMS, TimelineRangeCenterSubject, "normal");
		assert_peek_false!(CssAtomSet::ATOMS, TimelineRangeCenterSubject, "cover");
	}
}
