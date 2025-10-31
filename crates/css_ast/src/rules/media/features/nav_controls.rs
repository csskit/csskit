use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum NavControlsMediaFeature{CssAtomSet::NavControls, NavControlsMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum NavControlsMediaFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Back)]
	Back(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<NavControlsMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, NavControlsMediaFeature, "(nav-controls)");
		assert_parse!(CssAtomSet::ATOMS, NavControlsMediaFeature, "(nav-controls:back)");
		assert_parse!(CssAtomSet::ATOMS, NavControlsMediaFeature, "(nav-controls:none)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, NavControlsMediaFeature, "(nav-controls:)");
		assert_parse_error!(CssAtomSet::ATOMS, NavControlsMediaFeature, "(nav-controls: hoover)");
	}
}
