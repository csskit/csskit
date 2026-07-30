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
}
