use crate::{Visit, VisitMut, Visitable, VisitableMut};
use css_parse::{
	Cursor, Parse, Parser, Peek, Result as ParserResult, Span, ToCursors, ToSpan, keyword_set, token_macros::Ident,
};

keyword_set!(pub struct NoneKeyword "none");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum NoneOr<T> {
	None(Ident),
	Some(T),
}

impl<'a, T: Peek<'a>> Peek<'a> for NoneOr<T> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		NoneKeyword::peek(p, c) || T::peek(p, c)
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for NoneOr<T> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<NoneKeyword>() {
			p.parse::<NoneKeyword>().map(|kw| Self::None(kw.into()))
		} else {
			p.parse::<T>().map(Self::Some)
		}
	}
}

impl<T> ToCursors for NoneOr<T>
where
	T: ToCursors,
{
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		match self {
			Self::None(ident) => ident.to_cursors(s),
			Self::Some(t) => t.to_cursors(s),
		}
	}
}

impl<T> ToSpan for NoneOr<T>
where
	T: ToSpan,
{
	fn to_span(&self) -> Span {
		match self {
			Self::None(ident) => ident.to_span(),
			Self::Some(t) => t.to_span(),
		}
	}
}

impl<T> Visitable for NoneOr<T>
where
	T: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		match self {
			Self::None(_) => {}
			Self::Some(t) => t.accept(v),
		}
	}
}

impl<T> VisitableMut for NoneOr<T>
where
	T: VisitableMut,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		match self {
			Self::None(_) => {}
			Self::Some(t) => t.accept_mut(v),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{T, assert_parse, assert_parse_error};

	type NoneOrIdent = NoneOr<T![Ident]>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<NoneOrIdent>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(NoneOrIdent, "none", NoneOrIdent::None(_));
		assert_parse!(NoneOrIdent, "all", NoneOrIdent::Some(_));
		assert_parse!(NoneOrIdent, "auto", NoneOrIdent::Some(_));
		assert_parse!(NoneOrIdent, "some", NoneOrIdent::Some(_));
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(NoneOrIdent, "");
		assert_parse_error!(NoneOrIdent, "0");
		assert_parse_error!(NoneOrIdent, "none none");
		assert_parse_error!(NoneOrIdent, "none all");
	}
}
