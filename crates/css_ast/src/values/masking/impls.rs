#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ClipRuleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MaskBorderModeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MaskBorderOutsetStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<MaskBorderRepeatStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MaskBorderSourceStyleValue>(), 128);
		assert_eq!(std::mem::size_of::<MaskClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MaskOriginStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MaskPositionStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MaskRepeatStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MaskSizeStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MaskTypeStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ClipRuleStyleValue, "nonzero");
		assert_parse!(CssAtomSet::ATOMS, ClipRuleStyleValue, "evenodd");

		assert_parse!(CssAtomSet::ATOMS, MaskBorderModeStyleValue, "luminance");
		assert_parse!(CssAtomSet::ATOMS, MaskBorderModeStyleValue, "alpha");

		assert_parse!(CssAtomSet::ATOMS, MaskTypeStyleValue, "luminance");
		assert_parse!(CssAtomSet::ATOMS, MaskTypeStyleValue, "alpha");

		assert_parse!(CssAtomSet::ATOMS, MaskRepeatStyleValue, "repeat-x");
		assert_parse!(CssAtomSet::ATOMS, MaskRepeatStyleValue, "repeat-y");
		assert_parse!(CssAtomSet::ATOMS, MaskRepeatStyleValue, "repeat");
		assert_parse!(CssAtomSet::ATOMS, MaskRepeatStyleValue, "no-repeat");

		assert_parse!(CssAtomSet::ATOMS, MaskSizeStyleValue, "1px auto");
		assert_parse!(CssAtomSet::ATOMS, MaskSizeStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MaskSizeStyleValue, "contain");
		assert_parse!(CssAtomSet::ATOMS, MaskSizeStyleValue, "cover");

		assert_parse!(CssAtomSet::ATOMS, MaskPositionStyleValue, "-20% -30px");
		assert_parse!(CssAtomSet::ATOMS, MaskPositionStyleValue, "left center");
		assert_parse!(CssAtomSet::ATOMS, MaskPositionStyleValue, "center center");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ClipRuleStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, ClipRuleStyleValue, "1");

		assert_parse_error!(CssAtomSet::ATOMS, MaskBorderModeStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, MaskBorderModeStyleValue, "luminance alpha");

		assert_parse_error!(CssAtomSet::ATOMS, MaskTypeStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, MaskTypeStyleValue, "luminance alpha");

		assert_parse_error!(CssAtomSet::ATOMS, MaskRepeatStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, MaskRepeatStyleValue, "repeat-z");

		assert_parse_error!(CssAtomSet::ATOMS, MaskSizeStyleValue, "-1px");
		assert_parse_error!(CssAtomSet::ATOMS, MaskSizeStyleValue, "1px 2px 3px");

		assert_parse_error!(CssAtomSet::ATOMS, MaskPositionStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, MaskPositionStyleValue, "1px 2px 3px");
		assert_parse_error!(CssAtomSet::ATOMS, MaskPositionStyleValue, "left right");
	}
}
