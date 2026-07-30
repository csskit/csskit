#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PositionStyleValue, "sticky");
		assert_parse!(CssAtomSet::ATOMS, PositionStyleValue, "-webkit-sticky");
		assert_parse!(CssAtomSet::ATOMS, InsetBlockStartStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, InsetStyleValue, "1px 2px");
		assert_parse!(CssAtomSet::ATOMS, InsetStyleValue, "1px 2px 3px 4px");
	}
}
