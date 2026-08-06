#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ClipStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ClipStyleValue, "rect(10px,20px,30px,40px)");
		assert_parse!(CssAtomSet::ATOMS, ClipStyleValue, "rect(auto,auto,auto,auto)");
		assert_parse!(CssAtomSet::ATOMS, ClipStyleValue, "rect(-5px,auto,10px,0px)");

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

		assert_parse!(CssAtomSet::ATOMS, MaskStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MaskStyleValue, "url(mask.svg) luminance");
		assert_parse!(
			CssAtomSet::ATOMS,
			MaskStyleValue,
			"url(mask.svg) center / cover no-repeat border-box no-clip add alpha"
		);
		assert_parse!(CssAtomSet::ATOMS, MaskStyleValue, "url(a.svg), url(b.svg) add");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, ClipStyleValue, "none");
		assert_peek_false!(CssAtomSet::ATOMS, ClipRuleStyleValue, "auto");
		assert_peek_false!(CssAtomSet::ATOMS, ClipRuleStyleValue, "1");
		assert_parse_error!(CssAtomSet::ATOMS, ClipStyleValue, "rect(10px)");

		assert_peek_false!(CssAtomSet::ATOMS, MaskBorderModeStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, MaskBorderModeStyleValue, "luminance alpha");

		assert_peek_false!(CssAtomSet::ATOMS, MaskTypeStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, MaskTypeStyleValue, "luminance alpha");

		assert_peek_false!(CssAtomSet::ATOMS, MaskRepeatStyleValue, "auto");
		assert_peek_false!(CssAtomSet::ATOMS, MaskRepeatStyleValue, "repeat-z");

		assert_parse_error!(CssAtomSet::ATOMS, MaskSizeStyleValue, "-1px");
		assert_parse_error!(CssAtomSet::ATOMS, MaskSizeStyleValue, "1px 2px 3px");

		assert_peek_false!(CssAtomSet::ATOMS, MaskPositionStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, MaskPositionStyleValue, "1px 2px 3px");
		assert_parse_error!(CssAtomSet::ATOMS, MaskPositionStyleValue, "left right");

		assert_peek_false!(CssAtomSet::ATOMS, MaskStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, MaskStyleValue, "url(a.svg) url(b.svg)");
	}
}
