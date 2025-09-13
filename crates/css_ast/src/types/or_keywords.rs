use crate::{CssAtomSet, Visit, VisitMut, Visitable, VisitableMut};
use css_parse::{
	token_macros::Ident, Cursor, Parse, Parser, Peek, Result as ParserResult, ToCursors, ToSpan, T,
};
use csskit_derives::{Parse, Peek, ToCursors, Visitable};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
enum AutoOr<T> {
	Auto(Ident),
	Some(T),
}

impl<'a, T> Peek<'a> for AutoOr<T>
where
	T: Peek<'a>,
{
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		AutoKeyword::peek(p, c) || T::peek(p, c)
	}
}

impl<'a, T> Parse<'a> for AutoOr<T>
where
	T: Parse<'a>,
{
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<AutoKeyword>() {
			p.parse::<AutoKeyword>().map(|AutoKeyword::Auto(ident)| Self::Auto(ident))
		} else {
			p.parse::<T>().map(Self::Some)
		}
	}
}

impl<T> ToCursors for AutoOr<T>
where
	T: ToCursors,
{
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		match self {
			Self::Auto(ident) => ident.to_cursors(s),
			Self::Some(t) => t.to_cursors(s),
		}
	}
}

impl<T> ToSpan for AutoOr<T>
where
	T: ToSpan,
{
	fn to_span(&self) -> css_lexer::Span {
		match self {
			Self::Auto(ident) => ident.to_span(),
			Self::Some(t) => t.to_span(),
		}
	}
}

impl<T> Visitable for AutoOr<T>
where
	T: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		match self {
			Self::Auto(_) => {}
			Self::Some(t) => t.accept(v),
		}
	}
}

impl<T> VisitableMut for AutoOr<T>
where
	T: VisitableMut,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		match self {
			Self::Auto(_) => {}
			Self::Some(t) => t.accept_mut(v),
		}
	}
}
