pub(crate) use crate::{CssDiagnostic, traits::StyleValue};
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FlexDirectionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FlexWrapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FlexFlowStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<FlexStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<FlexGrowStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<FlexShrinkStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<FlexBasisStyleValue>(), 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FlexBasisStyleValue, "auto");
		assert_parse!(FlexBasisStyleValue, "4px");
	}
}
