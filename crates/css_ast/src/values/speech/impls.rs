pub(crate) use crate::{CssDiagnostic, traits::StyleValue};
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
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
		assert_eq!(std::mem::size_of::<CueBeforeStyleValue>(), 28);
		assert_eq!(std::mem::size_of::<CueAfterStyleValue>(), 28);
		assert_eq!(std::mem::size_of::<CueStyleValue>(), 56);
		// assert_eq!(std::mem::size_of::<VoiceFamilyStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<VoiceRateStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<VoicePitchStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<VoiceRangeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<VoiceStressStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<VoiceDurationStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CueBeforeStyleValue, "none");
		assert_parse!(CueBeforeStyleValue, "url(foo)");
		assert_parse!(CueBeforeStyleValue, "url(foo)20db");
		assert_parse!(CueAfterStyleValue, "none");
		assert_parse!(CueAfterStyleValue, "url(foo)");
		assert_parse!(CueAfterStyleValue, "url(foo)20db");
	}
}
