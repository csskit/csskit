#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FlexDirectionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FlexWrapStyleValue>(), 36);
		assert_eq!(std::mem::size_of::<FlexFlowStyleValue>(), 52);
		assert_eq!(std::mem::size_of::<FlexStyleValue>(), 68);
		assert_eq!(std::mem::size_of::<FlexGrowStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<FlexShrinkStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<FlexBasisStyleValue>(), 40);
	}

	#[test]
	fn test_flex_wrap_writes() {
		assert_parse!(CssAtomSet::ATOMS, FlexWrapStyleValue, "nowrap");
		assert_parse!(CssAtomSet::ATOMS, FlexWrapStyleValue, "wrap");
		assert_parse!(CssAtomSet::ATOMS, FlexWrapStyleValue, "wrap-reverse");
		assert_parse!(CssAtomSet::ATOMS, FlexWrapStyleValue, "balance");
	}

	#[test]
	fn test_flex_wrap_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FlexWrapStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, FlexWrapStyleValue, "reverse");
	}

	#[test]
	fn test_flex_direction_writes() {
		assert_parse!(CssAtomSet::ATOMS, FlexDirectionStyleValue, "row");
		assert_parse!(CssAtomSet::ATOMS, FlexDirectionStyleValue, "row-reverse");
		assert_parse!(CssAtomSet::ATOMS, FlexDirectionStyleValue, "column");
		assert_parse!(CssAtomSet::ATOMS, FlexDirectionStyleValue, "column-reverse");
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
