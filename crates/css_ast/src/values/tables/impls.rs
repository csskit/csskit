#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BorderCollapseStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderSpacingStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<CaptionSideStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<EmptyCellsStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TableLayoutStyleValue>(), 16);
	}
}
