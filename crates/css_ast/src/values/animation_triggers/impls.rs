#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

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

	#[test]
	fn test_writes() {
		// AnimationTriggerStyleValue: none | [ <dashed-ident> <animation-action>+ ]+  #
		assert_parse!(CssAtomSet::ATOMS, AnimationTriggerStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, AnimationTriggerStyleValue, "--my-anim play");
		assert_parse!(CssAtomSet::ATOMS, AnimationTriggerStyleValue, "--my-anim play pause");
		assert_parse!(CssAtomSet::ATOMS, AnimationTriggerStyleValue, "--a play, --b pause");

		// EventTriggerNameStyleValue: none | <dashed-ident>#
		assert_parse!(CssAtomSet::ATOMS, EventTriggerNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, EventTriggerNameStyleValue, "--my-trigger");
		assert_parse!(CssAtomSet::ATOMS, EventTriggerNameStyleValue, "--a, --b");

		// TimelineTriggerNameStyleValue: none | <dashed-ident>#
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerNameStyleValue, "--my-timeline");
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerNameStyleValue, "--a, --b");

		// TriggerScopeStyleValue: none | all | <dashed-ident>#
		assert_parse!(CssAtomSet::ATOMS, TriggerScopeStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TriggerScopeStyleValue, "all");
		assert_parse!(CssAtomSet::ATOMS, TriggerScopeStyleValue, "--my-scope");
		assert_parse!(CssAtomSet::ATOMS, TriggerScopeStyleValue, "--a, --b");

		// TimelineTriggerActivationRangeStartStyleValue: [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerActivationRangeStartStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerActivationRangeStartStyleValue, "0%");
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerActivationRangeStartStyleValue, "50%");
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerActivationRangeStartStyleValue, "100px");

		// TimelineTriggerActivationRangeEndStyleValue: [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerActivationRangeEndStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerActivationRangeEndStyleValue, "100%");

		// TimelineTriggerActiveRangeStartStyleValue: [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerActiveRangeStartStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerActiveRangeStartStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerActiveRangeStartStyleValue, "50%");

		// TimelineTriggerActiveRangeEndStyleValue: [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerActiveRangeEndStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerActiveRangeEndStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, TimelineTriggerActiveRangeEndStyleValue, "0%");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnimationTriggerStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, AnimationTriggerStyleValue, "play");
		assert_parse_error!(CssAtomSet::ATOMS, EventTriggerNameStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, TimelineTriggerNameStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, TriggerScopeStyleValue, "auto");
	}
}
