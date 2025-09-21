use super::prelude::*;

/// <https://drafts.csswg.org/css-color-hdr-1/#dynamic-range-limit-mix>
///
/// ```text,ignore
/// dynamic-range-limit-mix() = dynamic-range-limit-mix( [ <'dynamic-range-limit'> && <percentage [0,100]> ]#{2,} )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct DynamicRangeLimitMixFunction<'a> {
	#[atom(CssAtomSet::DynamicRangeLimitMix)]
	pub name: T![Function],
	pub params: CommaSeparated<'a, DynamicRangeLimitMixFunctionParams<'a>>,
	pub close: T![')'],
}

#[syntax(" <'dynamic-range-limit'> && <percentage [0,100]> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DynamicRangeLimitMixFunctionParams<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DynamicRangeLimitMixFunction>(), 56);
	}

	#[test]
	fn test_writes() {
		assert_parse!(
			CssAtomSet::ATOMS,
			DynamicRangeLimitMixFunction,
			"dynamic-range-limit-mix(constrained 80%,standard 20%)"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			DynamicRangeLimitMixFunction,
			"dynamic-range-limit-mix(80% constrained,20% standard)",
			"dynamic-range-limit-mix(constrained 80%,standard 20%)"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			DynamicRangeLimitMixFunction,
			"dynamic-range-limit-mix(constrained 8%,standard 2%)"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			DynamicRangeLimitMixFunction,
			"dynamic-range-limit-mix(constrained 8%,no-limit 2%)"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			DynamicRangeLimitMixFunction,
			"dynamic-range-limit-mix(dynamic-range-limit-mix(constrained 8%)2%,no-limit 2%)"
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(
			CssAtomSet::ATOMS,
			DynamicRangeLimitMixFunction,
			"dynamic-range-limit-mix(constrained -1%, standard 20%)"
		);
		assert_parse_error!(
			CssAtomSet::ATOMS,
			DynamicRangeLimitMixFunction,
			"dynamic-range-limit-mix(constrained 200%, standard 20%)"
		);
		assert_parse_error!(
			CssAtomSet::ATOMS,
			DynamicRangeLimitMixFunction,
			"dynamic-range-limit-mix(dynamic-range-limit-mix(constrained 200%), standard 20%)"
		);
	}
}
