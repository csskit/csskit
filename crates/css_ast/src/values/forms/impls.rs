pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FieldSizingStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<SliderOrientationStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<InputSecurityStyleValue>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FieldSizingStyleValue, "content");
		assert_parse!(SliderOrientationStyleValue, "bottom-to-top");
		assert_parse!(InputSecurityStyleValue, "none");
	}
}
