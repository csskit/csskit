use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum PrefersReducedMotionMediaFeature{CssAtomSet::PrefersReducedMotion, PrefersReducedMotionMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PrefersReducedMotionMediaFeatureKeyword {
	#[atom(CssAtomSet::NoPreference)]
	NoPreference(T![Ident]),
	#[atom(CssAtomSet::Reduce)]
	Reduce(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PrefersReducedMotionMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PrefersReducedMotionMediaFeature, "(prefers-reduced-motion)");
		assert_parse!(CssAtomSet::ATOMS, PrefersReducedMotionMediaFeature, "(prefers-reduced-motion:no-preference)");
		assert_parse!(CssAtomSet::ATOMS, PrefersReducedMotionMediaFeature, "(prefers-reduced-motion:reduce)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PrefersReducedMotionMediaFeature, "(prefers-reduced-motion:)");
		assert_parse_error!(CssAtomSet::ATOMS, PrefersReducedMotionMediaFeature, "(prefers-reduced-motion: reduced)");
	}
}
