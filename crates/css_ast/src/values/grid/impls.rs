#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		// assert_eq!(std::mem::size_of::<GridTemplateColumnsStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridTemplateRowsStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<GridTemplateAreasStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<GridTemplateStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<GridAutoColumnsStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<GridAutoRowsStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<GridAutoFlowStyleValue>(), 36);
		// assert_eq!(std::mem::size_of::<GridStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<GridRowStartStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<GridColumnStartStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<GridRowEndStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<GridColumnEndStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<GridRowStyleValue>(), 100);
		assert_eq!(std::mem::size_of::<GridColumnStyleValue>(), 100);
		assert_eq!(std::mem::size_of::<GridAreaStyleValue>(), 212);
		assert_eq!(std::mem::size_of::<FlowToleranceStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, GridTemplateAreasStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, GridTemplateAreasStyleValue, r#""foo""bar""#);

		assert_parse!(CssAtomSet::ATOMS, FlowToleranceStyleValue, "infinite");
		assert_parse!(CssAtomSet::ATOMS, FlowToleranceStyleValue, "30px");
	}

	#[test]
	fn test_grid_auto_flow() {
		assert_parse!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "row");
		assert_parse!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "column");
		assert_parse!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "dense");
		assert_parse!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "row dense");
		assert_parse!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "column dense");
		assert_parse_error!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, GridAutoFlowStyleValue, "row column");
	}
}
