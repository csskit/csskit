#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

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
		assert_peek_false!(CssAtomSet::ATOMS, OffsetRotateStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, OffsetRotateStyleValue, "none");
	}

	#[test]
	fn test_offset_path() {
		assert_parse!(CssAtomSet::ATOMS, OffsetPathStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, OffsetPathStyleValue, "content-box");
		assert_parse!(CssAtomSet::ATOMS, OffsetPathStyleValue, "border-box");
		assert_parse!(CssAtomSet::ATOMS, OffsetPathStyleValue, "fill-box");
		assert_parse!(CssAtomSet::ATOMS, OffsetPathStyleValue, "stroke-box");
		assert_parse!(CssAtomSet::ATOMS, OffsetPathStyleValue, "view-box");
		assert_parse!(CssAtomSet::ATOMS, OffsetPathStyleValue, "url(\"path.svg\")");
		assert_parse!(CssAtomSet::ATOMS, OffsetPathStyleValue, "url(\"path.svg\") fill-box");
		assert_parse!(CssAtomSet::ATOMS, OffsetPathStyleValue, "ray(45deg)");
		assert_parse!(CssAtomSet::ATOMS, OffsetPathStyleValue, "ray(45deg closest-side contain)");
		assert_parse!(CssAtomSet::ATOMS, OffsetPathStyleValue, "ray(45deg at center)border-box");
	}

	#[test]
	fn test_offset() {
		assert_parse!(CssAtomSet::ATOMS, OffsetStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, OffsetStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, OffsetStyleValue, "auto none");
		assert_parse!(CssAtomSet::ATOMS, OffsetStyleValue, "auto none 100px");
		assert_parse!(CssAtomSet::ATOMS, OffsetStyleValue, "auto none 100px 45deg");
		assert_parse!(CssAtomSet::ATOMS, OffsetStyleValue, "auto none 45deg 100px");
		assert_parse!(CssAtomSet::ATOMS, OffsetStyleValue, "none/auto");
		assert_parse!(CssAtomSet::ATOMS, OffsetStyleValue, "10px 20px none 50%/left top");
		assert_parse!(CssAtomSet::ATOMS, OffsetStyleValue, "ray(45deg)");
		assert_peek_false!(CssAtomSet::ATOMS, OffsetStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, OffsetStyleValue, "/auto");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, OffsetAnchorStyleValue, "none");
		assert_peek_false!(CssAtomSet::ATOMS, OffsetAnchorStyleValue, "30deg");

		assert_peek_false!(CssAtomSet::ATOMS, OffsetDistanceStyleValue, "none");
		assert_peek_false!(CssAtomSet::ATOMS, OffsetDistanceStyleValue, "30deg");

		assert_peek_false!(CssAtomSet::ATOMS, OffsetPathStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, OffsetPathStyleValue, "auto");

		assert_peek_false!(CssAtomSet::ATOMS, OffsetPositionStyleValue, "none");
		assert_peek_false!(CssAtomSet::ATOMS, OffsetPositionStyleValue, "30deg");
	}
}
