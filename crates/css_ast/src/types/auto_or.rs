use crate::{Visit, VisitMut, Visitable, VisitableMut};
use css_parse::{
	Cursor, Parse, Parser, Peek, Result as ParserResult, Span, ToCursors, ToSpan, keyword_set, token_macros::Ident,
};

keyword_set!(pub struct AutoKeyword "auto");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AutoOr<T> {
	Auto(Ident),
	Some(T),
}

impl<'a, T: Peek<'a>> Peek<'a> for AutoOr<T> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		AutoKeyword::peek(p, c) || T::peek(p, c)
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for AutoOr<T> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<AutoKeyword>() {
			p.parse::<AutoKeyword>().map(|kw| Self::Auto(kw.into()))
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
	fn to_span(&self) -> Span {
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

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{T, assert_parse, assert_parse_error};

	type AutoOrIdent = AutoOr<T![Ident]>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AutoOrIdent>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AutoOrIdent, "auto", AutoOrIdent::Auto(_));
		assert_parse!(AutoOrIdent, "all", AutoOrIdent::Some(_));
		assert_parse!(AutoOrIdent, "none", AutoOrIdent::Some(_));
		assert_parse!(AutoOrIdent, "some", AutoOrIdent::Some(_));
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(AutoOrIdent, "");
		assert_parse_error!(AutoOrIdent, "0");
		assert_parse_error!(AutoOrIdent, "auto auto");
		assert_parse_error!(AutoOrIdent, "auto all");
	}
}
