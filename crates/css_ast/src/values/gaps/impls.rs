#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColumnRuleColorStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<ColumnRuleStyleStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<ColumnRuleWidthStyleValue>(), 152);
		assert_eq!(std::mem::size_of::<ColumnRuleStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<RowRuleColorStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<RowRuleStyleStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<RowRuleWidthStyleValue>(), 152);
		assert_eq!(std::mem::size_of::<RowRuleStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<RuleColorStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<RuleStyleStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<RuleWidthStyleValue>(), 152);
		assert_eq!(std::mem::size_of::<RuleStyleValue>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "1px solid red");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "1px solid red, repeat(2, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "repeat(auto, 1px solid red)");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "1px solid red, repeat(auto, 2px dashed green)");
		assert_parse!(
			CssAtomSet::ATOMS,
			ColumnRuleStyleValue,
			"repeat(auto, 1px solid red), repeat(auto, 2px dashed green)"
		);
		assert_parse!(CssAtomSet::ATOMS, RowRuleStyleValue, "repeat(auto, 1px solid red), 2px dashed green");
		assert_parse!(
			CssAtomSet::ATOMS,
			RowRuleStyleValue,
			"repeat(auto, 1px solid red), repeat(auto, 2px dashed green)"
		);
		assert_parse!(CssAtomSet::ATOMS, RuleStyleValue, "1px solid red, repeat(2, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, RuleStyleValue, "repeat(auto, 1px solid red), 2px dashed green");
		assert_parse!(CssAtomSet::ATOMS, RuleStyleValue, "1px solid red, repeat(auto, 2px dashed green)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "repeat(auto,)");
		assert_parse_error!(CssAtomSet::ATOMS, RuleStyleValue, "repeat(auto,)");
		assert_parse_error!(CssAtomSet::ATOMS, RuleStyleValue, "1px solid red,");
	}
}
