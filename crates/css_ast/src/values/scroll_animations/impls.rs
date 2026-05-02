#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AnimationRangeEndStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnimationRangeStartStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnimationRangeStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ScrollTimelineAxisStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ScrollTimelineNameStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ScrollTimelineStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TimelineScopeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<ViewTimelineAxisStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ViewTimelineInsetStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ViewTimelineNameStyleValue>(), 32);
	}

	#[test]
	fn test_animation_range_start_writes() {
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "cover");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "contain");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "entry");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "exit");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "120%");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "120px");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "cover 100%");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "cover -42%");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "contain 42%");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "exit 42%");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "entry 42%");
	}

	#[test]
	fn test_animation_range_start_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "peek 50%");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "50% contain");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "normal 10px");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationRangeStartStyleValue, "contain contain");
	}

	#[test]
	fn test_animation_range_end_writes() {
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeEndStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeEndStyleValue, "cover");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeEndStyleValue, "contain");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeEndStyleValue, "120%");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeEndStyleValue, "120px");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeEndStyleValue, "cover 100%");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeEndStyleValue, "exit 42%");
		assert_parse!(CssAtomSet::ATOMS, AnimationRangeEndStyleValue, "entry 42px");
	}

	#[test]
	fn test_animation_range_end_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnimationRangeEndStyleValue, "infinite");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationRangeEndStyleValue, "peek 50%");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationRangeEndStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationRangeEndStyleValue, "normal 10px");
	}

	#[test]
	fn test_scroll_timeline_axis_writes() {
		assert_parse!(CssAtomSet::ATOMS, ScrollTimelineAxisStyleValue, "block");
		assert_parse!(CssAtomSet::ATOMS, ScrollTimelineAxisStyleValue, "inline");
		assert_parse!(CssAtomSet::ATOMS, ScrollTimelineAxisStyleValue, "x");
		assert_parse!(CssAtomSet::ATOMS, ScrollTimelineAxisStyleValue, "y");
	}

	#[test]
	fn test_scroll_timeline_axis_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ScrollTimelineAxisStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, ScrollTimelineAxisStyleValue, "horizontal");
		assert_parse_error!(CssAtomSet::ATOMS, ScrollTimelineAxisStyleValue, "block inline");
	}

	#[test]
	fn test_scroll_timeline_name_writes() {
		assert_parse!(CssAtomSet::ATOMS, ScrollTimelineNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ScrollTimelineNameStyleValue, "--my-timeline");
		assert_parse!(CssAtomSet::ATOMS, ScrollTimelineNameStyleValue, "--foo, --bar");
	}

	#[test]
	fn test_scroll_timeline_name_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ScrollTimelineNameStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, ScrollTimelineNameStyleValue, "foo");
	}

	#[test]
	fn test_view_timeline_axis_writes() {
		assert_parse!(CssAtomSet::ATOMS, ViewTimelineAxisStyleValue, "block");
		assert_parse!(CssAtomSet::ATOMS, ViewTimelineAxisStyleValue, "inline");
		assert_parse!(CssAtomSet::ATOMS, ViewTimelineAxisStyleValue, "x");
		assert_parse!(CssAtomSet::ATOMS, ViewTimelineAxisStyleValue, "y");
	}

	#[test]
	fn test_view_timeline_name_writes() {
		assert_parse!(CssAtomSet::ATOMS, ViewTimelineNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ViewTimelineNameStyleValue, "--my-timeline");
	}

	#[test]
	fn test_timeline_scope_writes() {
		assert_parse!(CssAtomSet::ATOMS, TimelineScopeStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TimelineScopeStyleValue, "all");
		assert_parse!(CssAtomSet::ATOMS, TimelineScopeStyleValue, "--my-scope");
		assert_parse!(CssAtomSet::ATOMS, TimelineScopeStyleValue, "--a, --b");
	}

	#[test]
	fn test_timeline_scope_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TimelineScopeStyleValue, "foo");
		assert_parse_error!(CssAtomSet::ATOMS, TimelineScopeStyleValue, "auto");
	}
}
