#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_parse() {
		assert_parse!(CssAtomSet::ATOMS, CursorStyleValue, "pointer");
		assert_parse!(CssAtomSet::ATOMS, CursorStyleValue, "grab");
		assert_parse!(CssAtomSet::ATOMS, CursorStyleValue, "-webkit-grab");
		assert_parse!(CssAtomSet::ATOMS, CursorStyleValue, "-webkit-grabbing");
		assert_parse!(CssAtomSet::ATOMS, NavDownStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, NavUpStyleValue, "#foo");
		assert_parse!(CssAtomSet::ATOMS, NavLeftStyleValue, "#foo current");
		assert_parse!(CssAtomSet::ATOMS, NavRightStyleValue, "#foo root");
		assert_parse!(CssAtomSet::ATOMS, NavRightStyleValue, "#foo \"frame\"");
	}

	#[test]
	fn test_parse_error() {
		assert_parse!(CssAtomSet::ATOMS, CursorStyleValue, "none");
	}
}
