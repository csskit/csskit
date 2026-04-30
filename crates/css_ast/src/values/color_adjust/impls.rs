#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorAdjustStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ForcedColorAdjustStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PrintColorAdjustStyleValue>(), 16);
	}

	#[test]
	fn test_forced_color_adjust() {
		assert_parse!(CssAtomSet::ATOMS, ForcedColorAdjustStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ForcedColorAdjustStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ForcedColorAdjustStyleValue, "preserve-parent-color");
	}

	#[test]
	fn test_forced_color_adjust_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ForcedColorAdjustStyleValue, "exact");
		assert_parse_error!(CssAtomSet::ATOMS, ForcedColorAdjustStyleValue, "auto none");
	}

	#[test]
	fn test_print_color_adjust() {
		assert_parse!(CssAtomSet::ATOMS, PrintColorAdjustStyleValue, "economy");
		assert_parse!(CssAtomSet::ATOMS, PrintColorAdjustStyleValue, "exact");
	}

	#[test]
	fn test_print_color_adjust_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PrintColorAdjustStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, PrintColorAdjustStyleValue, "economy exact");
	}
}
