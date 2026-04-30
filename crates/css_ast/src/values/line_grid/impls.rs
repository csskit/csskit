#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BoxSnapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<LineGridStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<LineSnapStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BoxSnapStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, BoxSnapStyleValue, "block-start");
		assert_parse!(CssAtomSet::ATOMS, BoxSnapStyleValue, "block-end");
		assert_parse!(CssAtomSet::ATOMS, BoxSnapStyleValue, "center");
		assert_parse!(CssAtomSet::ATOMS, BoxSnapStyleValue, "baseline");
		assert_parse!(CssAtomSet::ATOMS, BoxSnapStyleValue, "last-baseline");
		assert_parse!(CssAtomSet::ATOMS, LineGridStyleValue, "match-parent");
		assert_parse!(CssAtomSet::ATOMS, LineGridStyleValue, "create");
		assert_parse!(CssAtomSet::ATOMS, LineSnapStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, LineSnapStyleValue, "baseline");
		assert_parse!(CssAtomSet::ATOMS, LineSnapStyleValue, "contain");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BoxSnapStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, BoxSnapStyleValue, "none center");
		assert_parse_error!(CssAtomSet::ATOMS, LineGridStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, LineGridStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, LineGridStyleValue, "match-parent create");
		assert_parse_error!(CssAtomSet::ATOMS, LineSnapStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, LineSnapStyleValue, "none baseline");
	}
}
