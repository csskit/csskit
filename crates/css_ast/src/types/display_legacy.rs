use super::prelude::*;

/// <https://drafts.csswg.org/css-display-4/#typedef-display-legacy>
///
/// ```text,ignore
/// <display-box> = inline-block | inline-table | inline-flex | inline-grid
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum DisplayLegacy {
	#[atom(CssAtomSet::InlineBlock)]
	InlineBlock(T![Ident]),
	#[atom(CssAtomSet::InlineTable)]
	InlineTable(T![Ident]),
	#[atom(CssAtomSet::InlineFlex)]
	InlineFlex(T![Ident]),
	#[atom(CssAtomSet::InlineGrid)]
	InlineGrid(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacy, "inline-block");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacy, "inline-table");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacy, "inline-flex");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacy, "inline-grid");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, DisplayLegacy, "foo");
	}
}
