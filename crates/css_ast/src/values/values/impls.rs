#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<InterpolateSizeStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, InterpolateSizeStyleValue, "numeric-only");
		assert_parse!(CssAtomSet::ATOMS, InterpolateSizeStyleValue, "allow-keywords");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, InterpolateSizeStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, InterpolateSizeStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, InterpolateSizeStyleValue, "numeric-only allow-keywords");
	}
}
