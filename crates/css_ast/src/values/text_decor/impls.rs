#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<TextDecorationLineStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<TextDecorationStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationColorStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<TextDecorationStyleValue>(), 128);
		assert_eq!(std::mem::size_of::<TextUnderlinePositionStyleValue>(), 36);
		// assert_eq!(std::mem::size_of::<TextEmphasisStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextEmphasisColorStyleValue>(), 24);
		// assert_eq!(std::mem::size_of::<TextEmphasisStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextEmphasisPositionStyleValue>(), 28);
		assert_eq!(std::mem::size_of::<TextShadowStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TextDecorationThicknessStyleValue>(), 20);
		assert_eq!(std::mem::size_of::<TextUnderlineOffsetStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationInsetStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TextDecorationSkipStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationSkipSelfStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<TextDecorationSkipBoxStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationSkipSpacesStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<TextDecorationSkipInkStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextEmphasisSkipStyleValue>(), 64);
	}

	#[test]
	fn test_text_underline_position() {
		assert_parse!(CssAtomSet::ATOMS, TextUnderlinePositionStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, TextUnderlinePositionStyleValue, "from-font");
		assert_parse!(CssAtomSet::ATOMS, TextUnderlinePositionStyleValue, "under");
		assert_parse!(CssAtomSet::ATOMS, TextUnderlinePositionStyleValue, "from-font left");
		assert_parse!(CssAtomSet::ATOMS, TextUnderlinePositionStyleValue, "left");
		assert_parse!(CssAtomSet::ATOMS, TextUnderlinePositionStyleValue, "right");
		assert_parse_error!(CssAtomSet::ATOMS, TextUnderlinePositionStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, TextUnderlinePositionStyleValue, "1px");
	}

	#[test]
	fn test_text_decoration_line() {
		assert_parse!(CssAtomSet::ATOMS, TextDecorationLineStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationLineStyleValue, "underline");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationLineStyleValue, "overline");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationLineStyleValue, "line-through");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationLineStyleValue, "underline overline");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationLineStyleValue, "underline line-through overline");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationLineStyleValue, "spelling-error");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationLineStyleValue, "grammar-error");
		assert_parse_error!(CssAtomSet::ATOMS, TextDecorationLineStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, TextDecorationLineStyleValue, "none underline");
	}

	#[test]
	fn test_text_decoration_skip_self() {
		assert_parse!(CssAtomSet::ATOMS, TextDecorationSkipSelfStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationSkipSelfStyleValue, "skip-all");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationSkipSelfStyleValue, "no-skip");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationSkipSelfStyleValue, "skip-underline");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationSkipSelfStyleValue, "skip-overline");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationSkipSelfStyleValue, "skip-line-through");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationSkipSelfStyleValue, "skip-underline skip-overline");
		assert_parse_error!(CssAtomSet::ATOMS, TextDecorationSkipSelfStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, TextDecorationSkipSelfStyleValue, "left");
	}

	#[test]
	fn test_text_decoration_skip_spaces() {
		assert_parse!(CssAtomSet::ATOMS, TextDecorationSkipSpacesStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationSkipSpacesStyleValue, "all");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationSkipSpacesStyleValue, "start");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationSkipSpacesStyleValue, "end");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationSkipSpacesStyleValue, "start end");
		assert_parse_error!(CssAtomSet::ATOMS, TextDecorationSkipSpacesStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, TextDecorationSkipSpacesStyleValue, "left");
	}

	#[test]
	fn test_text_decoration() {
		// single longhand
		assert_parse!(CssAtomSet::ATOMS, TextDecorationStyleValue, "underline");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationStyleValue, "solid");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationStyleValue, "2px");
		// combinations
		assert_parse!(CssAtomSet::ATOMS, TextDecorationStyleValue, "underline solid");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationStyleValue, "underline solid red");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationStyleValue, "underline dotted red 2px");
		// order should not matter (|| combinator)
		assert_parse!(CssAtomSet::ATOMS, TextDecorationStyleValue, "red underline");
		// errors
		assert_parse_error!(CssAtomSet::ATOMS, TextDecorationStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, TextDecorationStyleValue, "invalid");
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TextDecorationInsetStyleValue, "1px 2px");
		assert_parse!(CssAtomSet::ATOMS, TextDecorationInsetStyleValue, "auto");

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
