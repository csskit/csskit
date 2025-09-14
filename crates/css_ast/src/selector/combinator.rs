use css_parse::T;
use csskit_derives::{Parse, ToCursors, ToSpan, Visitable};

// https://drafts.csswg.org/selectors/#combinators
#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Combinator>(), 28);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Combinator, ">");
		assert_parse!(Combinator, "+");
		assert_parse!(Combinator, "~");
		assert_parse!(Combinator, "&");
		// Descendent combinator
		assert_parse!(Combinator, "     ");
		assert_parse!(Combinator, "     ");
		assert_parse!(Combinator, "  /**/   /**/   /**/ ", "  ");
		// Column
		assert_parse!(Combinator, "||");
	}
}
