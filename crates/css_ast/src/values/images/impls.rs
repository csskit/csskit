#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ImageRenderingStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ObjectPositionStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<ObjectViewBoxStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ImageRenderingStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ImageRenderingStyleValue, "smooth");
		assert_parse!(CssAtomSet::ATOMS, ImageRenderingStyleValue, "high-quality");
		assert_parse!(CssAtomSet::ATOMS, ImageRenderingStyleValue, "crisp-edges");
		assert_parse!(CssAtomSet::ATOMS, ImageRenderingStyleValue, "pixelated");

		assert_parse!(CssAtomSet::ATOMS, ObjectPositionStyleValue, "-20% -30px");
		assert_parse!(CssAtomSet::ATOMS, ObjectPositionStyleValue, "left center");
		assert_parse!(CssAtomSet::ATOMS, ObjectPositionStyleValue, "center center");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ImageRenderingStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, ImageRenderingStyleValue, "high-quality crisp-edges");

		assert_parse_error!(CssAtomSet::ATOMS, ObjectPositionStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, ObjectPositionStyleValue, "1px 2px 3px");
		assert_parse_error!(CssAtomSet::ATOMS, ObjectPositionStyleValue, "left right");
	}
}
