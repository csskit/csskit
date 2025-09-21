use crate::CssAtomSet;
use css_parse::T;
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

#[derive(Peek, Parse, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum SystemColor {
	#[atom(CssAtomSet::Accentcolor)]
	Accentcolor(T![Ident]),
	#[atom(CssAtomSet::Accentcolortext)]
	Accentcolortext(T![Ident]),
	#[atom(CssAtomSet::Activetext)]
	Activetext(T![Ident]),
	#[atom(CssAtomSet::Buttonborder)]
	Buttonborder(T![Ident]),
	#[atom(CssAtomSet::Buttonface)]
	Buttonface(T![Ident]),
	#[atom(CssAtomSet::Buttontext)]
	Buttontext(T![Ident]),
	#[atom(CssAtomSet::Canvas)]
	Canvas(T![Ident]),
	#[atom(CssAtomSet::Canvastext)]
	Canvastext(T![Ident]),
	#[atom(CssAtomSet::Field)]
	Field(T![Ident]),
	#[atom(CssAtomSet::Fieldtext)]
	Fieldtext(T![Ident]),
	#[atom(CssAtomSet::Graytext)]
	Graytext(T![Ident]),
	#[atom(CssAtomSet::Highlight)]
	Highlight(T![Ident]),
	#[atom(CssAtomSet::Highlighttext)]
	Highlighttext(T![Ident]),
	#[atom(CssAtomSet::Linktext)]
	Linktext(T![Ident]),
	#[atom(CssAtomSet::Mark)]
	Mark(T![Ident]),
	#[atom(CssAtomSet::Marktext)]
	Marktext(T![Ident]),
	#[atom(CssAtomSet::Selecteditem)]
	Selecteditem(T![Ident]),
	#[atom(CssAtomSet::Selecteditemtext)]
	Selecteditemtext(T![Ident]),
	#[atom(CssAtomSet::Visitedtext)]
	Visitedtext(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SystemColor>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SystemColor, "marktext");
		assert_parse!(CssAtomSet::ATOMS, SystemColor, "visitedtext");
		assert_parse!(CssAtomSet::ATOMS, SystemColor, "graytext");
	}
}
