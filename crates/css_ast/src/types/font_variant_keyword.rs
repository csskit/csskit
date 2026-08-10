use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-5/#font-variant-css2>
///
/// ```text,ignore
/// <font-variant-css2> = normal | small-caps
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FontVariantKeyword {
	#[atom(CssAtomSet::Normal)]
	Normal(T![Ident]),
	#[atom(CssAtomSet::SmallCaps)]
	SmallCaps(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FontVariantKeyword, "normal");
		assert_parse!(CssAtomSet::ATOMS, FontVariantKeyword, "small-caps");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, FontVariantKeyword, "all-small-caps");
	}
}
