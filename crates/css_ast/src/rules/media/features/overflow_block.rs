use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum OverflowBlockMediaFeature{CssAtomSet::OverflowBlock, OverflowBlockMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum OverflowBlockMediaFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Scroll)]
	Scroll(T![Ident]),
	#[atom(CssAtomSet::Paged)]
	Paged(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OverflowBlockMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, OverflowBlockMediaFeature, "(overflow-block)");
		assert_parse!(CssAtomSet::ATOMS, OverflowBlockMediaFeature, "(overflow-block:none)");
		assert_parse!(CssAtomSet::ATOMS, OverflowBlockMediaFeature, "(overflow-block:scroll)");
		assert_parse!(CssAtomSet::ATOMS, OverflowBlockMediaFeature, "(overflow-block:paged)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, OverflowBlockMediaFeature, "(overflow-block:)");
		assert_parse_error!(CssAtomSet::ATOMS, OverflowBlockMediaFeature, "(overflow-block: page)");
	}
}
