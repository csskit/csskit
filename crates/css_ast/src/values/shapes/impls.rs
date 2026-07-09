#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ShapeImageThresholdStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ShapeInsideStyleValue>(), 264);
		assert_eq!(std::mem::size_of::<ShapeMarginStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ShapeOutsideStyleValue>(), 264);
		assert_eq!(std::mem::size_of::<ShapePaddingStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ShapeImageThresholdStyleValue, "12.5");
		assert_parse!(CssAtomSet::ATOMS, ShapeImageThresholdStyleValue, "-7");
		assert_parse!(CssAtomSet::ATOMS, ShapeImageThresholdStyleValue, "0.5");

		assert_parse!(CssAtomSet::ATOMS, ShapeMarginStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, ShapeMarginStyleValue, "20em");
		assert_parse!(CssAtomSet::ATOMS, ShapeMarginStyleValue, "37.5%");
	}

	#[test]
	fn test_shape_inside() {
		assert_parse!(CssAtomSet::ATOMS, ShapeInsideStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ShapeInsideStyleValue, "outside-shape");
		assert_parse!(CssAtomSet::ATOMS, ShapeInsideStyleValue, "display");
		assert_parse!(CssAtomSet::ATOMS, ShapeInsideStyleValue, "url(\"shape.svg\")");
		assert_peek_false!(CssAtomSet::ATOMS, ShapeInsideStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, ShapeInsideStyleValue, "none");
	}

	#[test]
	fn test_shape_outside() {
		assert_parse!(CssAtomSet::ATOMS, ShapeOutsideStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ShapeOutsideStyleValue, "margin-box");
		assert_parse!(CssAtomSet::ATOMS, ShapeOutsideStyleValue, "content-box");
		assert_parse!(CssAtomSet::ATOMS, ShapeOutsideStyleValue, "border-box");
		assert_parse!(CssAtomSet::ATOMS, ShapeOutsideStyleValue, "url(\"shape.svg\")");
		assert_peek_false!(CssAtomSet::ATOMS, ShapeOutsideStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, ShapeOutsideStyleValue, "auto");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, ShapeImageThresholdStyleValue, "auto");
		assert_peek_false!(CssAtomSet::ATOMS, ShapeImageThresholdStyleValue, "10px");

		assert_peek_false!(CssAtomSet::ATOMS, ShapeMarginStyleValue, "none");
		assert_peek_false!(CssAtomSet::ATOMS, ShapeMarginStyleValue, "10");
	}
}
