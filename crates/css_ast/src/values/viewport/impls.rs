pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<ZoomStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ZoomStyleValue, "10");
		assert_parse!(ZoomStyleValue, "10.2");
		assert_parse!(ZoomStyleValue, "100%");
		assert_parse!(ZoomStyleValue, "100.5%");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ZoomStyleValue, "-100%");
		assert_parse_error!(ZoomStyleValue, "-10");
		assert_parse_error!(ZoomStyleValue, "smaller");
		assert_parse_error!(ZoomStyleValue, "10 10%");
		assert_parse_error!(ZoomStyleValue, "10% 10");
	}
}
