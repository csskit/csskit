#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ImageAnimationStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ImageAnimationStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, ImageAnimationStyleValue, "paused");
		assert_parse!(CssAtomSet::ATOMS, ImageAnimationStyleValue, "stopped");
		assert_parse!(CssAtomSet::ATOMS, ImageAnimationStyleValue, "running");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ImageAnimationStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, ImageAnimationStyleValue, "normal paused");
	}
}
