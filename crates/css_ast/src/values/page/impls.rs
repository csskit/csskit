#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PageStyleValue>(), 16);
	}

	#[test]
	fn test_page_writes() {
		assert_parse!(CssAtomSet::ATOMS, PageStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, PageStyleValue, "table");
		assert_parse!(CssAtomSet::ATOMS, PageStyleValue, "xyzabc");
	}

	#[test]
	fn test_page_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PageStyleValue, "not valid");
		assert_parse_error!(CssAtomSet::ATOMS, PageStyleValue, "123px");
		assert_parse_error!(CssAtomSet::ATOMS, PageStyleValue, "default");
	}
}
