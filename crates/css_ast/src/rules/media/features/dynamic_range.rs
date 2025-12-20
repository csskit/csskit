use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum DynamicRangeMediaFeature{CssAtomSet::DynamicRange, DynamicRangeMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum DynamicRangeMediaFeatureKeyword {
	#[atom(CssAtomSet::Standard)]
	Standard(T![Ident]),
	#[atom(CssAtomSet::High)]
	High(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DynamicRangeMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DynamicRangeMediaFeature, "(dynamic-range)");
		assert_parse!(CssAtomSet::ATOMS, DynamicRangeMediaFeature, "(dynamic-range:standard)");
		assert_parse!(CssAtomSet::ATOMS, DynamicRangeMediaFeature, "(dynamic-range:high)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, DynamicRangeMediaFeature, "(dynamic-range:)");
		assert_parse_error!(CssAtomSet::ATOMS, DynamicRangeMediaFeature, "(dynamic-range: pointer)");
		assert_parse_error!(CssAtomSet::ATOMS, DynamicRangeMediaFeature, "(pointer: standard)");
	}
}
