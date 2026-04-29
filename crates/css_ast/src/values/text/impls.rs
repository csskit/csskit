#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<HyphenateLimitCharsStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<HyphenateCharacterStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<HyphenateLimitLastStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<HyphenateLimitLinesStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<HyphenateLimitZoneStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<HyphensStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<LetterSpacingStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<LineBreakStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<LinePaddingStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverflowWrapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TabSizeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextAlignAllStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextAlignLastStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextAlignStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextAutospaceStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextFitStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<TextGroupAlignStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextSpacingTrimStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextWrapModeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextWrapStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextWrapStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<WhiteSpaceCollapseStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<WordBreakStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<WordSpacingStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<WordWrapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<WrapAfterStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<WrapBeforeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<WrapInsideStyleValue>(), 16);
	}

	#[test]
	fn test_hyphenate_limit_chars() {
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "5");
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "auto 3");
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "5 2 2");
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "auto auto auto");
		assert_parse_error!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "5 2 2 2");
	}
}
