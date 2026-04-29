#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FlowFromStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FlowIntoStyleValue>(), 28);
		assert_eq!(std::mem::size_of::<RegionFragmentStyleValue>(), 16);
	}
}
