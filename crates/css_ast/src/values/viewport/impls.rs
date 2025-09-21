#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<ZoomStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ZoomStyleValue, "10");
		assert_parse!(CssAtomSet::ATOMS, ZoomStyleValue, "10.2");
		assert_parse!(CssAtomSet::ATOMS, ZoomStyleValue, "100%");
		assert_parse!(CssAtomSet::ATOMS, ZoomStyleValue, "100.5%");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ZoomStyleValue, "-100%");
		assert_parse_error!(CssAtomSet::ATOMS, ZoomStyleValue, "-10");
		assert_parse_error!(CssAtomSet::ATOMS, ZoomStyleValue, "smaller");
		assert_parse_error!(CssAtomSet::ATOMS, ZoomStyleValue, "10 10%");
		assert_parse_error!(CssAtomSet::ATOMS, ZoomStyleValue, "10% 10");
	}
}
