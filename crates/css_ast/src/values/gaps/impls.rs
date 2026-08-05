#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, RowGapStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, ColumnGapStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, GapStyleValue, "normal 1px");
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
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleStyleStyleValue, "solid");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleStyleStyleValue, "solid dashed repeat(2,dotted)");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleStyleStyleValue, "repeat(auto,solid)");
		assert_parse!(CssAtomSet::ATOMS, RowRuleStyleStyleValue, "solid repeat(auto,dashed) dotted");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleColorStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleColorStyleValue, "red repeat(2,green) currentcolor");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleColorStyleValue, "repeat(auto,red)");
		assert_parse!(CssAtomSet::ATOMS, RowRuleColorStyleValue, "red repeat(auto,green) blue");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleWidthStyleValue, "thin 2px");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleWidthStyleValue, "repeat(auto,2px)");
		assert_parse!(CssAtomSet::ATOMS, RowRuleWidthStyleValue, "thin repeat(auto,2px) thick");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "repeat(auto,)");
		assert_parse_error!(CssAtomSet::ATOMS, RuleStyleValue, "repeat(auto,)");
		assert_parse_error!(CssAtomSet::ATOMS, RuleStyleValue, "1px solid red,");
		assert_peek_false!(CssAtomSet::ATOMS, ColumnRuleStyleStyleValue, "florp");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnRuleColorStyleValue, "repeat(auto,)");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnRuleWidthStyleValue, "repeat(2,)");
	}
}
