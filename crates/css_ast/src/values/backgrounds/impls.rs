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
		assert_parse!(CssAtomSet::ATOMS, BackgroundPositionXStyleValue, "center");
		assert_parse!(CssAtomSet::ATOMS, BackgroundPositionXStyleValue, "left");
		assert_parse!(CssAtomSet::ATOMS, BackgroundPositionXStyleValue, "x-end 10px");
		assert_parse!(CssAtomSet::ATOMS, BackgroundPositionXStyleValue, "50%");
		assert_parse!(CssAtomSet::ATOMS, BackgroundPositionXStyleValue, "left 10px, center, 25%");
		assert_parse!(CssAtomSet::ATOMS, BackgroundPositionYStyleValue, "bottom");
		assert_parse!(CssAtomSet::ATOMS, BackgroundPositionYStyleValue, "y-start 2em");
		assert_parse!(CssAtomSet::ATOMS, BackgroundPositionBlockStyleValue, "start 10px");
		assert_parse!(CssAtomSet::ATOMS, BackgroundPositionInlineStyleValue, "end, center");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, BackgroundStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, BackgroundPositionXStyleValue, "top");
		assert_peek_false!(CssAtomSet::ATOMS, BackgroundPositionYStyleValue, "left");
	}
}
