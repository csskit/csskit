#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_all() {
		assert_parse!(CssAtomSet::ATOMS, AllStyleValue, "initial");
		assert_parse!(CssAtomSet::ATOMS, AllStyleValue, "inherit");
		assert_parse!(CssAtomSet::ATOMS, AllStyleValue, "unset");
		assert_parse!(CssAtomSet::ATOMS, AllStyleValue, "revert");
		assert_parse!(CssAtomSet::ATOMS, AllStyleValue, "revert-layer");
	}

	#[test]
	fn test_all_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, AllStyleValue, "auto");
		assert_peek_false!(CssAtomSet::ATOMS, AllStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, AllStyleValue, "unset inherit");
	}
}
