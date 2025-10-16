#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::Color;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorStyleValue>(), 140);
		assert_eq!(std::mem::size_of::<OpacityStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColorStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, OpacityStyleValue, "1");
	}
}
