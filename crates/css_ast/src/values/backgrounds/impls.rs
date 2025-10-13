#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BackgroundColorStyleValue>(), 140);
		// assert_eq!(std::mem::size_of::<BackgroundImageStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BackgroundRepeatStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BackgroundAttachmentStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<BackgroundPositionStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BackgroundClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BackgroundOriginStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<BackgroundSizeStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BackgroundStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BackgroundRepeatXStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BackgroundRepeatYStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BackgroundRepeatBlockStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BackgroundRepeatInlineStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<BackgroundPositionXStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BackgroundPositionYStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BackgroundPositionInlineStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BackgroundPositionBlockStyleValue>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BackgroundRepeatStyleValue, "repeat-x");
		assert_parse!(CssAtomSet::ATOMS, BackgroundRepeatStyleValue, "space round");
	}
}
