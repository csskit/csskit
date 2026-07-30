#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BorderBoundaryStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, BorderBoundaryStyleValue, "parent");
		assert_parse!(CssAtomSet::ATOMS, BorderBoundaryStyleValue, "display");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, BorderBoundaryStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, BorderBoundaryStyleValue, "none parent");
	}
}
