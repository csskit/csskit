#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OverflowXStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverflowYStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverflowBlockStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverflowInlineStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverflowStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ScrollBehaviorStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollbarGutterStyleValue>(), 28);
		// assert_eq!(std::mem::size_of::<TextOverflowStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<OverflowClipMarginTopStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginRightStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginBottomStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginLeftStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginInlineStartStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginBlockEndStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginInlineEndStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginInlineStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginBlockStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverflowClipMarginBlockStartStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BlockEllipsisStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<LineClampStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<WebkitLineClampStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MaxLinesStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ContinueStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollTargetGroupStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollMarkerGroupStyleValue>(), 36);
	}

	#[test]
	fn test_scrollbar_gutter() {
		assert_parse!(CssAtomSet::ATOMS, ScrollbarGutterStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ScrollbarGutterStyleValue, "stable");
		assert_parse!(CssAtomSet::ATOMS, ScrollbarGutterStyleValue, "stable both-edges");
		assert_parse_error!(CssAtomSet::ATOMS, ScrollbarGutterStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, ScrollbarGutterStyleValue, "1px");
	}

	#[test]
	fn test_scroll_marker_group() {
		assert_parse!(CssAtomSet::ATOMS, ScrollMarkerGroupStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ScrollMarkerGroupStyleValue, "before");
		assert_parse!(CssAtomSet::ATOMS, ScrollMarkerGroupStyleValue, "after");
		assert_parse!(CssAtomSet::ATOMS, ScrollMarkerGroupStyleValue, "before links");
		assert_parse_error!(CssAtomSet::ATOMS, ScrollMarkerGroupStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, ScrollMarkerGroupStyleValue, "1px");
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
		assert_parse_error!(CssAtomSet::ATOMS, LineClampStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, LineClampStyleValue, "-webkit-legacy");
		assert_parse_error!(CssAtomSet::ATOMS, LineClampStyleValue, "0");
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, OverflowXStyleValue, "scroll");
		assert_parse!(CssAtomSet::ATOMS, OverflowStyleValue, "hidden scroll");
	}
}
