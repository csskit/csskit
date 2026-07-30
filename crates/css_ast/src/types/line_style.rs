use super::prelude::*;

#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum LineStyle {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Hidden)]
	Hidden(T![Ident]),
	#[atom(CssAtomSet::Dotted)]
	Dotted(T![Ident]),
	#[atom(CssAtomSet::Dashed)]
	Dashed(T![Ident]),
	#[atom(CssAtomSet::Solid)]
	Solid(T![Ident]),
	#[atom(CssAtomSet::Double)]
	Double(T![Ident]),
	#[atom(CssAtomSet::Groove)]
	Groove(T![Ident]),
	#[atom(CssAtomSet::Ridge)]
	Ridge(T![Ident]),
	#[atom(CssAtomSet::Inset)]
	Inset(T![Ident]),
	#[atom(CssAtomSet::Outset)]
	Outset(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineStyle, "none");
		assert_parse!(CssAtomSet::ATOMS, LineStyle, "hidden");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, LineStyle, "florp");
		// Empty!
		assert_peek_false!(CssAtomSet::ATOMS, LineStyle, "");
	}
}
