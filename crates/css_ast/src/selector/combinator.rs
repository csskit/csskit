use css_parse::{Parse, Parser, Result as ParserResult, T};
use csskit_derives::{ToCursors, ToSpan, Visitable};

// https://drafts.csswg.org/selectors/#combinators
#[derive(ToSpan, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'a> Parse<'a> for Combinator {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![>]>() {
			Ok(Self::Child(p.parse::<T![>]>()?))
		} else if p.peek::<T![+]>() {
			Ok(Self::NextSibling(p.parse::<T![+]>()?))
		} else if p.peek::<T![~]>() {
			Ok(Self::SubsequentSibling(p.parse::<T![~]>()?))
		} else if p.peek::<T![&]>() {
			Ok(Self::Nesting(p.parse::<T![&]>()?))
		} else if p.peek::<T![||]>() {
			Ok(Self::Column(p.parse::<T![||]>()?))
		} else {
			Ok(Self::Descendant(p.parse::<T![' ']>()?))
		}
	}
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
