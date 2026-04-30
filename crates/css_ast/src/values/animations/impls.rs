#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AnimationNameStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnimationDurationStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnimationTimingFunctionStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnimationIterationCountStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnimationDirectionStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnimationPlayStateStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnimationDelayStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnimationFillModeStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<AnimationStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnimationCompositionStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnimationTimelineStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<AnimationTriggerBehaviorStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<AnimationTriggerTimelineStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<AnimationTriggerRangeStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<AnimationTriggerRangeStartStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<AnimationTriggerRangeEndStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<AnimationTriggerExitRangeStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<AnimationTriggerExitRangeStartStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<AnimationTriggerExitRangeEndStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<AnimationTriggerStyleValue>(), 32);
	}

	#[test]
	fn test_animation_delay() {
		assert_parse!(CssAtomSet::ATOMS, AnimationDelayStyleValue, "-5ms");
		assert_parse!(CssAtomSet::ATOMS, AnimationDelayStyleValue, "0s");
		assert_parse!(CssAtomSet::ATOMS, AnimationDelayStyleValue, "10s");
		assert_parse!(CssAtomSet::ATOMS, AnimationDelayStyleValue, "20s, 10s");
	}

	#[test]
	fn test_animation_delay_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnimationDelayStyleValue, "infinite");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationDelayStyleValue, "1s 2s 3s");
	}

	#[test]
	fn test_animation_direction() {
		assert_parse!(CssAtomSet::ATOMS, AnimationDirectionStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, AnimationDirectionStyleValue, "reverse");
		assert_parse!(CssAtomSet::ATOMS, AnimationDirectionStyleValue, "alternate");
		assert_parse!(CssAtomSet::ATOMS, AnimationDirectionStyleValue, "alternate-reverse");
		assert_parse!(CssAtomSet::ATOMS, AnimationDirectionStyleValue, "normal, reverse, alternate, alternate-reverse");
	}

	#[test]
	fn test_animation_direction_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnimationDirectionStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationDirectionStyleValue, "normal reverse");
	}

	#[test]
	fn test_animation_duration() {
		assert_parse!(CssAtomSet::ATOMS, AnimationDurationStyleValue, "3s");
		assert_parse!(CssAtomSet::ATOMS, AnimationDurationStyleValue, "500ms");
		assert_parse!(CssAtomSet::ATOMS, AnimationDurationStyleValue, "1s, 2s, 3s");
	}

	#[test]
	fn test_animation_duration_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnimationDurationStyleValue, "-3s");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationDurationStyleValue, "infinite");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationDurationStyleValue, "1s 2s");
	}

	#[test]
	fn test_animation_fill_mode() {
		assert_parse!(CssAtomSet::ATOMS, AnimationFillModeStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, AnimationFillModeStyleValue, "forwards");
		assert_parse!(CssAtomSet::ATOMS, AnimationFillModeStyleValue, "backwards");
		assert_parse!(CssAtomSet::ATOMS, AnimationFillModeStyleValue, "both");
		assert_parse!(CssAtomSet::ATOMS, AnimationFillModeStyleValue, "none, forwards, backwards, both");
	}

	#[test]
	fn test_animation_fill_mode_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnimationFillModeStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationFillModeStyleValue, "forwards backwards");
	}

	#[test]
	fn test_animation_iteration_count() {
		assert_parse!(CssAtomSet::ATOMS, AnimationIterationCountStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, AnimationIterationCountStyleValue, "3");
		assert_parse!(CssAtomSet::ATOMS, AnimationIterationCountStyleValue, "4.5");
		assert_parse!(CssAtomSet::ATOMS, AnimationIterationCountStyleValue, "infinite");
		assert_parse!(CssAtomSet::ATOMS, AnimationIterationCountStyleValue, "0, infinite, 3");
	}

	#[test]
	fn test_animation_iteration_count_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnimationIterationCountStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationIterationCountStyleValue, "-2");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationIterationCountStyleValue, "3 4");
	}

	#[test]
	fn test_animation_name() {
		assert_parse!(CssAtomSet::ATOMS, AnimationNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, AnimationNameStyleValue, "foo");
		assert_parse!(CssAtomSet::ATOMS, AnimationNameStyleValue, "ease-in");
		assert_parse!(CssAtomSet::ATOMS, AnimationNameStyleValue, "infinite");
		assert_parse!(CssAtomSet::ATOMS, AnimationNameStyleValue, "paused");
		assert_parse!(CssAtomSet::ATOMS, AnimationNameStyleValue, "first, second, third");
	}

	#[test]
	fn test_animation_name_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnimationNameStyleValue, "12");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationNameStyleValue, "one two");
	}

	#[test]
	fn test_animation_play_state() {
		assert_parse!(CssAtomSet::ATOMS, AnimationPlayStateStyleValue, "running");
		assert_parse!(CssAtomSet::ATOMS, AnimationPlayStateStyleValue, "paused");
		assert_parse!(CssAtomSet::ATOMS, AnimationPlayStateStyleValue, "running, paused");
	}

	#[test]
	fn test_animation_play_state_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnimationPlayStateStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationPlayStateStyleValue, "paused running");
	}

	#[test]
	fn test_animation_composition() {
		assert_parse!(CssAtomSet::ATOMS, AnimationCompositionStyleValue, "replace");
		assert_parse!(CssAtomSet::ATOMS, AnimationCompositionStyleValue, "add");
		assert_parse!(CssAtomSet::ATOMS, AnimationCompositionStyleValue, "accumulate");
		assert_parse!(CssAtomSet::ATOMS, AnimationCompositionStyleValue, "replace, add, accumulate");
	}

	#[test]
	fn test_animation_composition_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnimationCompositionStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationCompositionStyleValue, "add replace");
	}

	#[test]
	fn test_animation_timing_function_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnimationTimingFunctionStyleValue, "steps(2,()start)");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationTimingFunctionStyleValue, "steps(2())");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationTimingFunctionStyleValue, "steps(2,())");
	}
}
