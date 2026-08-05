#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FootnoteDisplayStyleValue, "block");
		assert_parse!(CssAtomSet::ATOMS, FootnoteDisplayStyleValue, "inline");
		assert_parse!(CssAtomSet::ATOMS, FootnoteDisplayStyleValue, "compact");
		assert_parse!(CssAtomSet::ATOMS, FootnotePolicyStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, FootnotePolicyStyleValue, "line");
		assert_parse!(CssAtomSet::ATOMS, FootnotePolicyStyleValue, "block");
		assert_parse!(CssAtomSet::ATOMS, RunningStyleValue, "myelement");
		assert_parse!(CssAtomSet::ATOMS, RunningStyleValue, "header");
	}

	#[test]
	fn test_string_set() {
		assert_parse!(CssAtomSet::ATOMS, StringSetStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, StringSetStyleValue, "header contents");
		assert_parse!(CssAtomSet::ATOMS, StringSetStyleValue, "header string(chapter)");
		assert_parse!(CssAtomSet::ATOMS, StringSetStyleValue, "header contents, footer string(chapter)");
		assert_peek_false!(CssAtomSet::ATOMS, StringSetStyleValue, "");
	}

	#[test]
	fn test_copy_into() {
		assert_parse!(CssAtomSet::ATOMS, CopyIntoStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, CopyIntoStyleValue, "header element");
		assert_parse!(CssAtomSet::ATOMS, CopyIntoStyleValue, "header content");
		assert_parse!(CssAtomSet::ATOMS, CopyIntoStyleValue, "header text");
		assert_parse!(CssAtomSet::ATOMS, CopyIntoStyleValue, "header attr(title)");
		assert_parse!(CssAtomSet::ATOMS, CopyIntoStyleValue, "header counter(page)");
		assert_parse!(CssAtomSet::ATOMS, CopyIntoStyleValue, "header counters(page,'.')");
		assert_parse!(CssAtomSet::ATOMS, CopyIntoStyleValue, "header element, footer text");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, FootnoteDisplayStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, FootnoteDisplayStyleValue, "block inline");
		assert_peek_false!(CssAtomSet::ATOMS, FootnotePolicyStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, FootnotePolicyStyleValue, "auto line");
	}
}
