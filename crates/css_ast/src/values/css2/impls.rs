#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_z_index() {
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "-1");
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "999");
	}

	#[test]
	fn test_z_index_tree_counting() {
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "sibling-index()");
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "sibling-count()");
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "calc(sibling-index() + 1)");
	}

	#[test]
	fn test_z_index_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, ZIndexStyleValue, "none");
		assert_peek_false!(CssAtomSet::ATOMS, ZIndexStyleValue, "1.5");
		assert_peek_false!(CssAtomSet::ATOMS, ZIndexStyleValue, "");
	}
}
