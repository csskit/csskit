use css_parse::{Cursor, Kind, KindSet, Parser, Peek, T};
use csskit_derives::{Parse, SemanticEq, ToCursors, ToSpan};

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Class {
	pub dot: T![.],
	pub name: T![Ident],
}

impl<'a> Peek<'a> for Class {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Delim]);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		c == Kind::Delim && c == '.' && p.peek_n(2) == Kind::Ident
	}
}
