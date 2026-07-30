#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_text_size_adjust_writes() {
		assert_parse!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "200%");
		assert_parse!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "100%");
		assert_parse!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "0%");
	}

	#[test]
	fn test_text_size_adjust_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "reverse");
		assert_peek_false!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "0");
		assert_peek_false!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "10px");
		assert_parse_error!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "-100%");
	}
}
