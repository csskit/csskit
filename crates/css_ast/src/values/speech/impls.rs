#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		// assert_eq!(std::mem::size_of::<VoiceVolumeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<VoiceBalanceStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<SpeakStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<SpeakAsStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PauseBeforeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PauseAfterStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PauseStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<RestBeforeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<RestAfterStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<RestStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<CueBeforeStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<CueAfterStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<CueStyleValue>(), 112);
		// assert_eq!(std::mem::size_of::<VoiceFamilyStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<VoiceRateStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<VoicePitchStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<VoiceRangeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<VoiceStressStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<VoiceDurationStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CueBeforeStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, CueBeforeStyleValue, "url(foo)");
		assert_parse!(CssAtomSet::ATOMS, CueBeforeStyleValue, "url(foo)20db");
		assert_parse!(CssAtomSet::ATOMS, CueAfterStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, CueAfterStyleValue, "url(foo)");
		assert_parse!(CssAtomSet::ATOMS, CueAfterStyleValue, "url(foo)20db");
	}
}
