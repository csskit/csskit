use css_parse::T;
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

// https://drafts.csswg.org/selectors/#combinators
#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum Combinator {
	Child(T![>]),
	NextSibling(T![+]),
	SubsequentSibling(T![~]),
	Column(T![||]),
	Nesting(T![&]),
	Descendant(T![' ']),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Combinator>(), 28);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Combinator, ">");
		assert_parse!(CssAtomSet::ATOMS, Combinator, "+");
		assert_parse!(CssAtomSet::ATOMS, Combinator, "~");
		assert_parse!(CssAtomSet::ATOMS, Combinator, "&");
		// Descendent combinator
		assert_parse!(CssAtomSet::ATOMS, Combinator, "     ");
		assert_parse!(CssAtomSet::ATOMS, Combinator, "     ");
		assert_parse!(CssAtomSet::ATOMS, Combinator, "  /**/   /**/   /**/ ", "  ");
		// Column
		assert_parse!(CssAtomSet::ATOMS, Combinator, "||");
	}
}
