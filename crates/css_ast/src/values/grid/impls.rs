pub(crate) use crate::{CssDiagnostic, traits::StyleValue};
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
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
		assert_eq!(std::mem::size_of::<ItemSlackStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ItemDirectionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ItemTrackStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<ItemWrapStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<ItemCrossStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<ItemPackStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<ItemFlowStyleValue>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(GridTemplateAreasStyleValue, "none");
		assert_parse!(GridTemplateAreasStyleValue, r#""foo""bar""#);

		assert_parse!(ItemSlackStyleValue, "infinite");
		assert_parse!(ItemSlackStyleValue, "30px");

		assert_parse!(ItemDirectionStyleValue, "auto");
		assert_parse!(ItemDirectionStyleValue, "row-reverse");

		assert_parse!(ItemTrackStyleValue, "auto");
		assert_parse!(ItemTrackStyleValue, "row-reverse");
	}
}
