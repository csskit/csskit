pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OutlineStyleValue>(), 176);
		assert_eq!(std::mem::size_of::<OutlineWidthStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OutlineStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<OutlineColorStyleValue>(), 144);
		assert_eq!(std::mem::size_of::<OutlineOffsetStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ResizeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<CursorStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<CaretColorStyleValue>(), 144);
		assert_eq!(std::mem::size_of::<CaretAnimationStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<CaretShapeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<CaretStyleValue>(), 176);
		// assert_eq!(std::mem::size_of::<NavUpStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<NavRightStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<NavDownStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<NavLeftStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<UserSelectStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PointerEventsStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<InteractivityStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<InterestDelayStartStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<InterestDelayEndStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<InterestDelayStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AccentColorStyleValue>(), 144);
		assert_eq!(std::mem::size_of::<AppearanceStyleValue>(), 20);
	}

	#[test]
	fn test_parse() {
		assert_parse!(CursorStyleValue, "pointer");
	}

	#[test]
	fn test_parse_error() {
		assert_parse!(CursorStyleValue, "none");
	}
}
