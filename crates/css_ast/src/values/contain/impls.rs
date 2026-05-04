#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ContentVisibilityStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ContainStyleValue>(), 68);
	}

	#[test]
	fn test_content_visibility() {
		assert_parse!(CssAtomSet::ATOMS, ContentVisibilityStyleValue, "visible");
		assert_parse!(CssAtomSet::ATOMS, ContentVisibilityStyleValue, "hidden");
		assert_parse!(CssAtomSet::ATOMS, ContentVisibilityStyleValue, "auto");
	}

	#[test]
	fn test_content_visibility_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ContentVisibilityStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, ContentVisibilityStyleValue, "visible hidden");
	}

	#[test]
	fn test_contain() {
		assert_parse!(CssAtomSet::ATOMS, ContainStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ContainStyleValue, "strict");
		assert_parse!(CssAtomSet::ATOMS, ContainStyleValue, "content");
		assert_parse!(CssAtomSet::ATOMS, ContainStyleValue, "size");
		assert_parse!(CssAtomSet::ATOMS, ContainStyleValue, "inline-size");
		assert_parse!(CssAtomSet::ATOMS, ContainStyleValue, "layout");
		assert_parse!(CssAtomSet::ATOMS, ContainStyleValue, "style");
		assert_parse!(CssAtomSet::ATOMS, ContainStyleValue, "paint");
		assert_parse!(CssAtomSet::ATOMS, ContainStyleValue, "size layout");
		assert_parse!(CssAtomSet::ATOMS, ContainStyleValue, "size layout style paint");
		assert_parse_error!(CssAtomSet::ATOMS, ContainStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, ContainStyleValue, "auto");
	}
}
