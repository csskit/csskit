#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<ViewTransitionNameStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ViewTransitionClassStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ViewTransitionGroupStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ViewTransitionNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ViewTransitionNameStyleValue, "foo");
		assert_parse!(CssAtomSet::ATOMS, ViewTransitionClassStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ViewTransitionClassStyleValue, "foo");
		assert_parse!(CssAtomSet::ATOMS, ViewTransitionClassStyleValue, "foo bar baz");
		assert_parse!(CssAtomSet::ATOMS, ViewTransitionGroupStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, ViewTransitionGroupStyleValue, "nearest");
		assert_parse!(CssAtomSet::ATOMS, ViewTransitionGroupStyleValue, "foo");
	}
}
