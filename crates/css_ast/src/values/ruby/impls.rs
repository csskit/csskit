#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<RubyAlignStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<RubyMergeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<RubyOverhangStyleValue>(), 16);
	}
}
