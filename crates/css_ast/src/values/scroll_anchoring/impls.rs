#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, OverflowAnchorStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, OverflowAnchorStyleValue, "none");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, OverflowAnchorStyleValue, "all");
		assert_parse_error!(CssAtomSet::ATOMS, OverflowAnchorStyleValue, "auto none");
	}
}
