#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BreakBeforeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BreakAfterStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BreakInsideStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OrphansStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<WidowsStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<BoxDecorationBreakStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<MarginBreakStyleValue>(), 16);
	}

	#[test]
	fn test_break_after() {
		assert_parse!(CssAtomSet::ATOMS, BreakAfterStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, BreakAfterStyleValue, "avoid");
		assert_parse!(CssAtomSet::ATOMS, BreakAfterStyleValue, "avoid-page");
		assert_parse!(CssAtomSet::ATOMS, BreakAfterStyleValue, "page");
		assert_parse!(CssAtomSet::ATOMS, BreakAfterStyleValue, "left");
		assert_parse!(CssAtomSet::ATOMS, BreakAfterStyleValue, "right");
		assert_parse!(CssAtomSet::ATOMS, BreakAfterStyleValue, "column");
		assert_parse!(CssAtomSet::ATOMS, BreakAfterStyleValue, "avoid-column");
	}

	#[test]
	fn test_break_after_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BreakAfterStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, BreakAfterStyleValue, "avoid region");
	}

	#[test]
	fn test_break_before() {
		assert_parse!(CssAtomSet::ATOMS, BreakBeforeStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, BreakBeforeStyleValue, "avoid");
		assert_parse!(CssAtomSet::ATOMS, BreakBeforeStyleValue, "page");
		assert_parse!(CssAtomSet::ATOMS, BreakBeforeStyleValue, "column");
	}

	#[test]
	fn test_break_before_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BreakBeforeStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, BreakBeforeStyleValue, "avoid region");
	}

	#[test]
	fn test_break_inside() {
		assert_parse!(CssAtomSet::ATOMS, BreakInsideStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, BreakInsideStyleValue, "avoid");
		assert_parse!(CssAtomSet::ATOMS, BreakInsideStyleValue, "avoid-page");
		assert_parse!(CssAtomSet::ATOMS, BreakInsideStyleValue, "avoid-column");
	}

	#[test]
	fn test_break_inside_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BreakInsideStyleValue, "region");
		assert_parse_error!(CssAtomSet::ATOMS, BreakInsideStyleValue, "auto avoid");
	}

	#[test]
	fn test_box_decoration_break() {
		assert_parse!(CssAtomSet::ATOMS, BoxDecorationBreakStyleValue, "slice");
		assert_parse!(CssAtomSet::ATOMS, BoxDecorationBreakStyleValue, "clone");
	}

	#[test]
	fn test_box_decoration_break_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BoxDecorationBreakStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, BoxDecorationBreakStyleValue, "slice clone");
	}

	#[test]
	fn test_orphans() {
		assert_parse!(CssAtomSet::ATOMS, OrphansStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, OrphansStyleValue, "234");
	}

	#[test]
	fn test_orphans_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, OrphansStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, OrphansStyleValue, "1 234");
		assert_parse_error!(CssAtomSet::ATOMS, OrphansStyleValue, "-234");
		assert_parse_error!(CssAtomSet::ATOMS, OrphansStyleValue, "0");
	}

	#[test]
	fn test_widows() {
		assert_parse!(CssAtomSet::ATOMS, WidowsStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, WidowsStyleValue, "234");
	}

	#[test]
	fn test_widows_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WidowsStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, WidowsStyleValue, "-1");
		assert_parse_error!(CssAtomSet::ATOMS, WidowsStyleValue, "0");
	}
}
