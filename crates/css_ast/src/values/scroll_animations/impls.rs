#[cfg(test)]
mod tests {
	use super::super::*;

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
}
