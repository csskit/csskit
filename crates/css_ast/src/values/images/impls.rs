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
		assert_eq!(std::mem::size_of::<ImageOrientationStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ObjectFitStyleValue>(), 36);
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

	#[test]
	fn test_image_orientation() {
		assert_parse!(CssAtomSet::ATOMS, ImageOrientationStyleValue, "from-image");
		assert_parse!(CssAtomSet::ATOMS, ImageOrientationStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ImageOrientationStyleValue, "90deg");
		assert_parse!(CssAtomSet::ATOMS, ImageOrientationStyleValue, "flip");
		assert_parse!(CssAtomSet::ATOMS, ImageOrientationStyleValue, "90deg flip");
		assert_parse_error!(CssAtomSet::ATOMS, ImageOrientationStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, ImageOrientationStyleValue, "auto");
	}

	#[test]
	fn test_object_fit() {
		assert_parse!(CssAtomSet::ATOMS, ObjectFitStyleValue, "fill");
		assert_parse!(CssAtomSet::ATOMS, ObjectFitStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ObjectFitStyleValue, "contain");
		assert_parse!(CssAtomSet::ATOMS, ObjectFitStyleValue, "cover");
		assert_parse!(CssAtomSet::ATOMS, ObjectFitStyleValue, "scale-down");
		assert_parse!(CssAtomSet::ATOMS, ObjectFitStyleValue, "contain scale-down");
		assert_parse!(CssAtomSet::ATOMS, ObjectFitStyleValue, "cover scale-down");
		assert_parse_error!(CssAtomSet::ATOMS, ObjectFitStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, ObjectFitStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, ObjectFitStyleValue, "contain cover");
	}
}
