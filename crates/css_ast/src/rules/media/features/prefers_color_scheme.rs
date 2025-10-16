use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum PrefersColorSchemeMediaFeature{CssAtomSet::PrefersColorScheme, PrefersColorSchemeMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum PrefersColorSchemeMediaFeatureKeyword {
	#[atom(CssAtomSet::Light)]
	Light(T![Ident]),
	#[atom(CssAtomSet::Dark)]
	Dark(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PrefersColorSchemeMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PrefersColorSchemeMediaFeature, "(prefers-color-scheme)");
		assert_parse!(CssAtomSet::ATOMS, PrefersColorSchemeMediaFeature, "(prefers-color-scheme:light)");
		assert_parse!(CssAtomSet::ATOMS, PrefersColorSchemeMediaFeature, "(prefers-color-scheme:dark)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PrefersColorSchemeMediaFeature, "(prefers-color-scheme:)");
		assert_parse_error!(CssAtomSet::ATOMS, PrefersColorSchemeMediaFeature, "(prefers-color-scheme: dimmed)");
	}
}
