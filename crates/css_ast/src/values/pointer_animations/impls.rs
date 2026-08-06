#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "block");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "inline");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "x");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "y");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "block, inline");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineNameStyleValue, "--my-timeline");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineNameStyleValue, "none, --my-timeline");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineStyleValue, "--my-timeline");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineStyleValue, "--my-timeline block");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeCenterStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeCenterStyleValue, "50%");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeCenterStyleValue, "source");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeCenterStyleValue, "target");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeCenterStyleValue, "source 10px");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeCenterStyleValue, "target 25%");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeCenterStyleValue, "normal, target 25%");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "block inline");
		assert_peek_false!(CssAtomSet::ATOMS, PointerTimelineNameStyleValue, "auto");
		assert_peek_false!(CssAtomSet::ATOMS, PointerTimelineStyleValue, "auto");
		assert_peek_false!(CssAtomSet::ATOMS, AnimationRangeCenterStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationRangeCenterStyleValue, "source target");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationRangeCenterStyleValue, "10px source");
	}
}
