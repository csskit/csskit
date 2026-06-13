#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::{ColumnGapStyleValue, CssAtomSet, GapStyleValue, RowGapStyleValue};
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AlignContentStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<JustifyContentStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<PlaceContentStyleValue>(), 96);
		assert_eq!(std::mem::size_of::<JustifySelfStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<AlignSelfStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<PlaceSelfStyleValue>(), 96);
		assert_eq!(std::mem::size_of::<JustifyItemsStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<AlignItemsStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<PlaceItemsStyleValue>(), 96);
		assert_eq!(std::mem::size_of::<RowGapStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ColumnGapStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<GapStyleValue>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AlignContentStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, AlignContentStyleValue, "safe flex-end");
		assert_parse!(CssAtomSet::ATOMS, AlignContentStyleValue, "flex-end");
		// assert_parse!(CssAtomSet::ATOMS, PlaceContentStyleValue, "unsafe flex-end");
		// assert_parse!(CssAtomSet::ATOMS, PlaceContentStyleValue, "flex-end");
		assert_parse!(CssAtomSet::ATOMS, AlignSelfStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, AlignSelfStyleValue, "safe normal");
		assert_parse!(CssAtomSet::ATOMS, AlignSelfStyleValue, "safe end");
		assert_parse!(CssAtomSet::ATOMS, AlignSelfStyleValue, "safe flex-start");
		assert_parse!(CssAtomSet::ATOMS, AlignSelfStyleValue, "flex-start");
		assert_parse!(CssAtomSet::ATOMS, RowGapStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, ColumnGapStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, GapStyleValue, "normal 1px");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, AlignSelfStyleValue, "none");
	}
}
