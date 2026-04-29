#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ShapeImageThresholdStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ShapeMarginStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ShapePaddingStyleValue>(), 16);
	}
}
