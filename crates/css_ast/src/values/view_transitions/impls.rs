pub(crate) use crate::{CssDiagnostic, traits::StyleValue};
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<ViewTransitionNameStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ViewTransitionClassStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ViewTransitionGroupStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ViewTransitionNameStyleValue, "none");
		assert_parse!(ViewTransitionNameStyleValue, "foo");
		assert_parse!(ViewTransitionClassStyleValue, "none");
		assert_parse!(ViewTransitionClassStyleValue, "foo");
		assert_parse!(ViewTransitionClassStyleValue, "foo bar baz");
		assert_parse!(ViewTransitionGroupStyleValue, "normal");
		assert_parse!(ViewTransitionGroupStyleValue, "nearest");
		assert_parse!(ViewTransitionGroupStyleValue, "foo");
	}
}
