use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum PrefersReducedTransparencyMediaFeature<CssAtomSet::PrefersReducedTransparency, PrefersReducedTransparencyMediaFeatureKeyword>
);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum PrefersReducedTransparencyMediaFeatureKeyword {
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
		assert_eq!(std::mem::size_of::<PrefersReducedTransparencyMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PrefersReducedTransparencyMediaFeature, "(prefers-reduced-transparency)");
		assert_parse!(
			CssAtomSet::ATOMS,
			PrefersReducedTransparencyMediaFeature,
			"(prefers-reduced-transparency:no-preference)"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			PrefersReducedTransparencyMediaFeature,
			"(prefers-reduced-transparency:reduce)"
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(
			CssAtomSet::ATOMS,
			PrefersReducedTransparencyMediaFeature,
			"(prefers-reduced-transparency:)"
		);
		assert_parse_error!(
			CssAtomSet::ATOMS,
			PrefersReducedTransparencyMediaFeature,
			"(prefers-reduced-transparency: reduced)"
		);
	}
}
