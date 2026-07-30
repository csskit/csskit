#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

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
		assert_peek_false!(CssAtomSet::ATOMS, TouchActionStyleValue, "1px");
		assert_peek_false!(CssAtomSet::ATOMS, TouchActionStyleValue, "any");
	}
}
