use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum DisplayModeMediaFeature{CssAtomSet::DisplayMode, DisplayModeMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum DisplayModeMediaFeatureKeyword {
	#[atom(CssAtomSet::Fullscreen)]
	Fullscreen(T![Ident]),
	#[atom(CssAtomSet::Standalone)]
	Standalone(T![Ident]),
	#[atom(CssAtomSet::MinimalUi)]
	MinimalUi(T![Ident]),
	#[atom(CssAtomSet::Browser)]
	Browser(T![Ident]),
	#[atom(CssAtomSet::PictureInPicture)]
	PictureInPicture(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DisplayModeMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DisplayModeMediaFeature, "(display-mode)");
		assert_parse!(CssAtomSet::ATOMS, DisplayModeMediaFeature, "(display-mode:fullscreen)");
		assert_parse!(CssAtomSet::ATOMS, DisplayModeMediaFeature, "(display-mode:minimal-ui)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, DisplayModeMediaFeature, "(display-mode:)");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayModeMediaFeature, "(display-mode: pointer)");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayModeMediaFeature, "(pointer: standalone)");
	}
}
