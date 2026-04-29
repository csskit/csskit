#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OffsetAnchorStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<OffsetDistanceStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OffsetPositionStyleValue>(), 64);
	}
}
