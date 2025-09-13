#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AlignContentStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<JustifyContentStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<PlaceContentStyleValue>(), 48);
		// assert_eq!(std::mem::size_of::<JustifySelfStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<AlignSelfStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<PlaceSelfStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<JustifyItemsStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<AlignItemsStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<PlaceItemsStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<RowGapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnGapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<GapStyleValue>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AlignContentStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, AlignContentStyleValue, "safe flex-end");
		assert_parse!(CssAtomSet::ATOMS, AlignContentStyleValue, "flex-end");
		// assert_parse!(CssAtomSet::ATOMS, PlaceContentStyleValue, "unsafe flex-end");
		// assert_parse!(CssAtomSet::ATOMS, PlaceContentStyleValue, "flex-end");
		assert_parse!(CssAtomSet::ATOMS, AlignSelfStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, AlignSelfStyleValue, "safe flex-start");
		assert_parse!(CssAtomSet::ATOMS, AlignSelfStyleValue, "flex-start");
		assert_parse!(CssAtomSet::ATOMS, RowGapStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, ColumnGapStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, GapStyleValue, "normal 1px");
	}
}
