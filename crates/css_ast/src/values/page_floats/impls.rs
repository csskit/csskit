#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FloatReferenceStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FloatStyleValue>(), 92);
		assert_eq!(std::mem::size_of::<ClearStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FloatDeferStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FloatOffsetStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FloatStyleValue, "left");
		assert_parse!(CssAtomSet::ATOMS, FloatStyleValue, "snap-block(1px,near)");
		assert_parse!(CssAtomSet::ATOMS, FloatStyleValue, "snap-inline(1px,near)");
	}
}
