use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum OrientationMediaFeature<"orientation", OrientationMediaFeatureKeyword>
);

keyword_set!(pub enum OrientationMediaFeatureKeyword { Portrait: "portrait", Landscape: "landscape" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OrientationMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OrientationMediaFeature, "(orientation)");
		assert_parse!(OrientationMediaFeature, "(orientation:portrait)");
		assert_parse!(OrientationMediaFeature, "(orientation:landscape)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(OrientationMediaFeature, "(orientation:)");
		assert_parse_error!(OrientationMediaFeature, "(orientation: landscope)");
	}
}
