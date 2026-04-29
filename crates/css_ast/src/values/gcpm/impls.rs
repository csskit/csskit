#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FootnoteDisplayStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FootnotePolicyStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<RunningStyleValue>(), 12);
	}
}
