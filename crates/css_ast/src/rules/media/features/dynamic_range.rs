use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum DynamicRangeMediaFeature<"dynamic-range", DynamicRangeMediaFeatureKeyword>
);

keyword_set!(pub enum DynamicRangeMediaFeatureKeyword { Standard: "standard", High: "high" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DynamicRangeMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DynamicRangeMediaFeature, "(dynamic-range)");
		assert_parse!(DynamicRangeMediaFeature, "(dynamic-range:standard)");
		assert_parse!(DynamicRangeMediaFeature, "(dynamic-range:high)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(DynamicRangeMediaFeature, "(dynamic-range:)");
		assert_parse_error!(DynamicRangeMediaFeature, "(dynamic-range: pointer)");
		assert_parse_error!(DynamicRangeMediaFeature, "(pointer: standard)");
	}
}
