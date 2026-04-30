#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AllStyleValue>(), 16);
	}

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
		assert_parse_error!(CssAtomSet::ATOMS, AllStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, AllStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, AllStyleValue, "unset inherit");
	}
}
