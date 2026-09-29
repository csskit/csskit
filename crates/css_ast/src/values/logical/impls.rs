#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_margin_block_start_writes() {
		assert_parse!(CssAtomSet::ATOMS, MarginBlockStartStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, MarginBlockStartStyleValue, "-10px");
	}

	#[test]
	fn test_margin_block_start_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MarginBlockStartStyleValue, "none");
		assert_peek_false!(CssAtomSet::ATOMS, MarginBlockStartStyleValue, "10");
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
		assert_peek_false!(CssAtomSet::ATOMS, MarginBlockStyleValue, "none");
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
		assert_peek_false!(CssAtomSet::ATOMS, PaddingBlockStartStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, PaddingBlockStartStyleValue, "-10px");
		assert_peek_false!(CssAtomSet::ATOMS, PaddingBlockStartStyleValue, "auto");
		assert_peek_false!(CssAtomSet::ATOMS, PaddingBlockStartStyleValue, "10");
	}

	#[test]
	fn test_padding_block_end_writes() {
		assert_parse!(CssAtomSet::ATOMS, PaddingBlockEndStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, PaddingBlockEndStyleValue, "20%");
	}

	#[test]
	fn test_padding_block_end_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, PaddingBlockEndStyleValue, "auto");
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
		assert_peek_false!(CssAtomSet::ATOMS, PaddingBlockStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, PaddingBlockStyleValue, "1px 2px 3px");
		assert_peek_false!(CssAtomSet::ATOMS, PaddingBlockStyleValue, "auto");
	}

	#[test]
	fn test_padding_inline_writes() {
		assert_parse!(CssAtomSet::ATOMS, PaddingInlineStyleValue, "20%");
	}

	#[test]
	fn test_padding_inline_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, PaddingInlineStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, PaddingInlineStyleValue, "10px auto 20px");
	}
}
