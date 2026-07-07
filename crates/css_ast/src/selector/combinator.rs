use css_parse::T;
use csskit_derives::*;

/// <https://drafts.csswg.org/selectors/#combinators>
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum Combinator {
	Child(#[semantic_eq(skip)] T![>]),
	NextSibling(#[semantic_eq(skip)] T![+]),
	SubsequentSibling(#[semantic_eq(skip)] T![~]),
	Column(#[semantic_eq(skip)] T![||]),
	Nesting(#[semantic_eq(skip)] T![&]),
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
		assert_parse!(CssAtomSet::ATOMS, Combinator, "  /**/   /**/   /**/ ");
		// Column
		assert_parse!(CssAtomSet::ATOMS, Combinator, "||");
	}
}
