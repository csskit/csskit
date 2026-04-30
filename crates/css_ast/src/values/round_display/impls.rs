#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BorderBoundaryStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BorderBoundaryStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, BorderBoundaryStyleValue, "parent");
		assert_parse!(CssAtomSet::ATOMS, BorderBoundaryStyleValue, "display");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BorderBoundaryStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, BorderBoundaryStyleValue, "none parent");
	}
}
