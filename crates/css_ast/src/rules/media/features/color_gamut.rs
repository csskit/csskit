use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum ColorGamutMediaFeature{CssAtomSet::ColorGamut, ColorGamutMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ColorGamutMediaFeatureKeyword {
	#[atom(CssAtomSet::Srgb)]
	Srgb(T![Ident]),
	#[atom(CssAtomSet::P3)]
	P3(T![Ident]),
	#[atom(CssAtomSet::Rec2020)]
	Rec2020(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorGamutMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColorGamutMediaFeature, "(color-gamut)");
		assert_parse!(CssAtomSet::ATOMS, ColorGamutMediaFeature, "(color-gamut:srgb)");
		assert_parse!(CssAtomSet::ATOMS, ColorGamutMediaFeature, "(color-gamut:p3)");
		assert_parse!(CssAtomSet::ATOMS, ColorGamutMediaFeature, "(color-gamut:rec2020)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ColorGamutMediaFeature, "(color-gamut:)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorGamutMediaFeature, "(color-gamut: pointer)");
	}
}
