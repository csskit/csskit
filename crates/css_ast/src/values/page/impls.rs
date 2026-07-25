#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PageStyleValue>(), 24);
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
		assert_peek_false!(CssAtomSet::ATOMS, PageStyleValue, "123px");
		assert_peek_false!(CssAtomSet::ATOMS, PageStyleValue, "default");
	}
}
