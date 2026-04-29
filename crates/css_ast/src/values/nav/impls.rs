#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SpatialNavigationActionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<SpatialNavigationContainStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<SpatialNavigationFunctionStyleValue>(), 16);
	}
}
