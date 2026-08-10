use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-5/#font-width-css3>
///
/// ```text,ignore
/// <font-width-css3> = normal | ultra-condensed | extra-condensed | condensed | semi-condensed | semi-expanded | expanded | extra-expanded | ultra-expanded
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FontWidthKeyword {
	#[atom(CssAtomSet::Normal)]
	Normal(T![Ident]),
	#[atom(CssAtomSet::UltraCondensed)]
	UltraCondensed(T![Ident]),
	#[atom(CssAtomSet::ExtraCondensed)]
	ExtraCondensed(T![Ident]),
	#[atom(CssAtomSet::Condensed)]
	Condensed(T![Ident]),
	#[atom(CssAtomSet::SemiCondensed)]
	SemiCondensed(T![Ident]),
	#[atom(CssAtomSet::SemiExpanded)]
	SemiExpanded(T![Ident]),
	#[atom(CssAtomSet::Expanded)]
	Expanded(T![Ident]),
	#[atom(CssAtomSet::ExtraExpanded)]
	ExtraExpanded(T![Ident]),
	#[atom(CssAtomSet::UltraExpanded)]
	UltraExpanded(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FontWidthKeyword, "normal");
		assert_parse!(CssAtomSet::ATOMS, FontWidthKeyword, "ultra-condensed");
		assert_parse!(CssAtomSet::ATOMS, FontWidthKeyword, "semi-expanded");
		assert_parse!(CssAtomSet::ATOMS, FontWidthKeyword, "ultra-expanded");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, FontWidthKeyword, "50%");
	}
}
