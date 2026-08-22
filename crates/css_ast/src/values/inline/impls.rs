#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineHeightStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, LineHeightStyleValue, "1.618");

		assert_parse!(CssAtomSet::ATOMS, TextBoxStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, TextBoxStyleValue, "trim-start auto");
		assert_parse!(CssAtomSet::ATOMS, TextBoxStyleValue, "auto trim-start");
	}

	#[test]
	fn test_line_height_tree_counting() {
		assert_parse!(CssAtomSet::ATOMS, LineHeightStyleValue, "sibling-index()");
		assert_parse!(CssAtomSet::ATOMS, LineHeightStyleValue, "sibling-count()");
	}

	#[test]
	fn test_vertical_align() {
		use css_parse::{assert_parse_error, assert_peek_false};
		// [ first | last ] || <'alignment-baseline'> || <'baseline-shift'>
		assert_parse!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "first");
		assert_parse!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "last");
		assert_parse!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "baseline");
		assert_parse!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "sub");
		assert_parse!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "first baseline");
		assert_peek_false!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "first last");
	}

	#[test]
	fn test_initial_letter_align() {
		use css_parse::{assert_parse_error, assert_peek_false};
		// [ border-box? [ alphabetic | ideographic | hanging | leading ]? ]!
		assert_parse!(CssAtomSet::ATOMS, InitialLetterAlignStyleValue, "border-box");
		assert_parse!(CssAtomSet::ATOMS, InitialLetterAlignStyleValue, "alphabetic");
		assert_parse!(CssAtomSet::ATOMS, InitialLetterAlignStyleValue, "ideographic");
		assert_parse!(CssAtomSet::ATOMS, InitialLetterAlignStyleValue, "hanging");
		assert_parse!(CssAtomSet::ATOMS, InitialLetterAlignStyleValue, "leading");
		assert_parse!(CssAtomSet::ATOMS, InitialLetterAlignStyleValue, "border-box hanging");
		assert_peek_false!(CssAtomSet::ATOMS, InitialLetterAlignStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, InitialLetterAlignStyleValue, "hanging border-box");
		assert_parse_error!(CssAtomSet::ATOMS, InitialLetterAlignStyleValue, "border-box border-box");
	}
}
