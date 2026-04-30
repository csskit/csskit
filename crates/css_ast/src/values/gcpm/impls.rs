#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FootnoteDisplayStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FootnotePolicyStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<RunningStyleValue>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FootnoteDisplayStyleValue, "block");
		assert_parse!(CssAtomSet::ATOMS, FootnoteDisplayStyleValue, "inline");
		assert_parse!(CssAtomSet::ATOMS, FootnoteDisplayStyleValue, "compact");
		assert_parse!(CssAtomSet::ATOMS, FootnotePolicyStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, FootnotePolicyStyleValue, "line");
		assert_parse!(CssAtomSet::ATOMS, FootnotePolicyStyleValue, "block");
		assert_parse!(CssAtomSet::ATOMS, RunningStyleValue, "myelement");
		assert_parse!(CssAtomSet::ATOMS, RunningStyleValue, "header");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FootnoteDisplayStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, FootnoteDisplayStyleValue, "block inline");
		assert_parse_error!(CssAtomSet::ATOMS, FootnotePolicyStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, FootnotePolicyStyleValue, "auto line");
	}
}
