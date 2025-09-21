use super::prelude::*;

#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
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
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LineStyle>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineStyle, "none");
		assert_parse!(CssAtomSet::ATOMS, LineStyle, "hidden");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, LineStyle, "florp");
		// Empty!
		assert_parse_error!(CssAtomSet::ATOMS, LineStyle, "");
	}
}
