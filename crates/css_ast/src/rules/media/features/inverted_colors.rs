use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum InvertedColorsMediaFeature<"inverted-colors", InvertedColorsMediaFeatureKeyword>
);

keyword_set!(pub enum InvertedColorsMediaFeatureKeyword { None: "none", Inverted: "inverted" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<InvertedColorsMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(InvertedColorsMediaFeature, "(inverted-colors)");
		assert_parse!(InvertedColorsMediaFeature, "(inverted-colors:inverted)");
		assert_parse!(InvertedColorsMediaFeature, "(inverted-colors:none)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(InvertedColorsMediaFeature, "(inverted-colors:)");
		assert_parse_error!(InvertedColorsMediaFeature, "(inverted-colors: invited)");
	}
}
