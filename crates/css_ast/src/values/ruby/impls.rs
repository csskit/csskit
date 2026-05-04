#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<RubyAlignStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<RubyMergeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<RubyOverhangStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<RubyPositionStyleValue>(), 36);
	}

	#[test]
	fn test_ruby_align_writes() {
		assert_parse!(CssAtomSet::ATOMS, RubyAlignStyleValue, "start");
		assert_parse!(CssAtomSet::ATOMS, RubyAlignStyleValue, "center");
		assert_parse!(CssAtomSet::ATOMS, RubyAlignStyleValue, "space-between");
		assert_parse!(CssAtomSet::ATOMS, RubyAlignStyleValue, "space-around");
	}

	#[test]
	fn test_ruby_align_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, RubyAlignStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, RubyAlignStyleValue, "left");
		assert_parse_error!(CssAtomSet::ATOMS, RubyAlignStyleValue, "10px");
		assert_parse_error!(CssAtomSet::ATOMS, RubyAlignStyleValue, "center start");
	}

	#[test]
	fn test_ruby_merge_writes() {
		assert_parse!(CssAtomSet::ATOMS, RubyMergeStyleValue, "separate");
		assert_parse!(CssAtomSet::ATOMS, RubyMergeStyleValue, "merge");
		assert_parse!(CssAtomSet::ATOMS, RubyMergeStyleValue, "auto");
	}

	#[test]
	fn test_ruby_merge_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, RubyMergeStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, RubyMergeStyleValue, "collapse");
		assert_parse_error!(CssAtomSet::ATOMS, RubyMergeStyleValue, "10px");
		assert_parse_error!(CssAtomSet::ATOMS, RubyMergeStyleValue, "merge separate");
	}

	#[test]
	fn test_ruby_overhang_writes() {
		assert_parse!(CssAtomSet::ATOMS, RubyOverhangStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, RubyOverhangStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, RubyOverhangStyleValue, "spaces");
	}

	#[test]
	fn test_ruby_overhang_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, RubyOverhangStyleValue, "auto none");
		assert_parse_error!(CssAtomSet::ATOMS, RubyOverhangStyleValue, "simple");
		assert_parse_error!(CssAtomSet::ATOMS, RubyOverhangStyleValue, "auto spaces");
	}

	#[test]
	fn test_ruby_position() {
		assert_parse!(CssAtomSet::ATOMS, RubyPositionStyleValue, "over");
		assert_parse!(CssAtomSet::ATOMS, RubyPositionStyleValue, "under");
		assert_parse!(CssAtomSet::ATOMS, RubyPositionStyleValue, "alternate");
		assert_parse!(CssAtomSet::ATOMS, RubyPositionStyleValue, "inter-character");
		assert_parse!(CssAtomSet::ATOMS, RubyPositionStyleValue, "alternate over");
		assert_parse!(CssAtomSet::ATOMS, RubyPositionStyleValue, "alternate under");
		assert_parse_error!(CssAtomSet::ATOMS, RubyPositionStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, RubyPositionStyleValue, "left");
		assert_parse_error!(CssAtomSet::ATOMS, RubyPositionStyleValue, "over under");
	}
}
