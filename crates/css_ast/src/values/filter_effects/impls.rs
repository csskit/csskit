#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BackdropFilterStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ColorInterpolationFiltersStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FilterStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<FloodColorStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<FloodOpacityStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<LightingColorStyleValue>(), 24);
	}
}
