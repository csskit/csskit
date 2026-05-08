#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FlowFromStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FlowIntoStyleValue>(), 28);
		assert_eq!(std::mem::size_of::<RegionFragmentStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FlowFromStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, FlowFromStyleValue, "myflow");
		assert_parse!(CssAtomSet::ATOMS, FlowIntoStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, FlowIntoStyleValue, "myflow");
		assert_parse!(CssAtomSet::ATOMS, FlowIntoStyleValue, "myflow element");
		assert_parse!(CssAtomSet::ATOMS, FlowIntoStyleValue, "myflow content");
		assert_parse!(CssAtomSet::ATOMS, FlowIntoStyleValue, "element content");
		assert_parse!(CssAtomSet::ATOMS, FlowIntoStyleValue, "element element");
		assert_parse!(CssAtomSet::ATOMS, FlowIntoStyleValue, "content content");
		assert_parse!(CssAtomSet::ATOMS, RegionFragmentStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, RegionFragmentStyleValue, "break");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FlowFromStyleValue, "none myflow");
		assert_parse_error!(CssAtomSet::ATOMS, FlowIntoStyleValue, "element myflow");
		assert_parse_error!(CssAtomSet::ATOMS, FlowIntoStyleValue, "content myflow");
		assert_parse_error!(CssAtomSet::ATOMS, RegionFragmentStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, RegionFragmentStyleValue, "auto break");
	}
}
