#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BlockStepAlignStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BlockStepInsertStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BlockStepRoundStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BlockStepSizeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BlockStepStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<LineHeightStepStyleValue>(), 16);
	}

	#[test]
	fn test_block_step_size_writes() {
		assert_parse!(CssAtomSet::ATOMS, BlockStepSizeStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, BlockStepSizeStyleValue, "2em");
		assert_parse!(CssAtomSet::ATOMS, BlockStepSizeStyleValue, "none");
	}

	#[test]
	fn test_block_step_size_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepSizeStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepSizeStyleValue, "-1px");
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepSizeStyleValue, "min-content");
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepSizeStyleValue, "10%");
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepSizeStyleValue, "20");
	}

	#[test]
	fn test_block_step_align_writes() {
		assert_parse!(CssAtomSet::ATOMS, BlockStepAlignStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, BlockStepAlignStyleValue, "center");
		assert_parse!(CssAtomSet::ATOMS, BlockStepAlignStyleValue, "start");
		assert_parse!(CssAtomSet::ATOMS, BlockStepAlignStyleValue, "end");
	}

	#[test]
	fn test_block_step_align_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepAlignStyleValue, "auto auto");
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepAlignStyleValue, "start end");
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepAlignStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepAlignStyleValue, "-1px");
	}

	#[test]
	fn test_block_step_round_writes() {
		assert_parse!(CssAtomSet::ATOMS, BlockStepRoundStyleValue, "up");
		assert_parse!(CssAtomSet::ATOMS, BlockStepRoundStyleValue, "down");
		assert_parse!(CssAtomSet::ATOMS, BlockStepRoundStyleValue, "nearest");
	}

	#[test]
	fn test_block_step_round_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepRoundStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepRoundStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepRoundStyleValue, "up down");
	}

	#[test]
	fn test_block_step_insert_writes() {
		assert_parse!(CssAtomSet::ATOMS, BlockStepInsertStyleValue, "margin-box");
		assert_parse!(CssAtomSet::ATOMS, BlockStepInsertStyleValue, "padding-box");
		assert_parse!(CssAtomSet::ATOMS, BlockStepInsertStyleValue, "content-box");
	}

	#[test]
	fn test_block_step_insert_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepInsertStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepInsertStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepInsertStyleValue, "border-box");
		assert_parse_error!(CssAtomSet::ATOMS, BlockStepInsertStyleValue, "margin-box padding-box");
	}

	#[test]
	fn test_line_height_step_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineHeightStepStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, LineHeightStepStyleValue, "2em");
	}

	#[test]
	fn test_line_height_step_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, LineHeightStepStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, LineHeightStepStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, LineHeightStepStyleValue, "-1px");
		assert_parse_error!(CssAtomSet::ATOMS, LineHeightStepStyleValue, "10%");
	}
}
