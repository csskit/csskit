#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ScrollbarColorStyleValue>(), 280);
	}

	#[test]
	fn test_parse() {
		assert_parse!(CssAtomSet::ATOMS, ScrollbarColorStyleValue, "red red");
		assert_parse!(CssAtomSet::ATOMS, ScrollbarColorStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ScrollbarColorStyleValue, "red #eee");
	}

	#[test]
	fn test_parse_error() {
		assert_parse_error!(CssAtomSet::ATOMS, ScrollbarColorStyleValue, "auto red");
		assert_parse_error!(CssAtomSet::ATOMS, ScrollbarColorStyleValue, "red");
		assert_parse_error!(CssAtomSet::ATOMS, ScrollbarColorStyleValue, "red green blue");
	}
}
