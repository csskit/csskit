#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PointerTimelineAxisStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<PointerTimelineNameStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<PointerTimelineStyleValue>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "block");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "inline");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "x");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "y");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "block, inline");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineNameStyleValue, "--my-timeline");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineNameStyleValue, "none, --my-timeline");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineStyleValue, "--my-timeline");
		assert_parse!(CssAtomSet::ATOMS, PointerTimelineStyleValue, "--my-timeline block");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, PointerTimelineAxisStyleValue, "block inline");
		assert_parse_error!(CssAtomSet::ATOMS, PointerTimelineNameStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, PointerTimelineStyleValue, "auto");
	}
}
