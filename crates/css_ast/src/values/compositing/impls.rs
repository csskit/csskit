#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BackgroundBlendModeStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<IsolationStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MixBlendModeStyleValue>(), 16);
	}
}
