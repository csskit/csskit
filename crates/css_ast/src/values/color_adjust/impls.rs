#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorAdjustStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ForcedColorAdjustStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PrintColorAdjustStyleValue>(), 16);
	}
}
