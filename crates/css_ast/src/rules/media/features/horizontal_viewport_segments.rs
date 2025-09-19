use super::prelude::*;
use crate::units::CSSInt;

keyword_set!(pub enum HorizontalViewportSegmentsMediaFeatureKeyword {
	HorizontalViewportSegments: "horizontal-viewport-segments",
	MaxHorizontalViewportSegments: "max-horizontal-viewport-segments",
	MinHorizontalViewportSegments: "min-horizontal-viewport-segments",
});

impl RangedFeatureKeyword for HorizontalViewportSegmentsMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxHorizontalViewportSegments(_) | Self::MinHorizontalViewportSegments(_))
	}
}

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum HorizontalViewportSegmentsMediaFeature<HorizontalViewportSegmentsMediaFeatureKeyword, CSSInt>
);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<HorizontalViewportSegmentsMediaFeature>(), 120);
	}

	#[test]
	fn test_writes() {
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments:2)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments:8)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(min-horizontal-viewport-segments:2)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(max-horizontal-viewport-segments:2)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments<=3)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments>=5)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments>=8)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments=16)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(6=horizontal-viewport-segments)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(2<=horizontal-viewport-segments)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(2<horizontal-viewport-segments<4)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(4>horizontal-viewport-segments<8)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(4>=horizontal-viewport-segments<=8)");
		assert_parse!(HorizontalViewportSegmentsMediaFeature, "(4<=horizontal-viewport-segments>8)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments:)");
		assert_parse_error!(HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments: > 10px)");
		assert_parse_error!(HorizontalViewportSegmentsMediaFeature, "(max-horizontal-viewport-segments > 10px)");
		assert_parse_error!(HorizontalViewportSegmentsMediaFeature, "(min-horizontal-viewport-segments > 10px)");
		assert_parse_error!(HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments: 1px)");
		assert_parse_error!(HorizontalViewportSegmentsMediaFeature, "(pointer: 1)");
	}
}
