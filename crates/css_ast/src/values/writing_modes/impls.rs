pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DirectionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<UnicodeBidiStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<WritingModeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextOrientationStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<GlyphOrientationVerticalStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextCombineUprightStyleValue>(), 28);
	}

	#[test]
	fn test_parse() {
		assert_parse!(GlyphOrientationVerticalStyleValue, "auto");
		assert_parse!(GlyphOrientationVerticalStyleValue, "0");
		assert_parse!(GlyphOrientationVerticalStyleValue, "90");
		assert_parse!(GlyphOrientationVerticalStyleValue, "90deg");
		assert_parse!(TextCombineUprightStyleValue, "none");
		assert_parse!(TextCombineUprightStyleValue, "all");
		assert_parse!(TextCombineUprightStyleValue, "digits");
		assert_parse!(TextCombineUprightStyleValue, "digits 2");
		assert_parse!(TextCombineUprightStyleValue, "digits 4");
	}

	#[test]
	fn test_parse_error() {
		assert_parse_error!(GlyphOrientationVerticalStyleValue, "128");
		assert_parse_error!(GlyphOrientationVerticalStyleValue, "50deg");
		assert_parse_error!(GlyphOrientationVerticalStyleValue, "50deg");
		assert_parse_error!(TextCombineUprightStyleValue, "digits 1");
		assert_parse_error!(TextCombineUprightStyleValue, "digits 2 2");
		assert_parse_error!(TextCombineUprightStyleValue, "digits 5");
	}
}
