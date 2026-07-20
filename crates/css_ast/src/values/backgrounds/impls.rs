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
	fn size_test() {
		assert_eq!(std::mem::size_of::<BackgroundColorStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BackgroundImageStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BackgroundRepeatStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BackgroundAttachmentStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BackgroundPositionStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BackgroundClipStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BackgroundOriginStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BackgroundSizeStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BackgroundStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BackgroundRepeatXStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BackgroundRepeatYStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BackgroundRepeatBlockStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BackgroundRepeatInlineStyleValue>(), 24);
	}

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
