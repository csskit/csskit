use css_parse::{Cursor, Kind, Parse, Parser, Peek, Result as ParserResult, T};
use csskit_derives::{ToCursors, ToSpan, Visitable};

#[derive(ToSpan, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
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

impl<'a> Parse<'a> for Class {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let dot = p.parse::<T![.]>()?;
		let name = p.parse::<T![Ident]>()?;
		Ok(Self { dot, name })
	}
}
