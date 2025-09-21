#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		// assert_eq!(std::mem::size_of::<ContainerTypeStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<ContainerNameStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<ContainerStyleValue>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ContainerNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ContainerNameStyleValue, "a");
		assert_parse!(CssAtomSet::ATOMS, ContainerNameStyleValue, "a b c");
	}
}
