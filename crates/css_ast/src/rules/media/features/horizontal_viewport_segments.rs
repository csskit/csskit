use super::prelude::*;
use crate::units::CSSInt;

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum HorizontalViewportSegmentsMediaFeature<CssAtomSet::HorizontalViewportSegments | CssAtomSet::MinHorizontalViewportSegments | CssAtomSet::MaxHorizontalViewportSegments, CSSInt>
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<HorizontalViewportSegmentsMediaFeature>(), 116);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments:2)");
		assert_parse!(CssAtomSet::ATOMS, HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments:8)");
		assert_parse!(
			CssAtomSet::ATOMS,
			HorizontalViewportSegmentsMediaFeature,
			"(min-horizontal-viewport-segments:2)"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			HorizontalViewportSegmentsMediaFeature,
			"(max-horizontal-viewport-segments:2)"
		);
		assert_parse!(CssAtomSet::ATOMS, HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments<=3)");
		assert_parse!(CssAtomSet::ATOMS, HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments>=5)");
		assert_parse!(CssAtomSet::ATOMS, HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments>=8)");
		assert_parse!(CssAtomSet::ATOMS, HorizontalViewportSegmentsMediaFeature, "(horizontal-viewport-segments=16)");
		assert_parse!(CssAtomSet::ATOMS, HorizontalViewportSegmentsMediaFeature, "(6=horizontal-viewport-segments)");
		assert_parse!(CssAtomSet::ATOMS, HorizontalViewportSegmentsMediaFeature, "(2<=horizontal-viewport-segments)");
		assert_parse!(CssAtomSet::ATOMS, HorizontalViewportSegmentsMediaFeature, "(2<horizontal-viewport-segments<4)");
		assert_parse!(CssAtomSet::ATOMS, HorizontalViewportSegmentsMediaFeature, "(4>horizontal-viewport-segments<8)");
		assert_parse!(
			CssAtomSet::ATOMS,
			HorizontalViewportSegmentsMediaFeature,
			"(4>=horizontal-viewport-segments<=8)"
		);
		assert_parse!(CssAtomSet::ATOMS, HorizontalViewportSegmentsMediaFeature, "(4<=horizontal-viewport-segments>8)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(
			CssAtomSet::ATOMS,
			HorizontalViewportSegmentsMediaFeature,
			"(horizontal-viewport-segments:)"
		);
		assert_parse_error!(
			CssAtomSet::ATOMS,
			HorizontalViewportSegmentsMediaFeature,
			"(horizontal-viewport-segments: > 10px)"
		);
		assert_parse_error!(
			CssAtomSet::ATOMS,
			HorizontalViewportSegmentsMediaFeature,
			"(max-horizontal-viewport-segments > 10px)"
		);
		assert_parse_error!(
			CssAtomSet::ATOMS,
			HorizontalViewportSegmentsMediaFeature,
			"(min-horizontal-viewport-segments > 10px)"
		);
		assert_parse_error!(
			CssAtomSet::ATOMS,
			HorizontalViewportSegmentsMediaFeature,
			"(horizontal-viewport-segments: 1px)"
		);
		assert_parse_error!(CssAtomSet::ATOMS, HorizontalViewportSegmentsMediaFeature, "(pointer: 1)");
	}
}
