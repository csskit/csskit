#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_scrollbar_gutter() {
		assert_parse!(CssAtomSet::ATOMS, ScrollbarGutterStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ScrollbarGutterStyleValue, "stable");
		assert_parse!(CssAtomSet::ATOMS, ScrollbarGutterStyleValue, "stable both-edges");
		assert_peek_false!(CssAtomSet::ATOMS, ScrollbarGutterStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, ScrollbarGutterStyleValue, "1px");
	}

	#[test]
	fn test_scroll_marker_group() {
		assert_parse!(CssAtomSet::ATOMS, ScrollMarkerGroupStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ScrollMarkerGroupStyleValue, "before");
		assert_parse!(CssAtomSet::ATOMS, ScrollMarkerGroupStyleValue, "after");
		assert_parse!(CssAtomSet::ATOMS, ScrollMarkerGroupStyleValue, "before links");
		assert_peek_false!(CssAtomSet::ATOMS, ScrollMarkerGroupStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, ScrollMarkerGroupStyleValue, "1px");
	}

	#[test]
	fn test_line_clamp() {
		assert_parse!(CssAtomSet::ATOMS, LineClampStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, LineClampStyleValue, "3");
		assert_parse!(CssAtomSet::ATOMS, LineClampStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, LineClampStyleValue, "3 auto");
		assert_parse!(CssAtomSet::ATOMS, LineClampStyleValue, "3 -webkit-legacy");
		assert_parse!(CssAtomSet::ATOMS, LineClampStyleValue, "auto -webkit-legacy");
		assert_parse!(CssAtomSet::ATOMS, LineClampStyleValue, "3 auto -webkit-legacy");
		assert_peek_false!(CssAtomSet::ATOMS, LineClampStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, LineClampStyleValue, "-webkit-legacy");
		assert_parse_error!(CssAtomSet::ATOMS, LineClampStyleValue, "0");
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, OverflowXStyleValue, "scroll");
		assert_parse!(CssAtomSet::ATOMS, OverflowStyleValue, "hidden scroll");
	}

	#[test]
	fn test_text_overflow() {
		assert_parse!(CssAtomSet::ATOMS, TextOverflowStyleValue, "clip");
		assert_parse!(CssAtomSet::ATOMS, TextOverflowStyleValue, "ellipsis");
		assert_parse!(CssAtomSet::ATOMS, TextOverflowStyleValue, "\"...\"");
		assert_parse!(CssAtomSet::ATOMS, TextOverflowStyleValue, "clip ellipsis");
		assert_parse!(CssAtomSet::ATOMS, TextOverflowStyleValue, "ellipsis clip");
		assert_parse!(CssAtomSet::ATOMS, TextOverflowStyleValue, "clip clip");
		assert_parse!(CssAtomSet::ATOMS, TextOverflowStyleValue, "\"...\" ellipsis");
		assert_parse!(CssAtomSet::ATOMS, TextOverflowStyleValue, "fade");
		assert_parse!(CssAtomSet::ATOMS, TextOverflowStyleValue, "fade(2em)");
		assert_parse!(CssAtomSet::ATOMS, TextOverflowStyleValue, "clip fade(10%)");
		assert_peek_false!(CssAtomSet::ATOMS, TextOverflowStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, TextOverflowStyleValue, "1px");
	}
}
