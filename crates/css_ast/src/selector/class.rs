use css_parse::{Cursor, Kind, Parser, Peek, T};
use csskit_derives::{Parse, ToCursors, ToSpan};

#[derive(Parse, ToSpan, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct Class {
	pub dot: T![.],
	pub name: T![Ident],
}

impl<'a> Peek<'a> for Class {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		c == Kind::Delim && c == '.' && p.peek_n(2) == Kind::Ident
	}
}
