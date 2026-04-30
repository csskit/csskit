#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TextSizeAdjustStyleValue>(), 16);
	}

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
		assert_parse_error!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "reverse");
		assert_parse_error!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "0");
		assert_parse_error!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "10px");
		assert_parse_error!(CssAtomSet::ATOMS, TextSizeAdjustStyleValue, "-100%");
	}
}
