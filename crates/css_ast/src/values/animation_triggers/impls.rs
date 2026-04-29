#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AnimationTriggerStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<EventTriggerNameStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<EventTriggerSourceStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<EventTriggerStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TimelineTriggerActivationRangeEndStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TimelineTriggerActivationRangeStartStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TimelineTriggerActivationRangeStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TimelineTriggerActiveRangeEndStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TimelineTriggerActiveRangeStartStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TimelineTriggerActiveRangeStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TimelineTriggerNameStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TimelineTriggerSourceStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TimelineTriggerStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TriggerScopeStyleValue>(), 40);
	}
}
