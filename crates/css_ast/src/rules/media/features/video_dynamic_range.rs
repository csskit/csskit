use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum VideoDynamicRangeMediaFeature<"video-dynamic-range", VideoDynamicRangeMediaFeatureKeyword>
);

keyword_set!(pub enum VideoDynamicRangeMediaFeatureKeyword { Standard: "standard", Hight: "high" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<VideoDynamicRangeMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(VideoDynamicRangeMediaFeature, "(video-dynamic-range)");
		assert_parse!(VideoDynamicRangeMediaFeature, "(video-dynamic-range:standard)");
		assert_parse!(VideoDynamicRangeMediaFeature, "(video-dynamic-range:high)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(VideoDynamicRangeMediaFeature, "(video-dynamic-range:)");
		assert_parse_error!(VideoDynamicRangeMediaFeature, "(video-dynamic-range: low)");
	}
}
