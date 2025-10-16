use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum EnvironmentBlendingMediaFeature{CssAtomSet::EnvironmentBlending, EnvironmentBlendingMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum EnvironmentBlendingMediaFeatureKeyword {
	#[atom(CssAtomSet::Opaque)]
	Opaque(T![Ident]),
	#[atom(CssAtomSet::Additive)]
	Additive(T![Ident]),
	#[atom(CssAtomSet::Subtractive)]
	Subtractive(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<EnvironmentBlendingMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, EnvironmentBlendingMediaFeature, "(environment-blending)");
		assert_parse!(CssAtomSet::ATOMS, EnvironmentBlendingMediaFeature, "(environment-blending:opaque)");
		assert_parse!(CssAtomSet::ATOMS, EnvironmentBlendingMediaFeature, "(environment-blending:additive)");
		assert_parse!(CssAtomSet::ATOMS, EnvironmentBlendingMediaFeature, "(environment-blending:subtractive)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, EnvironmentBlendingMediaFeature, "(environment-blending:)");
		assert_parse_error!(CssAtomSet::ATOMS, EnvironmentBlendingMediaFeature, "(environment-blending: pointer)");
		assert_parse_error!(CssAtomSet::ATOMS, EnvironmentBlendingMediaFeature, "(pointer: subtractive)");
	}
}
