#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ImageAnimationStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, ImageAnimationStyleValue, "paused");
		assert_parse!(CssAtomSet::ATOMS, ImageAnimationStyleValue, "stopped");
		assert_parse!(CssAtomSet::ATOMS, ImageAnimationStyleValue, "running");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, ImageAnimationStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, ImageAnimationStyleValue, "normal paused");
	}
}
