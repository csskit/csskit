#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ScrollPaddingTopStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, ScrollMarginTopStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, ScrollSnapAlignStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ScrollSnapAlignStyleValue, "start end");
		assert_parse!(CssAtomSet::ATOMS, ScrollSnapAlignStyleValue, "center center");
	}
}
