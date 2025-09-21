#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		// assert_eq!(std::mem::size_of::<ContentStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<QuotesStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<StringSetStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BookmarkLevelStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<BookmarkLabelStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BookmarkStateStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BookmarkLevelStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, BookmarkStateStyleValue, "open");
	}
}
