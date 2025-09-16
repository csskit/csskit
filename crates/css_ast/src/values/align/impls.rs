pub(crate) use crate::{CssDiagnostic, traits::StyleValue};
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
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
		assert_parse!(AlignContentStyleValue, "normal");
		assert_parse!(AlignContentStyleValue, "safe flex-end");
		assert_parse!(AlignContentStyleValue, "flex-end");
		// assert_parse!(PlaceContentStyleValue, "unsafe flex-end");
		// assert_parse!(PlaceContentStyleValue, "flex-end");
		assert_parse!(AlignSelfStyleValue, "normal");
		assert_parse!(AlignSelfStyleValue, "safe flex-start");
		assert_parse!(AlignSelfStyleValue, "flex-start");
		assert_parse!(RowGapStyleValue, "normal");
		assert_parse!(ColumnGapStyleValue, "1px");
		assert_parse!(GapStyleValue, "normal 1px");
	}
}
