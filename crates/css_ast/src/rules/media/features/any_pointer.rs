use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum AnyPointerMediaFeature{CssAtomSet::AnyPointer, AnyPointerMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AnyPointerMediaFeatureKeyword {
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AnyPointerMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AnyPointerMediaFeature, "(any-pointer)");
		assert_parse!(CssAtomSet::ATOMS, AnyPointerMediaFeature, "(any-pointer:none)");
		assert_parse!(CssAtomSet::ATOMS, AnyPointerMediaFeature, "(any-pointer:coarse)");
		assert_parse!(CssAtomSet::ATOMS, AnyPointerMediaFeature, "(any-pointer:fine)");
	}
}
