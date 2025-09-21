#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TransformStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<TransformOriginStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TransformBoxStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<TranslateStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<RotateStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScaleStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<TransformStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PerspectiveStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PerspectiveOriginStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<BackfaceVisibilityStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TransformStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TransformStyleValue, "scale(1)");
		assert_parse!(CssAtomSet::ATOMS, TransformBoxStyleValue, "fill-box");
		assert_parse!(CssAtomSet::ATOMS, ScaleStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ScaleStyleValue, "1%");
		assert_parse!(CssAtomSet::ATOMS, ScaleStyleValue, "1 2 3");
		assert_parse!(CssAtomSet::ATOMS, ScaleStyleValue, "1.7 50%");
		assert_parse!(CssAtomSet::ATOMS, TransformStyleStyleValue, "flat");
		assert_parse!(CssAtomSet::ATOMS, PerspectiveOriginStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, BackfaceVisibilityStyleValue, "visible");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TransformStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, ScaleStyleValue, "none none");
	}
}
