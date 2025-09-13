#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		// assert_eq!(std::mem::size_of::<TransitionPropertyStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<TransitionDurationStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<TransitionTimingFunctionStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<TransitionDelayStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<TransitionStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<TransitionBehaviorStyleValue>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TransitionBehaviorStyleValue, "allow-discrete");
	}
}
