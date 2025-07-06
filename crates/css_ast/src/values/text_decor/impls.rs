pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		// assert_eq!(std::mem::size_of::<TextDecorationLineStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextDecorationColorStyleValue>(), 160);
		// assert_eq!(std::mem::size_of::<TextDecorationStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<TextUnderlinePositionStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<TextEmphasisStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextEmphasisColorStyleValue>(), 160);
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
		// assert_eq!(std::mem::size_of::<TextEmphasisSkipStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TextDecorationTrimStyleValue, "1px 2px");
		assert_parse!(TextDecorationTrimStyleValue, "auto");
	}
}
