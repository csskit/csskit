use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum OverflowInlineMediaFeature{CssAtomSet::OverflowInline, OverflowInlineMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum OverflowInlineMediaFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Scroll)]
	Scroll(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OverflowInlineMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, OverflowInlineMediaFeature, "(overflow-inline)");
		assert_parse!(CssAtomSet::ATOMS, OverflowInlineMediaFeature, "(overflow-inline:none)");
		assert_parse!(CssAtomSet::ATOMS, OverflowInlineMediaFeature, "(overflow-inline:scroll)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, OverflowInlineMediaFeature, "(overflow-inline:)");
		assert_parse_error!(CssAtomSet::ATOMS, OverflowInlineMediaFeature, "(overflow-inline: page)");
	}
}
