#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DynamicRangeLimitStyleValue, "standard");
		assert_parse!(
			CssAtomSet::ATOMS,
			DynamicRangeLimitStyleValue,
			"dynamic-range-limit-mix(no-limit 80%,standard 20%)"
		);
	}
}
