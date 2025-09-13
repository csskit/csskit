#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		// assert_eq!(std::mem::size_of::<ScrollSnapTypeStyleValue>(), 2);
		// assert_eq!(std::mem::size_of::<ScrollPaddingStyleValue>(), 2);
		// assert_eq!(std::mem::size_of::<ScrollMarginStyleValue>(), 2);
		assert_eq!(std::mem::size_of::<ScrollSnapAlignStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ScrollSnapStopStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollPaddingTopStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollPaddingRightStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollPaddingBottomStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollPaddingLeftStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollPaddingInlineStartStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollPaddingBlockStartStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollPaddingInlineEndStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollPaddingBlockEndStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<ScrollPaddingBlockStyleValue>(), 2);
		// assert_eq!(std::mem::size_of::<ScrollPaddingInlineStyleValue>(), 2);
		assert_eq!(std::mem::size_of::<ScrollMarginTopStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollMarginRightStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollMarginBottomStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollMarginLeftStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollMarginBlockStartStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollMarginInlineStartStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollMarginBlockEndStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollMarginInlineEndStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollMarginBlockStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ScrollMarginInlineStyleValue>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ScrollPaddingTopStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, ScrollMarginTopStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, ScrollSnapAlignStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ScrollSnapAlignStyleValue, "start end");
		assert_parse!(CssAtomSet::ATOMS, ScrollSnapAlignStyleValue, "center center");
	}
}
