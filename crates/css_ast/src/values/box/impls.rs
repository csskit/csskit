// shortcuts for logical properties to resolve to 0
// impl MarginTop {
// 	#[allow(non_upper_case_globals)]
// 	pub const Zero: MarginTop = MarginTop::LengthPercentage(LengthPercentage::Zero);
// }

// impl PaddingTop {
// 	#[allow(non_upper_case_globals)]
// 	pub const Zero: PaddingTop = PaddingTop(LengthPercentage::Zero);
// }

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<MarginTopStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<MarginRightStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<MarginBottomStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<MarginLeftStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<MarginStyleValue>(), 96);
		assert_eq!(std::mem::size_of::<PaddingTopStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<PaddingRightStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<PaddingBottomStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<PaddingLeftStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<PaddingStyleValue>(), 96);
		assert_eq!(std::mem::size_of::<MarginTrimStyleValue>(), 64);
	}

	#[test]
	fn test_margin_trim() {
		assert_parse!(CssAtomSet::ATOMS, MarginTrimStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MarginTrimStyleValue, "block");
		assert_parse!(CssAtomSet::ATOMS, MarginTrimStyleValue, "block-start");
		assert_parse!(CssAtomSet::ATOMS, MarginTrimStyleValue, "block-start block-end");
		assert_parse!(CssAtomSet::ATOMS, MarginTrimStyleValue, "inline");
		assert_parse!(CssAtomSet::ATOMS, MarginTrimStyleValue, "inline-start");
		assert_parse!(CssAtomSet::ATOMS, MarginTrimStyleValue, "inline-end");
		assert_parse!(CssAtomSet::ATOMS, MarginTrimStyleValue, "block inline");
		assert_peek_false!(CssAtomSet::ATOMS, MarginTrimStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, MarginTrimStyleValue, "auto");
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, MarginLeftStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MarginStyleValue, "1px 1px");
		assert_parse!(CssAtomSet::ATOMS, MarginStyleValue, "1px 2px");
		assert_parse!(CssAtomSet::ATOMS, MarginStyleValue, "1px 2px 3px 4px");
	}
}
