#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorInterpolationStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MarkerStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<MarkerEndStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<MarkerMidStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<MarkerStartStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<PaintOrderStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<ShapeRenderingStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextRenderingStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColorInterpolationStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ColorInterpolationStyleValue, "sRGB");
		assert_parse!(CssAtomSet::ATOMS, ColorInterpolationStyleValue, "linearRGB");
		assert_parse!(CssAtomSet::ATOMS, MarkerStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MarkerEndStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MarkerMidStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MarkerStartStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, PaintOrderStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, PaintOrderStyleValue, "fill stroke markers");
		assert_parse!(CssAtomSet::ATOMS, ShapeRenderingStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ShapeRenderingStyleValue, "crispEdges");
		assert_parse!(CssAtomSet::ATOMS, ShapeRenderingStyleValue, "geometricPrecision");
		assert_parse!(CssAtomSet::ATOMS, TextRenderingStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, TextRenderingStyleValue, "optimizeLegibility");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ColorInterpolationStyleValue, "red");
		assert_parse_error!(CssAtomSet::ATOMS, PaintOrderStyleValue, "fill fill");
		assert_parse_error!(CssAtomSet::ATOMS, ShapeRenderingStyleValue, "crisp-edges");
	}
}
