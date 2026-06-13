#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ScrollSnapTypeStyleValue>(), 28);
		assert_eq!(std::mem::size_of::<ScrollPaddingStyleValue>(), 96);
		assert_eq!(std::mem::size_of::<ScrollMarginStyleValue>(), 96);
		assert_eq!(std::mem::size_of::<ScrollSnapAlignStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ScrollSnapStopStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScrollPaddingTopStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollPaddingRightStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollPaddingBottomStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollPaddingLeftStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollPaddingInlineStartStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollPaddingBlockStartStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollPaddingInlineEndStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollPaddingBlockEndStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollPaddingBlockStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<ScrollPaddingInlineStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<ScrollMarginTopStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollMarginRightStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollMarginBottomStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollMarginLeftStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollMarginBlockStartStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollMarginInlineStartStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollMarginBlockEndStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollMarginInlineEndStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<ScrollMarginBlockStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<ScrollMarginInlineStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<ScrollInitialTargetStyleValue>(), 16);
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
