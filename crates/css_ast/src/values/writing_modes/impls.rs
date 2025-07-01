pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_parse_error() {
		assert_parse_error!(GlyphOrientationVerticalStyleValue, "128");
		assert_parse_error!(GlyphOrientationVerticalStyleValue, "50deg");
	}

	#[test]
	fn test_parse() {
		assert_parse!(GlyphOrientationVerticalStyleValue, "auto");
		assert_parse!(GlyphOrientationVerticalStyleValue, "0");
		assert_parse!(GlyphOrientationVerticalStyleValue, "90");
		assert_parse!(GlyphOrientationVerticalStyleValue, "90deg");
	}
}
