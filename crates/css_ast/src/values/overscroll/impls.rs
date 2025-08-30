pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<OverscrollBehaviorStyleValue>(), 28);
		assert_eq!(std::mem::size_of::<OverscrollBehaviorXStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverscrollBehaviorYStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverscrollBehaviorInlineStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OverscrollBehaviorBlockStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OverscrollBehaviorStyleValue, "contain");
		assert_parse!(OverscrollBehaviorStyleValue, "contain none");
		assert_parse!(OverscrollBehaviorInlineStyleValue, "contain");
	}
}
