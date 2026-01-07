#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<DisplayStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<OrderStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<ReadingFlowStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ReadingOrderStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<VisibilityStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		// https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/display#syntax
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "block");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "inline");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "inline-block");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "flex");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "inline-flex");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "grid");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "inline-grid");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "flow-root");

		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "contents");

		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "block flex");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "block flow");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "block flow-root");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "block grid");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "inline flex");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "inline flow");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "inline flow-root");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "inline grid");

		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "table");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "table-row");
		assert_parse!(CssAtomSet::ATOMS, DisplayStyleValue, "list-item");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, DisplayStyleValue, "block none");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayStyleValue, "table-row flex");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayStyleValue, "flex flex");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayStyleValue, "flow flex");

		assert_parse_error!(CssAtomSet::ATOMS, DisplayStyleValue, "inherit");
	}
}
