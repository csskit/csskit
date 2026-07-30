#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FieldSizingStyleValue, "content");
		assert_parse!(CssAtomSet::ATOMS, SliderOrientationStyleValue, "bottom-to-top");
		assert_parse!(CssAtomSet::ATOMS, InputSecurityStyleValue, "none");
	}
}
