#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BlockSizeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<InlineSizeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<MinBlockSizeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<MinInlineSizeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<MaxBlockSizeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<MaxInlineSizeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<MarginBlockStartStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MarginBlockEndStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MarginInlineStartStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MarginInlineEndStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MarginBlockStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MarginInlineStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<PaddingBlockStartStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PaddingBlockEndStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PaddingInlineStartStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PaddingInlineEndStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PaddingBlockStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<PaddingInlineStyleValue>(), 32);
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
		assert_parse_error!(CssAtomSet::ATOMS, BlockSizeStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, BlockSizeStyleValue, "-10px");
		assert_parse_error!(CssAtomSet::ATOMS, BlockSizeStyleValue, "-20%");
		assert_parse_error!(CssAtomSet::ATOMS, BlockSizeStyleValue, "60");
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
		assert_parse_error!(CssAtomSet::ATOMS, InlineSizeStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, InlineSizeStyleValue, "-10px");
		assert_parse_error!(CssAtomSet::ATOMS, InlineSizeStyleValue, "60");
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
	fn test_margin_block_start_writes() {
		assert_parse!(CssAtomSet::ATOMS, MarginBlockStartStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MarginBlockStartStyleValue, "-10px");
	}

	#[test]
	fn test_margin_block_start_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MarginBlockStartStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, MarginBlockStartStyleValue, "10");
	}

	#[test]
	fn test_margin_block_end_writes() {
		assert_parse!(CssAtomSet::ATOMS, MarginBlockEndStyleValue, "-10px");
		assert_parse!(CssAtomSet::ATOMS, MarginBlockEndStyleValue, "auto");
	}

	#[test]
	fn test_margin_inline_start_writes() {
		assert_parse!(CssAtomSet::ATOMS, MarginInlineStartStyleValue, "-20%");
	}

	#[test]
	fn test_margin_inline_end_writes() {
		assert_parse!(CssAtomSet::ATOMS, MarginInlineEndStyleValue, "auto");
	}

	#[test]
	fn test_margin_block_writes() {
		assert_parse!(CssAtomSet::ATOMS, MarginBlockStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MarginBlockStyleValue, "-10px");
	}

	#[test]
	fn test_margin_block_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MarginBlockStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, MarginBlockStyleValue, "10px auto 20px");
	}

	#[test]
	fn test_margin_inline_writes() {
		assert_parse!(CssAtomSet::ATOMS, MarginInlineStyleValue, "20%");
		assert_parse!(CssAtomSet::ATOMS, MarginInlineStyleValue, "-10px auto");
	}

	#[test]
	fn test_padding_block_start_writes() {
		assert_parse!(CssAtomSet::ATOMS, PaddingBlockStartStyleValue, "10px");
	}

	#[test]
	fn test_padding_block_start_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PaddingBlockStartStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, PaddingBlockStartStyleValue, "-10px");
		assert_parse_error!(CssAtomSet::ATOMS, PaddingBlockStartStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, PaddingBlockStartStyleValue, "10");
	}

	#[test]
	fn test_padding_block_end_writes() {
		assert_parse!(CssAtomSet::ATOMS, PaddingBlockEndStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, PaddingBlockEndStyleValue, "20%");
	}

	#[test]
	fn test_padding_block_end_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PaddingBlockEndStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, PaddingBlockEndStyleValue, "-10px");
		assert_parse_error!(CssAtomSet::ATOMS, PaddingBlockEndStyleValue, "1px, 2px");
	}

	#[test]
	fn test_padding_inline_start_writes() {
		assert_parse!(CssAtomSet::ATOMS, PaddingInlineStartStyleValue, "20%");
		assert_parse!(CssAtomSet::ATOMS, PaddingInlineStartStyleValue, "10px");
	}

	#[test]
	fn test_padding_inline_end_writes() {
		assert_parse!(CssAtomSet::ATOMS, PaddingInlineEndStyleValue, "10px");
	}

	#[test]
	fn test_padding_block_writes() {
		assert_parse!(CssAtomSet::ATOMS, PaddingBlockStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, PaddingBlockStyleValue, "10px 20%");
	}

	#[test]
	fn test_padding_block_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PaddingBlockStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, PaddingBlockStyleValue, "1px 2px 3px");
		assert_parse_error!(CssAtomSet::ATOMS, PaddingBlockStyleValue, "auto");
	}

	#[test]
	fn test_padding_inline_writes() {
		assert_parse!(CssAtomSet::ATOMS, PaddingInlineStyleValue, "20%");
	}

	#[test]
	fn test_padding_inline_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PaddingInlineStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, PaddingInlineStyleValue, "10px auto 20px");
	}
}
