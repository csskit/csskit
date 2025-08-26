pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
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
		assert_parse!(TransformStyleValue, "none");
		assert_parse!(TransformStyleValue, "scale(1)");
		assert_parse!(TransformBoxStyleValue, "fill-box");
		assert_parse!(ScaleStyleValue, "none");
		assert_parse!(ScaleStyleValue, "1%");
		assert_parse!(ScaleStyleValue, "1 2 3");
		assert_parse!(ScaleStyleValue, "1.7 50%");
		assert_parse!(TransformStyleStyleValue, "flat");
		assert_parse!(PerspectiveOriginStyleValue, "1px");
		assert_parse!(BackfaceVisibilityStyleValue, "visible");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(TransformStyleValue, "auto");
		assert_parse_error!(ScaleStyleValue, "none none");
	}
}
