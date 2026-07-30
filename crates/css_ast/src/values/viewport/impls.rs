#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ZoomStyleValue, "10");
		assert_parse!(CssAtomSet::ATOMS, ZoomStyleValue, "10.2");
		assert_parse!(CssAtomSet::ATOMS, ZoomStyleValue, "100%");
		assert_parse!(CssAtomSet::ATOMS, ZoomStyleValue, "100.5%");
	}

	#[test]
	fn test_peek() {
		assert_peek_false!(CssAtomSet::ATOMS, ZoomStyleValue, "smaller");
	}

	#[test]
	fn test_errors() {
		// ZoomStyleValue needs a lifetime to use NonNegative constraint
		// assert_parse_error!(CssAtomSet::ATOMS, ZoomStyleValue, "-100%");
		// assert_parse_error!(CssAtomSet::ATOMS, ZoomStyleValue, "-10");
		assert_parse_error!(CssAtomSet::ATOMS, ZoomStyleValue, "10 10%");
		assert_parse_error!(CssAtomSet::ATOMS, ZoomStyleValue, "10% 10");
	}
}
