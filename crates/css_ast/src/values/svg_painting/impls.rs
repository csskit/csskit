#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

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
		assert_peek_false!(CssAtomSet::ATOMS, ColorInterpolationStyleValue, "red");
		assert_peek_false!(CssAtomSet::ATOMS, ShapeRenderingStyleValue, "crisp-edges");
		assert_parse_error!(CssAtomSet::ATOMS, PaintOrderStyleValue, "fill fill");
	}
}
