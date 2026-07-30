#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_text_align() {
		assert_parse!(CssAtomSet::ATOMS, TextAlignStyleValue, "start");
		assert_parse!(CssAtomSet::ATOMS, TextAlignStyleValue, "left");
		assert_parse!(CssAtomSet::ATOMS, TextAlignStyleValue, "match-parent");
		assert_parse!(CssAtomSet::ATOMS, TextAlignStyleValue, "-webkit-match-parent");
		assert_parse!(CssAtomSet::ATOMS, TextAlignAllStyleValue, "match-parent");
		assert_parse!(CssAtomSet::ATOMS, TextAlignAllStyleValue, "-webkit-match-parent");
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
		assert_peek_false!(CssAtomSet::ATOMS, TextTransformStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, TextTransformStyleValue, "1px");
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
		assert_peek_false!(CssAtomSet::ATOMS, HangingPunctuationStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, HangingPunctuationStyleValue, "auto");
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
		assert_peek_false!(CssAtomSet::ATOMS, TextJustifyStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, TextJustifyStyleValue, "left");
	}

	#[test]
	fn test_text_indent() {
		assert_parse!(CssAtomSet::ATOMS, TextIndentStyleValue, "1em");
		assert_parse!(CssAtomSet::ATOMS, TextIndentStyleValue, "10%");
		assert_parse!(CssAtomSet::ATOMS, TextIndentStyleValue, "1em hanging");
		assert_parse!(CssAtomSet::ATOMS, TextIndentStyleValue, "1em each-line");
		assert_parse!(CssAtomSet::ATOMS, TextIndentStyleValue, "1em hanging each-line");
		assert_peek_false!(CssAtomSet::ATOMS, TextIndentStyleValue, "");
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
		assert_peek_false!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, WhiteSpaceStyleValue, "1px");
	}

	#[test]
	fn test_white_space_trim() {
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "discard-before");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "discard-after");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "discard-inner");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "discard-before discard-after");
		assert_parse!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "discard-before discard-after discard-inner");
		assert_peek_false!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, WhiteSpaceTrimStyleValue, "auto");
	}

	#[test]
	fn test_word_space_transform() {
		assert_parse!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "space");
		assert_parse!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "ideographic-space");
		assert_parse!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "space auto-phrase");
		assert_parse!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "ideographic-space auto-phrase");
		assert_peek_false!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, WordSpaceTransformStyleValue, "auto");
	}

	#[test]
	fn test_hyphenate_limit_chars() {
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "5");
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "auto 3");
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "5 2 2");
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "auto auto auto");
		assert_peek_false!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "5 2 2 2");
	}
}
