#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColumnCountStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnFillStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnHeightStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnSpanStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnWidthStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnWrapStyleValue>(), 16);
	}
}
