use css_parse::{Cursor, Kind, Parser, Peek, T};
use csskit_derives::{Parse, ToCursors, ToSpan, Visitable};

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct Class {
	pub dot: T![.],
	pub name: T![Ident],
}

impl<'a> Peek<'a> for Class {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		c == Kind::Delim && c == '.' && p.peek_n(2) == Kind::Ident
	}
}
