#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<HyphenateLimitCharsStyleValue>(), 48);
	}

	#[test]
	fn test_hyphenate_limit_chars() {
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "5");
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "auto 3");
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "5 2 2");
		assert_parse!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "auto auto auto");
		assert_parse_error!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, HyphenateLimitCharsStyleValue, "5 2 2 2");
	}
}
