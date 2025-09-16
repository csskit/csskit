pub(crate) use crate::{CssDiagnostic, traits::StyleValue};
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FieldSizingStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<SliderOrientationStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<InputSecurityStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FieldSizingStyleValue, "content");
		assert_parse!(SliderOrientationStyleValue, "bottom-to-top");
		assert_parse!(InputSecurityStyleValue, "none");
	}
}
