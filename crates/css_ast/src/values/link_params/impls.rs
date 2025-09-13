#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<LinkParametersStyleValue>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LinkParametersStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, LinkParametersStyleValue, "param(--foo,var(--bar))");
		assert_parse!(CssAtomSet::ATOMS, LinkParametersStyleValue, "param(--foo,var(--bar)),param(--bar,'bar')");
	}
}
