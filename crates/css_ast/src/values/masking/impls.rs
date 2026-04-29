#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ClipRuleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MaskBorderModeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MaskBorderOutsetStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<MaskBorderRepeatStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MaskBorderSourceStyleValue>(), 128);
		assert_eq!(std::mem::size_of::<MaskClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MaskOriginStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MaskPositionStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MaskRepeatStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MaskSizeStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<MaskTypeStyleValue>(), 16);
	}
}
