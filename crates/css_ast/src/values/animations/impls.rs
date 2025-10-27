#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

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
}
