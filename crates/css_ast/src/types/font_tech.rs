use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#font-tech-values>
///
/// ```text,ignore
/// <font-tech> = [ <font-features-tech> | <color-font-tech> | variations | palettes | incremental ]
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FontTech {
	#[atom(CssAtomSet::FeaturesOpentype)]
	FeaturesOpentype(T![Ident]),
	#[atom(CssAtomSet::FeaturesAat)]
	FeaturesAat(T![Ident]),
	#[atom(CssAtomSet::FeaturesGraphite)]
	FeaturesGraphite(T![Ident]),
	#[atom(CssAtomSet::ColorColrv0)]
	ColorColrv0(T![Ident]),
	#[atom(CssAtomSet::ColorColrv1)]
	ColorColrv1(T![Ident]),
	#[atom(CssAtomSet::ColorSvg)]
	ColorSvg(T![Ident]),
	#[atom(CssAtomSet::ColorSbix)]
	ColorSbix(T![Ident]),
	#[atom(CssAtomSet::ColorCbdt)]
	ColorCbdt(T![Ident]),
	#[atom(CssAtomSet::Variations)]
	Variations(T![Ident]),
	#[atom(CssAtomSet::Palettes)]
	Palettes(T![Ident]),
	#[atom(CssAtomSet::Incremental)]
	Incremental(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FontTech, "features-opentype");
		assert_parse!(CssAtomSet::ATOMS, FontTech, "features-aat");
		assert_parse!(CssAtomSet::ATOMS, FontTech, "color-COLRv1");
		assert_parse!(CssAtomSet::ATOMS, FontTech, "color-sbix");
		assert_parse!(CssAtomSet::ATOMS, FontTech, "variations");
		assert_parse!(CssAtomSet::ATOMS, FontTech, "palettes");
		assert_parse!(CssAtomSet::ATOMS, FontTech, "incremental");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, FontTech, "\"variations\"");
	}
}
