#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WrapFlowStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<WrapThroughStyleValue>(), 16);
	}
}
