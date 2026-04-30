#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WrapFlowStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<WrapThroughStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, WrapFlowStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, WrapFlowStyleValue, "both");
		assert_parse!(CssAtomSet::ATOMS, WrapFlowStyleValue, "start");
		assert_parse!(CssAtomSet::ATOMS, WrapFlowStyleValue, "end");
		assert_parse!(CssAtomSet::ATOMS, WrapFlowStyleValue, "minimum");
		assert_parse!(CssAtomSet::ATOMS, WrapFlowStyleValue, "maximum");
		assert_parse!(CssAtomSet::ATOMS, WrapFlowStyleValue, "clear");
		assert_parse!(CssAtomSet::ATOMS, WrapThroughStyleValue, "wrap");
		assert_parse!(CssAtomSet::ATOMS, WrapThroughStyleValue, "none");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WrapFlowStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, WrapFlowStyleValue, "auto both");
		assert_parse_error!(CssAtomSet::ATOMS, WrapThroughStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, WrapThroughStyleValue, "wrap none");
	}
}
