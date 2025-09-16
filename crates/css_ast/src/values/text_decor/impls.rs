pub(crate) use crate::{CssDiagnostic, traits::StyleValue};
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		// assert_eq!(std::mem::size_of::<TextDecorationLineStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationColorStyleValue>(), 144);
		// assert_eq!(std::mem::size_of::<TextDecorationStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<TextUnderlinePositionStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<TextEmphasisStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextEmphasisColorStyleValue>(), 144);
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
		assert_parse!(TextDecorationTrimStyleValue, "1px 2px");
		assert_parse!(TextDecorationTrimStyleValue, "auto");

		assert_parse!(TextEmphasisSkipStyleValue, "spaces");
		assert_parse!(TextEmphasisSkipStyleValue, "punctuation");
		assert_parse!(TextEmphasisSkipStyleValue, "symbols");
		assert_parse!(TextEmphasisSkipStyleValue, "narrow");
		// Out of order keywords also work
		assert_parse!(TextEmphasisSkipStyleValue, "narrow symbols", "symbols narrow");
		assert_parse!(
			TextEmphasisSkipStyleValue,
			"punctuation symbols spaces narrow",
			"spaces punctuation symbols narrow"
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(TextEmphasisSkipStyleValue, "");
		assert_parse_error!(TextEmphasisSkipStyleValue, "spaces spaces");
		assert_parse_error!(TextEmphasisSkipStyleValue, "punctuation punctuation");
		assert_parse_error!(TextEmphasisSkipStyleValue, "foo");
		assert_parse_error!(TextEmphasisSkipStyleValue, "punctuation bar narrow");
	}
}
