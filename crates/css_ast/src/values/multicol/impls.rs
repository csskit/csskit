#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColumnCountStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnFillStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnHeightStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnSpanStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnWidthStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnWrapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ColumnsStyleValue>(), 60);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColumnCountStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ColumnCountStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, ColumnCountStyleValue, "234");

		assert_parse!(CssAtomSet::ATOMS, ColumnFillStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ColumnFillStyleValue, "balance");
		assert_parse!(CssAtomSet::ATOMS, ColumnFillStyleValue, "balance-all");

		assert_parse!(CssAtomSet::ATOMS, ColumnSpanStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ColumnSpanStyleValue, "all");

		assert_parse!(CssAtomSet::ATOMS, ColumnWidthStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ColumnWidthStyleValue, "10px");
	}

	#[test]
	fn test_columns() {
		assert_parse!(CssAtomSet::ATOMS, ColumnsStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ColumnsStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, ColumnsStyleValue, "3");
		assert_parse!(CssAtomSet::ATOMS, ColumnsStyleValue, "10px 3");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnsStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnsStyleValue, "none");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ColumnCountStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnCountStyleValue, "2.5");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnCountStyleValue, "-1");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnCountStyleValue, "0");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnCountStyleValue, "1 234");

		assert_parse_error!(CssAtomSet::ATOMS, ColumnFillStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnFillStyleValue, "auto balance");

		assert_parse_error!(CssAtomSet::ATOMS, ColumnSpanStyleValue, "none all");

		assert_parse_error!(CssAtomSet::ATOMS, ColumnWidthStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnWidthStyleValue, "10");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnWidthStyleValue, "-20px");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnWidthStyleValue, "30%");
	}
}
