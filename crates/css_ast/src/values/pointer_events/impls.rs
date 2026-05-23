#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TouchActionStyleValue>(), 52);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TouchActionStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, TouchActionStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TouchActionStyleValue, "pan-x pan-y");
		assert_parse!(CssAtomSet::ATOMS, TouchActionStyleValue, "pan-y pinch-zoom");
		assert_parse!(CssAtomSet::ATOMS, TouchActionStyleValue, "pan-x pan-y pinch-zoom");
		assert_parse!(CssAtomSet::ATOMS, TouchActionStyleValue, "pinch-zoom");
		assert_parse!(CssAtomSet::ATOMS, TouchActionStyleValue, "manipulation");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TouchActionStyleValue, "1px");
		assert_parse_error!(CssAtomSet::ATOMS, TouchActionStyleValue, "any");
	}
}
