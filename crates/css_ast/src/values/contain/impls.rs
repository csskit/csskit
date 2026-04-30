#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ContentVisibilityStyleValue>(), 16);
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
}
