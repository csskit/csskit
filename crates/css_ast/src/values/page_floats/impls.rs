#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FloatStyleValue, "left");
		assert_parse!(CssAtomSet::ATOMS, FloatStyleValue, "snap-block(1px,near)");
		assert_parse!(CssAtomSet::ATOMS, FloatStyleValue, "snap-inline(1px,near)");
	}
}
