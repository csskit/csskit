use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum PrefersContrastMediaFeature{CssAtomSet::PrefersContrast, PrefersContrastMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PrefersContrastMediaFeatureKeyword {
	#[atom(CssAtomSet::NoPreference)]
	NoPreference(T![Ident]),
	#[atom(CssAtomSet::Less)]
	Less(T![Ident]),
	#[atom(CssAtomSet::More)]
	More(T![Ident]),
	#[atom(CssAtomSet::Custom)]
	Custom(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PrefersContrastMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PrefersContrastMediaFeature, "(prefers-contrast)");
		assert_parse!(CssAtomSet::ATOMS, PrefersContrastMediaFeature, "(prefers-contrast:no-preference)");
		assert_parse!(CssAtomSet::ATOMS, PrefersContrastMediaFeature, "(prefers-contrast:less)");
		assert_parse!(CssAtomSet::ATOMS, PrefersContrastMediaFeature, "(prefers-contrast:more)");
		assert_parse!(CssAtomSet::ATOMS, PrefersContrastMediaFeature, "(prefers-contrast:custom)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PrefersContrastMediaFeature, "(prefers-contrast:)");
		assert_parse_error!(CssAtomSet::ATOMS, PrefersContrastMediaFeature, "(prefers-contrast: no-pref)");
	}
}
