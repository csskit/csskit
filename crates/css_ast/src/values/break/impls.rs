#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BreakBeforeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OrphansStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<WidowsStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<BoxDecorationBreakStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MarginBreakStyleValue>(), 16);
	}
}
