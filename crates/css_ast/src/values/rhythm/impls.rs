#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BlockStepAlignStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BlockStepInsertStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BlockStepRoundStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BlockStepSizeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BlockStepStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<LineHeightStepStyleValue>(), 16);
	}
}
