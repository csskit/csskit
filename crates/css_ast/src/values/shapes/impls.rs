#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ShapeImageThresholdStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ShapeMarginStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ShapePaddingStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ShapeImageThresholdStyleValue, "0.5");

		assert_parse!(CssAtomSet::ATOMS, ShapeMarginStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, ShapeMarginStyleValue, "20em");
		assert_parse!(CssAtomSet::ATOMS, ShapeMarginStyleValue, "37.5%");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ShapeImageThresholdStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, ShapeImageThresholdStyleValue, "10px");

		assert_parse_error!(CssAtomSet::ATOMS, ShapeMarginStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, ShapeMarginStyleValue, "10");
	}
}
