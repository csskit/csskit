#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<LineHeightStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TextBoxStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<AlignmentBaselineStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BaselineShiftStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BaselineSourceStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<DominantBaselineStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<InitialLetterStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<InitialLetterWrapStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<InlineSizingStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<LineFitEdgeStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<TextBoxEdgeStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<TextBoxTrimStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<VerticalAlignStyleValue>(), 72);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineHeightStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, LineHeightStyleValue, "1.618");

		assert_parse!(CssAtomSet::ATOMS, TextBoxStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, TextBoxStyleValue, "trim-start auto");
		assert_parse!(CssAtomSet::ATOMS, TextBoxStyleValue, "auto trim-start");
	}

	#[test]
	fn test_vertical_align() {
		use css_parse::{assert_parse_error, assert_peek_false};
		// [ first | last ] || <'alignment-baseline'> || <'baseline-shift'>
		assert_parse!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "first");
		assert_parse!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "last");
		assert_parse!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "baseline");
		assert_parse!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "sub");
		assert_parse!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "first baseline");
		assert_peek_false!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, VerticalAlignStyleValue, "first last");
	}
}
