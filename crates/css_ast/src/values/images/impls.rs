#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ImageRenderingStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ObjectPositionStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<ObjectViewBoxStyleValue>(), 16);
	}
}
