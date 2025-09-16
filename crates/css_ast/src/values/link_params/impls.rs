pub(crate) use crate::{CssDiagnostic, traits::StyleValue};
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<LinkParametersStyleValue>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(LinkParametersStyleValue, "none");
		assert_parse!(LinkParametersStyleValue, "param(--foo,var(--bar))");
		assert_parse!(LinkParametersStyleValue, "param(--foo,var(--bar)),param(--bar,'bar')");
	}
}
