#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::{ColumnGapStyleValue, CssAtomSet, GapStyleValue, RowGapStyleValue};
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

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
