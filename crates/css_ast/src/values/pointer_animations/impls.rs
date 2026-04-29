#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PointerTimelineAxisStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<PointerTimelineNameStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<PointerTimelineStyleValue>(), 32);
	}
}
