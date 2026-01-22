use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum VideoColorGamutMediaFeature{CssAtomSet::VideoColorGamut, VideoColorGamutMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum VideoColorGamutMediaFeatureKeyword {
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
		assert_eq!(std::mem::size_of::<VideoColorGamutMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, VideoColorGamutMediaFeature, "(video-color-gamut)");
		assert_parse!(CssAtomSet::ATOMS, VideoColorGamutMediaFeature, "(video-color-gamut:srgb)");
		assert_parse!(CssAtomSet::ATOMS, VideoColorGamutMediaFeature, "(video-color-gamut:p3)");
		assert_parse!(CssAtomSet::ATOMS, VideoColorGamutMediaFeature, "(video-color-gamut:rec2020)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, VideoColorGamutMediaFeature, "(video-color-gamut:)");
		assert_parse_error!(CssAtomSet::ATOMS, VideoColorGamutMediaFeature, "(video-color-gamut: rec)");
	}
}
