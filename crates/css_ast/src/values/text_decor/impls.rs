#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		// assert_eq!(std::mem::size_of::<TextDecorationLineStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationColorStyleValue>(), 140);
		// assert_eq!(std::mem::size_of::<TextDecorationStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<TextUnderlinePositionStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<TextEmphasisStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextEmphasisColorStyleValue>(), 140);
		// assert_eq!(std::mem::size_of::<TextEmphasisStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<TextEmphasisPositionStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<TextShadowStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationThicknessStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextUnderlineOffsetStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationTrimStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TextDecorationSkipStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<TextDecorationSkipSelfStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationSkipBoxStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<TextDecorationSkipSpacesStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationSkipInkStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextEmphasisSkipStyleValue>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TextDecorationTrimStyleValue, "1px 2px");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationTrimStyleValue, "auto");

		assert_parse!(CssAtomSet::ATOMS, TextEmphasisSkipStyleValue, "spaces");
		assert_parse!(CssAtomSet::ATOMS, TextEmphasisSkipStyleValue, "punctuation");
		assert_parse!(CssAtomSet::ATOMS, TextEmphasisSkipStyleValue, "symbols");
		assert_parse!(CssAtomSet::ATOMS, TextEmphasisSkipStyleValue, "narrow");
		// Out of order keywords also work
		assert_parse!(CssAtomSet::ATOMS, TextEmphasisSkipStyleValue, "narrow symbols");
		assert_parse!(CssAtomSet::ATOMS, TextEmphasisSkipStyleValue, "punctuation symbols spaces narrow");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TextEmphasisSkipStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, TextEmphasisSkipStyleValue, "spaces spaces");
		assert_parse_error!(CssAtomSet::ATOMS, TextEmphasisSkipStyleValue, "punctuation punctuation");
		assert_parse_error!(CssAtomSet::ATOMS, TextEmphasisSkipStyleValue, "foo");
		assert_parse_error!(CssAtomSet::ATOMS, TextEmphasisSkipStyleValue, "punctuation bar narrow");
	}
}
