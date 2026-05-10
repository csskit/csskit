#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ZIndexStyleValue>(), 16);
	}

	#[test]
	fn test_z_index() {
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "-1");
		assert_parse!(CssAtomSet::ATOMS, ZIndexStyleValue, "999");
	}

	#[test]
	fn test_z_index_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ZIndexStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, ZIndexStyleValue, "1.5");
		assert_parse_error!(CssAtomSet::ATOMS, ZIndexStyleValue, "");
	}
}
