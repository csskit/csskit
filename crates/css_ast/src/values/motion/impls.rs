#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OffsetAnchorStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<OffsetDistanceStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OffsetPositionStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<OffsetRotateStyleValue>(), 36);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, OffsetAnchorStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, OffsetAnchorStyleValue, "left bottom");
		assert_parse!(CssAtomSet::ATOMS, OffsetAnchorStyleValue, "center center");
		assert_parse!(CssAtomSet::ATOMS, OffsetAnchorStyleValue, "10px 20%");

		assert_parse!(CssAtomSet::ATOMS, OffsetDistanceStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, OffsetDistanceStyleValue, "20%");

		assert_parse!(CssAtomSet::ATOMS, OffsetPositionStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, OffsetPositionStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, OffsetPositionStyleValue, "left bottom");
		assert_parse!(CssAtomSet::ATOMS, OffsetPositionStyleValue, "center center");
		assert_parse!(CssAtomSet::ATOMS, OffsetPositionStyleValue, "10px 20%");
	}

	#[test]
	fn test_offset_rotate() {
		// [ auto | reverse ] || <angle>
		assert_parse!(CssAtomSet::ATOMS, OffsetRotateStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, OffsetRotateStyleValue, "reverse");
		assert_parse!(CssAtomSet::ATOMS, OffsetRotateStyleValue, "45deg");
		assert_parse!(CssAtomSet::ATOMS, OffsetRotateStyleValue, "auto 45deg");
		assert_parse!(CssAtomSet::ATOMS, OffsetRotateStyleValue, "reverse 90deg");
		assert_parse_error!(CssAtomSet::ATOMS, OffsetRotateStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, OffsetRotateStyleValue, "none");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, OffsetAnchorStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, OffsetAnchorStyleValue, "30deg");

		assert_parse_error!(CssAtomSet::ATOMS, OffsetDistanceStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, OffsetDistanceStyleValue, "30deg");

		assert_parse_error!(CssAtomSet::ATOMS, OffsetPositionStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, OffsetPositionStyleValue, "30deg");
	}
}
