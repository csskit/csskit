pub(crate) use crate::{CssDiagnostic, traits::StyleValue};
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		// assert_eq!(std::mem::size_of::<ContainerTypeStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<ContainerNameStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<ContainerStyleValue>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ContainerNameStyleValue, "none");
		assert_parse!(ContainerNameStyleValue, "a");
		assert_parse!(ContainerNameStyleValue, "a b c");
	}
}
