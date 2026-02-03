#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		// assert_eq!(std::mem::size_of::<GridTemplateColumnsStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridTemplateRowsStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<GridTemplateAreasStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<GridTemplateStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridAutoColumnsStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridAutoRowsStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridAutoFlowStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridRowStartStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridColumnStartStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridRowEndStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridColumnEndStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridRowStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridColumnStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<GridAreaStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<FlowToleranceStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ItemDirectionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ItemTrackStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<ItemWrapStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<ItemCrossStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<ItemPackStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<ItemFlowStyleValue>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, GridTemplateAreasStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, GridTemplateAreasStyleValue, r#""foo""bar""#);

		assert_parse!(CssAtomSet::ATOMS, FlowToleranceStyleValue, "infinite");
		assert_parse!(CssAtomSet::ATOMS, FlowToleranceStyleValue, "30px");

		assert_parse!(CssAtomSet::ATOMS, ItemDirectionStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ItemDirectionStyleValue, "row-reverse");

		assert_parse!(CssAtomSet::ATOMS, ItemTrackStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ItemTrackStyleValue, "row-reverse");

		assert_parse!(CssAtomSet::ATOMS, ItemPackStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, ItemPackStyleValue, "dense balance");
		assert_parse!(CssAtomSet::ATOMS, ItemPackStyleValue, "balance dense");
	}
}
