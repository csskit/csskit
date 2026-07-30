use super::super::prelude::*;
use super::*;
use crate::{BgLayer, CssAtomSet};
use css_parse::{CommaSeparated, Parse};

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BackgroundRepeatStyleValue, "repeat-x");
		assert_parse!(CssAtomSet::ATOMS, BackgroundRepeatStyleValue, "space round");
		assert_parse!(CssAtomSet::ATOMS, BackgroundStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, BackgroundStyleValue, "transparent");
		assert_parse!(CssAtomSet::ATOMS, BackgroundStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, BackgroundStyleValue, "#fff");
		assert_parse!(CssAtomSet::ATOMS, BackgroundStyleValue, "#000");
		assert_parse!(CssAtomSet::ATOMS, BackgroundStyleValue, "0 0");
		assert_parse!(CssAtomSet::ATOMS, BackgroundStyleValue, "url(foo.png) no-repeat");
		assert_parse!(CssAtomSet::ATOMS, BackgroundStyleValue, "url(bg.png) no-repeat fixed");
		assert_parse!(CssAtomSet::ATOMS, BackgroundStyleValue, "red, none");
		assert_parse!(CssAtomSet::ATOMS, BackgroundStyleValue, "center center");
		assert_parse!(CssAtomSet::ATOMS, BackgroundStyleValue, "50%");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, BackgroundStyleValue, "");
	}
}
