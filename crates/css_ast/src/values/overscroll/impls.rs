#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<OverscrollBehaviorStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<OverscrollBehaviorXStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverscrollBehaviorYStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverscrollBehaviorInlineStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverscrollBehaviorBlockStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, OverscrollBehaviorStyleValue, "contain");
		assert_parse!(CssAtomSet::ATOMS, OverscrollBehaviorStyleValue, "contain none");
		assert_parse!(CssAtomSet::ATOMS, OverscrollBehaviorInlineStyleValue, "contain");
	}
}
