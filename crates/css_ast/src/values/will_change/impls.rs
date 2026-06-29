#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, WillChangeStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, WillChangeStyleValue, "foo,bar,baz");
		assert_parse!(CssAtomSet::ATOMS, WillChangeStyleValue, "-webkit-perspective");
		assert_parse!(CssAtomSet::ATOMS, WillChangeStyleValue, "transform,filter,mask");
	}

	#[test]
	fn test_peek() {
		assert_peek_false!(CssAtomSet::ATOMS, WillChangeStyleValue, ""); // must be at-least-one
		assert_peek_false!(CssAtomSet::ATOMS, WillChangeStyleValue, "0px 3px"); // dimensions not idents
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WillChangeStyleValue, "auto auto"); // two autos is illegal
		assert_parse_error!(CssAtomSet::ATOMS, WillChangeStyleValue, "transform filter"); // no commas
	}
}
