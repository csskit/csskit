#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BoxSnapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<LineGridStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<LineSnapStyleValue>(), 16);
	}
}
