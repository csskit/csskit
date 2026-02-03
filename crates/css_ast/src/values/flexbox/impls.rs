#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FlexDirectionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FlexWrapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FlexFlowStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<FlexStyleValue>(), 68);
		assert_eq!(std::mem::size_of::<FlexGrowStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<FlexShrinkStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<FlexBasisStyleValue>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FlexBasisStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, FlexBasisStyleValue, "4px");
		assert_parse!(CssAtomSet::ATOMS, FlexStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, FlexStyleValue, "2");
		assert_parse!(CssAtomSet::ATOMS, FlexStyleValue, "10em");
		assert_parse!(CssAtomSet::ATOMS, FlexStyleValue, "30%");
		assert_parse!(CssAtomSet::ATOMS, FlexStyleValue, "min-content");
		assert_parse!(CssAtomSet::ATOMS, FlexStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, FlexStyleValue, "0 1 0px");
		assert_parse!(CssAtomSet::ATOMS, FlexStyleValue, "0 1 0px");
		assert_parse!(CssAtomSet::ATOMS, FlexStyleValue, "0 1 0%");
	}
}
