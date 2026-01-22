use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum PointerMediaFeature{CssAtomSet::Pointer, PointerMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PointerMediaFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Coarse)]
	Coarse(T![Ident]),
	#[atom(CssAtomSet::Fine)]
	Fine(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PointerMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PointerMediaFeature, "(pointer)");
		assert_parse!(CssAtomSet::ATOMS, PointerMediaFeature, "(pointer:none)");
		assert_parse!(CssAtomSet::ATOMS, PointerMediaFeature, "(pointer:coarse)");
		assert_parse!(CssAtomSet::ATOMS, PointerMediaFeature, "(pointer:fine)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PointerMediaFeature, "(pointer:)");
		assert_parse_error!(CssAtomSet::ATOMS, PointerMediaFeature, "(pointer: pointer)");
	}
}
