#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<BookmarkLevelStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BookmarkLabelStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BookmarkStateStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ContentStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<QuotesStyleValue>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BookmarkLevelStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, BookmarkStateStyleValue, "open");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "\"\"");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "\"hello\"");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "\"\\f105\"");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "url(dot.gif)");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "open-quote");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "counter(section,decimal)");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "\"Chapter\" counter(chapter)");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "\"hello\" / \"alt text\"");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "url(img.png) / \"alt\"");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ContentStyleValue, "invalid-keyword");
	}
}
