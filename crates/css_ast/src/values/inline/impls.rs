#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<LineHeightStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextBoxStyleValue>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineHeightStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, LineHeightStyleValue, "1.618");

		assert_parse!(CssAtomSet::ATOMS, TextBoxStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, TextBoxStyleValue, "trim-start auto");
		assert_parse!(CssAtomSet::ATOMS, TextBoxStyleValue, "auto trim-start");
	}
}
