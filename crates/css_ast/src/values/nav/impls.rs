#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SpatialNavigationActionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<SpatialNavigationContainStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<SpatialNavigationFunctionStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SpatialNavigationActionStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, SpatialNavigationActionStyleValue, "focus");
		assert_parse!(CssAtomSet::ATOMS, SpatialNavigationActionStyleValue, "scroll");
		assert_parse!(CssAtomSet::ATOMS, SpatialNavigationContainStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, SpatialNavigationContainStyleValue, "contain");
		assert_parse!(CssAtomSet::ATOMS, SpatialNavigationFunctionStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, SpatialNavigationFunctionStyleValue, "grid");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, SpatialNavigationActionStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, SpatialNavigationActionStyleValue, "auto focus");
		assert_parse_error!(CssAtomSet::ATOMS, SpatialNavigationContainStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, SpatialNavigationContainStyleValue, "auto contain");
		assert_parse_error!(CssAtomSet::ATOMS, SpatialNavigationFunctionStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, SpatialNavigationFunctionStyleValue, "normal grid");
	}
}
