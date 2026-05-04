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
		assert_eq!(std::mem::size_of::<HangingPunctuationStyleValue>(), 52);
		assert_eq!(std::mem::size_of::<TextIndentStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<TextJustifyStyleValue>(), 36);
		assert_eq!(std::mem::size_of::<WhiteSpaceTrimStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<WordSpaceTransformStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TextTransformStyleValue>(), 52);
		assert_eq!(std::mem::size_of::<WhiteSpaceStyleValue>(), 80);
	}

	#[test]
	fn test_text_transform() {
		assert_parse!(CssAtomSet::ATOMS, TextTransformStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TextTransformStyleValue, "capitalize");
		assert_parse!(CssAtomSet::ATOMS, TextTransformStyleValue, "uppercase");
		assert_parse!(CssAtomSet::ATOMS, TextTransformStyleValue, "lowercase");
		assert_parse!(CssAtomSet::ATOMS, TextTransformStyleValue, "full-width");
		assert_parse!(CssAtomSet::ATOMS, TextTransformStyleValue, "full-size-kana");
		assert_parse!(CssAtomSet::ATOMS, TextTransformStyleValue, "math-auto");
		assert_parse!(CssAtomSet::ATOMS, TextTransformStyleValue, "capitalize full-width");
		assert_parse!(CssAtomSet::ATOMS, TextTransformStyleValue, "full-width full-size-kana");
		assert_parse_error!(CssAtomSet::ATOMS, TextTransformStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, TextTransformStyleValue, "1px");
	}

	#[test]
	fn test_hanging_punctuation() {
		assert_parse!(CssAtomSet::ATOMS, HangingPunctuationStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, HangingPunctuationStyleValue, "first");
		assert_parse!(CssAtomSet::ATOMS, HangingPunctuationStyleValue, "last");
		assert_parse!(CssAtomSet::ATOMS, HangingPunctuationStyleValue, "force-end");
		assert_parse!(CssAtomSet::ATOMS, HangingPunctuationStyleValue, "allow-end");
		assert_parse!(CssAtomSet::ATOMS, HangingPunctuationStyleValue, "first last");
		assert_parse!(CssAtomSet::ATOMS, HangingPunctuationStyleValue, "first force-end last");
		assert_parse_error!(CssAtomSet::ATOMS, HangingPunctuationStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, HangingPunctuationStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, HangingPunctuationStyleValue, "force-end allow-end");
	}

	#[test]
	fn test_text_justify() {
		assert_parse!(CssAtomSet::ATOMS, TextJustifyStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, TextJustifyStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TextJustifyStyleValue, "inter-word");
		assert_parse!(CssAtomSet::ATOMS, TextJustifyStyleValue, "inter-character");
		assert_parse!(CssAtomSet::ATOMS, TextJustifyStyleValue, "ruby");
		assert_parse!(CssAtomSet::ATOMS, TextJustifyStyleValue, "no-compress");
		assert_parse!(CssAtomSet::ATOMS, TextJustifyStyleValue, "auto no-compress");
		assert_parse!(CssAtomSet::ATOMS, TextJustifyStyleValue, "inter-word no-compress");
		assert_parse_error!(CssAtomSet::ATOMS, TextJustifyStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, TextJustifyStyleValue, "left");
	}

	#[test]
	fn test_text_indent() {
		assert_parse!(CssAtomSet::ATOMS, TextIndentStyleValue, "1em");
		assert_parse!(CssAtomSet::ATOMS, TextIndentStyleValue, "10%");
		assert_parse!(CssAtomSet::ATOMS, TextIndentStyleValue, "1em hanging");
		assert_parse!(CssAtomSet::ATOMS, TextIndentStyleValue, "1em each-line");
		assert_parse!(CssAtomSet::ATOMS, TextIndentStyleValue, "1em hanging each-line");
		assert_parse_error!(CssAtomSet::ATOMS, TextIndentStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, TextIndentStyleValue, "hanging");
	}

	#[test]
	fn test_white_space() {
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "pre");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "pre-wrap");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "pre-line");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "preserve");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "collapse");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "wrap");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "nowrap");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "preserve nowrap");
		assert_parse_error!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "1px");
	}

	#[test]
	fn test_white_space_trim() {
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "discard-before");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "discard-after");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "discard-inner");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "discard-before discard-after");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "discard-before discard-after discard-inner");
		assert_parse_error!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "auto");
	}

	#[test]
	fn test_word_space_transform() {
		assert_parse!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "space");
		assert_parse!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "ideographic-space");
		assert_parse!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "space auto-phrase");
		assert_parse!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "ideographic-space auto-phrase");
		assert_parse_error!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "auto");
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
