use super::prelude::*;

/// The origin/positioning-area keyword set shared verbatim by `fill-origin`
/// and `stroke-origin`.
///
/// ```text,ignore
/// match-parent | fill-box | stroke-box | content-box | padding-box | border-box
/// ```
///
/// <https://drafts.csswg.org/fill-stroke-3/#fill-origin>
/// <https://drafts.csswg.org/fill-stroke-3/#stroke-origin>
#[derive(
	Parse, Peek, IntoCursor, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum FillOrigin {
	#[atom(CssAtomSet::MatchParent)]
	MatchParent(T![Ident]),
	#[atom(CssAtomSet::FillBox)]
	FillBox(T![Ident]),
	#[atom(CssAtomSet::StrokeBox)]
	StrokeBox(T![Ident]),
	#[atom(CssAtomSet::ContentBox)]
	ContentBox(T![Ident]),
	#[atom(CssAtomSet::PaddingBox)]
	PaddingBox(T![Ident]),
	#[atom(CssAtomSet::BorderBox)]
	BorderBox(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FillOrigin>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FillOrigin, "match-parent");
		assert_parse!(CssAtomSet::ATOMS, FillOrigin, "fill-box");
		assert_parse!(CssAtomSet::ATOMS, FillOrigin, "stroke-box");
		assert_parse!(CssAtomSet::ATOMS, FillOrigin, "content-box");
		assert_parse!(CssAtomSet::ATOMS, FillOrigin, "padding-box");
		assert_parse!(CssAtomSet::ATOMS, FillOrigin, "border-box");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, FillOrigin, "");
		assert_peek_false!(CssAtomSet::ATOMS, FillOrigin, "auto");
	}
}
