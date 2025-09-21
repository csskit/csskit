#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;

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
}
