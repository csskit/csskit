#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BorderCollapseStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderSpacingStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<CaptionSideStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<EmptyCellsStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TableLayoutStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BorderCollapseStyleValue, "separate");
		assert_parse!(CssAtomSet::ATOMS, BorderCollapseStyleValue, "collapse");

		assert_parse!(CssAtomSet::ATOMS, BorderSpacingStyleValue, "0px");
		assert_parse!(CssAtomSet::ATOMS, BorderSpacingStyleValue, "10px 20px");

		assert_parse!(CssAtomSet::ATOMS, CaptionSideStyleValue, "top");
		assert_parse!(CssAtomSet::ATOMS, CaptionSideStyleValue, "bottom");

		assert_parse!(CssAtomSet::ATOMS, EmptyCellsStyleValue, "show");
		assert_parse!(CssAtomSet::ATOMS, EmptyCellsStyleValue, "hide");

		assert_parse!(CssAtomSet::ATOMS, TableLayoutStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, TableLayoutStyleValue, "fixed");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BorderCollapseStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, BorderCollapseStyleValue, "separate collapse");

		assert_parse_error!(CssAtomSet::ATOMS, BorderSpacingStyleValue, "10%");
		assert_parse_error!(CssAtomSet::ATOMS, BorderSpacingStyleValue, "30");
		assert_parse_error!(CssAtomSet::ATOMS, BorderSpacingStyleValue, "40px 50px 60px");

		assert_parse_error!(CssAtomSet::ATOMS, CaptionSideStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, CaptionSideStyleValue, "left");
		assert_parse_error!(CssAtomSet::ATOMS, CaptionSideStyleValue, "top bottom");

		assert_parse_error!(CssAtomSet::ATOMS, EmptyCellsStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, EmptyCellsStyleValue, "show hide");

		assert_parse_error!(CssAtomSet::ATOMS, TableLayoutStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, TableLayoutStyleValue, "auto fixed");
	}
}
