// use super::types::LengthPercentage;
// use super::{MinWidthStyleValue, Width};

// shortcuts for logical properties to resolve to 0
// impl Width {
// 	#[allow(non_upper_case_globals)]
// 	pub const Zero: Width = Width::LengthPercentage(LengthPercentage::Zero();
// }
//
// impl MinWidth {
// 	#[allow(non_upper_case_globals)]
// 	pub const Zero: MinWidth = MinWidth::LengthPercentage(LengthPercentage::Zero);
// }

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_contain_intrinsic() {
		assert_parse!(CssAtomSet::ATOMS, ContainIntrinsicWidthStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ContainIntrinsicWidthStyleValue, "100px");
		assert_parse!(CssAtomSet::ATOMS, ContainIntrinsicWidthStyleValue, "auto none");
		assert_parse!(CssAtomSet::ATOMS, ContainIntrinsicWidthStyleValue, "auto 100px");
		assert_parse!(CssAtomSet::ATOMS, ContainIntrinsicSizeStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ContainIntrinsicSizeStyleValue, "100px");
		assert_parse!(CssAtomSet::ATOMS, ContainIntrinsicSizeStyleValue, "none none");
		assert_parse!(CssAtomSet::ATOMS, ContainIntrinsicSizeStyleValue, "auto none 100px");
		assert_parse_error!(CssAtomSet::ATOMS, ContainIntrinsicWidthStyleValue, "auto");
		assert_peek_false!(CssAtomSet::ATOMS, ContainIntrinsicWidthStyleValue, "");
	}

	#[test]
	fn test_min_intrinsic_sizing() {
		assert_parse!(CssAtomSet::ATOMS, MinIntrinsicSizingStyleValue, "legacy");
		assert_parse!(CssAtomSet::ATOMS, MinIntrinsicSizingStyleValue, "zero-if-scroll");
		assert_parse!(CssAtomSet::ATOMS, MinIntrinsicSizingStyleValue, "zero-if-extrinsic");
		assert_parse!(CssAtomSet::ATOMS, MinIntrinsicSizingStyleValue, "zero-if-scroll zero-if-extrinsic");
		assert_peek_false!(CssAtomSet::ATOMS, MinIntrinsicSizingStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, MinIntrinsicSizingStyleValue, "auto");
	}

	#[test]
	fn test_block_size_writes() {
		assert_parse!(CssAtomSet::ATOMS, BlockSizeStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, BlockSizeStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, BlockSizeStyleValue, "20%");
		assert_parse!(CssAtomSet::ATOMS, BlockSizeStyleValue, "min-content");
		assert_parse!(CssAtomSet::ATOMS, BlockSizeStyleValue, "max-content");
	}

	#[test]
	fn test_block_size_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, BlockSizeStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, BlockSizeStyleValue, "-10px");
		assert_parse_error!(CssAtomSet::ATOMS, BlockSizeStyleValue, "-20%");
		assert_peek_false!(CssAtomSet::ATOMS, BlockSizeStyleValue, "60");
		assert_parse_error!(CssAtomSet::ATOMS, BlockSizeStyleValue, "10px 20%");
	}

	#[test]
	fn test_inline_size_writes() {
		assert_parse!(CssAtomSet::ATOMS, InlineSizeStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, InlineSizeStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, InlineSizeStyleValue, "20%");
		assert_parse!(CssAtomSet::ATOMS, InlineSizeStyleValue, "min-content");
		assert_parse!(CssAtomSet::ATOMS, InlineSizeStyleValue, "max-content");
	}

	#[test]
	fn test_inline_size_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, InlineSizeStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, InlineSizeStyleValue, "-10px");
		assert_peek_false!(CssAtomSet::ATOMS, InlineSizeStyleValue, "60");
		assert_parse_error!(CssAtomSet::ATOMS, InlineSizeStyleValue, "10px 20%");
	}

	#[test]
	fn test_min_block_size_writes() {
		assert_parse!(CssAtomSet::ATOMS, MinBlockSizeStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MinBlockSizeStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, MinBlockSizeStyleValue, "20%");
		assert_parse!(CssAtomSet::ATOMS, MinBlockSizeStyleValue, "min-content");
		assert_parse!(CssAtomSet::ATOMS, MinBlockSizeStyleValue, "max-content");
	}

	#[test]
	fn test_max_block_size_writes() {
		assert_parse!(CssAtomSet::ATOMS, MaxBlockSizeStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MaxBlockSizeStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, MaxBlockSizeStyleValue, "20%");
		assert_parse!(CssAtomSet::ATOMS, MaxBlockSizeStyleValue, "min-content");
		assert_parse!(CssAtomSet::ATOMS, MaxBlockSizeStyleValue, "max-content");
	}

	#[test]
	fn test_max_inline_size_writes() {
		assert_parse!(CssAtomSet::ATOMS, MaxInlineSizeStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, MaxInlineSizeStyleValue, "10px");
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "fit-content");
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "fit-content(20rem)");
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "fit-content(0)");
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "stretch");
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "contain");
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "-webkit-min-content");
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "-webkit-max-content");
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "-moz-min-content");
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "-moz-max-content");
		assert_parse!(CssAtomSet::ATOMS, HeightStyleValue, "-webkit-min-content");
		assert_parse!(CssAtomSet::ATOMS, HeightStyleValue, "-webkit-max-content");
		assert_parse!(CssAtomSet::ATOMS, HeightStyleValue, "-moz-min-content");
		assert_parse!(CssAtomSet::ATOMS, HeightStyleValue, "-moz-max-content");
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "calc-size(auto, size/2)");
		assert_parse!(CssAtomSet::ATOMS, WidthStyleValue, "calc-size(any, size + 10px)");
		assert_parse!(CssAtomSet::ATOMS, HeightStyleValue, "calc-size(fit-content, size*2)");
		assert_parse!(CssAtomSet::ATOMS, MaxWidthStyleValue, "calc-size(max-content, size)");

		assert_parse!(CssAtomSet::ATOMS, AspectRatioStyleValue, "auto 1/5");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioStyleValue, "auto 1/2");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioStyleValue, "1/3 auto");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AspectRatioStyleValue, "auto auto");
		assert_parse_error!(CssAtomSet::ATOMS, AspectRatioStyleValue, "1/2 1/2");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("12px", WidthStyleValue, BoxSize, LengthPercentage, Length);
	}
}
