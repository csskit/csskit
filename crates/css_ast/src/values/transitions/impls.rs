#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TransitionPropertyStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TransitionDurationStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TransitionTimingFunctionStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TransitionDelayStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TransitionStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TransitionBehaviorStyleValue>(), 32);
	}

	#[test]
	fn test_transition_delay() {
		assert_parse!(CssAtomSet::ATOMS, TransitionDelayStyleValue, "-5ms");
		assert_parse!(CssAtomSet::ATOMS, TransitionDelayStyleValue, "0s");
		assert_parse!(CssAtomSet::ATOMS, TransitionDelayStyleValue, "10s");
		assert_parse!(CssAtomSet::ATOMS, TransitionDelayStyleValue, "20s, 10s");
	}

	#[test]
	fn test_transition_delay_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TransitionDelayStyleValue, "0");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionDelayStyleValue, "0px");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionDelayStyleValue, "infinite");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionDelayStyleValue, "1s 2s 3s");
	}

	#[test]
	fn test_transition_duration() {
		assert_parse!(CssAtomSet::ATOMS, TransitionDurationStyleValue, "0s");
		assert_parse!(CssAtomSet::ATOMS, TransitionDurationStyleValue, "3s");
		assert_parse!(CssAtomSet::ATOMS, TransitionDurationStyleValue, "500ms");
		assert_parse!(CssAtomSet::ATOMS, TransitionDurationStyleValue, "1s, 2s, 3s");
	}

	#[test]
	fn test_transition_duration_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TransitionDurationStyleValue, "0");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionDurationStyleValue, "0px");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionDurationStyleValue, "-3s");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionDurationStyleValue, "infinite");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionDurationStyleValue, "1s 2s");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionDurationStyleValue, "auto");
	}

	#[test]
	fn test_transition_property() {
		assert_parse!(CssAtomSet::ATOMS, TransitionPropertyStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TransitionPropertyStyleValue, "all");
		assert_parse!(CssAtomSet::ATOMS, TransitionPropertyStyleValue, "opacity");
		assert_parse!(CssAtomSet::ATOMS, TransitionPropertyStyleValue, "color");
		assert_parse!(CssAtomSet::ATOMS, TransitionPropertyStyleValue, "background-color");
		assert_parse!(CssAtomSet::ATOMS, TransitionPropertyStyleValue, "opacity, color");
	}

	#[test]
	fn test_transition_property_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TransitionPropertyStyleValue, "12");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionPropertyStyleValue, "opacity color");
	}

	#[test]
	fn test_transition_timing_function() {
		assert_parse!(CssAtomSet::ATOMS, TransitionTimingFunctionStyleValue, "ease");
		assert_parse!(CssAtomSet::ATOMS, TransitionTimingFunctionStyleValue, "linear");
		assert_parse!(CssAtomSet::ATOMS, TransitionTimingFunctionStyleValue, "ease-in");
		assert_parse!(CssAtomSet::ATOMS, TransitionTimingFunctionStyleValue, "ease-out");
		assert_parse!(CssAtomSet::ATOMS, TransitionTimingFunctionStyleValue, "ease-in-out");
		assert_parse!(CssAtomSet::ATOMS, TransitionTimingFunctionStyleValue, "steps(2, start)");
		assert_parse!(CssAtomSet::ATOMS, TransitionTimingFunctionStyleValue, "cubic-bezier(0.1, 0.7, 1, 0.1)");
		assert_parse!(CssAtomSet::ATOMS, TransitionTimingFunctionStyleValue, "ease, linear, ease-in");
	}

	#[test]
	fn test_transition_timing_function_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TransitionTimingFunctionStyleValue, "steps(2,()start)");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionTimingFunctionStyleValue, "steps(2())");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionTimingFunctionStyleValue, "steps(2,())");
	}

	#[test]
	fn test_transition_behavior() {
		assert_parse!(CssAtomSet::ATOMS, TransitionBehaviorStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, TransitionBehaviorStyleValue, "allow-discrete");
		assert_parse!(CssAtomSet::ATOMS, TransitionBehaviorStyleValue, "allow-discrete, normal");
	}

	#[test]
	fn test_transition_behavior_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TransitionBehaviorStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionBehaviorStyleValue, "discrete");
		assert_parse_error!(CssAtomSet::ATOMS, TransitionBehaviorStyleValue, "allow-discrete normal");
	}
}
