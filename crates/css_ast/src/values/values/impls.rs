#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, InterpolateSizeStyleValue, "numeric-only");
		assert_parse!(CssAtomSet::ATOMS, InterpolateSizeStyleValue, "allow-keywords");
	}

	#[test]
	fn test_peek() {
		assert_peek_false!(CssAtomSet::ATOMS, InterpolateSizeStyleValue, "auto");
		assert_peek_false!(CssAtomSet::ATOMS, InterpolateSizeStyleValue, "none");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, InterpolateSizeStyleValue, "numeric-only allow-keywords");
	}
}
