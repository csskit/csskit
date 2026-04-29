#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<LineHeightStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextBoxStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AlignmentBaselineStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BaselineShiftStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BaselineSourceStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<DominantBaselineStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<InitialLetterStyleValue>(), 28);
		assert_eq!(std::mem::size_of::<InitialLetterWrapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<InlineSizingStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<LineFitEdgeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextBoxEdgeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextBoxTrimStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineHeightStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, LineHeightStyleValue, "1.618");

		assert_parse!(CssAtomSet::ATOMS, TextBoxStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, TextBoxStyleValue, "trim-start auto");
		assert_parse!(CssAtomSet::ATOMS, TextBoxStyleValue, "auto trim-start");
	}
}
