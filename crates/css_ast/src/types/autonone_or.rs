use crate::{Visit, VisitMut, Visitable, VisitableMut};
use css_parse::{
	Cursor, Parse, Parser, Peek, Result as ParserResult, Span, ToCursors, ToNumberValue, ToSpan, keyword_set,
	token_macros::Ident,
};

keyword_set!(pub enum AutoOrNoneKeywords {
	Auto: "auto"
	None: "none"
});

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AutoNoneOr<T> {
	Auto(Ident),
	None(Ident),
	Some(T),
}

impl<'a, T: Peek<'a>> Peek<'a> for AutoNoneOr<T> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		AutoOrNoneKeywords::peek(p, c) || T::peek(p, c)
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for AutoNoneOr<T> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		match p.parse_if_peek::<AutoOrNoneKeywords>()? {
			Some(AutoOrNoneKeywords::Auto(kw)) => Ok(Self::Auto(kw)),
			Some(AutoOrNoneKeywords::None(kw)) => Ok(Self::None(kw)),
			None => p.parse::<T>().map(Self::Some),
		}
	}
}

impl<T> ToCursors for AutoNoneOr<T>
where
	T: ToCursors,
{
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		match self {
			Self::Auto(ident) => ident.to_cursors(s),
			Self::None(ident) => ident.to_cursors(s),
			Self::Some(t) => t.to_cursors(s),
		}
	}
}

impl<T> ToSpan for AutoNoneOr<T>
where
	T: ToSpan,
{
	fn to_span(&self) -> Span {
		match self {
			Self::Auto(ident) => ident.to_span(),
			Self::None(ident) => ident.to_span(),
			Self::Some(t) => t.to_span(),
		}
	}
}

impl<T> Visitable for AutoNoneOr<T>
where
	T: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) {
		match self {
			Self::Auto(_) | Self::None(_) => {}
			Self::Some(t) => t.accept(v),
		}
	}
}

impl<T> VisitableMut for AutoNoneOr<T>
where
	T: VisitableMut,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		match self {
			Self::Auto(_) | Self::None(_) => {}
			Self::Some(t) => t.accept_mut(v),
		}
	}
}

impl<T: ToNumberValue> ToNumberValue for AutoNoneOr<T> {
	fn to_number_value(&self) -> Option<f32> {
		match self {
			Self::None(_) => None,
			Self::Auto(_) => None,
			Self::Some(t) => t.to_number_value(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Length;
	use bumpalo::Bump;
	use css_parse::{T, assert_parse, assert_parse_error};

	type AuroNoneOrIdent = AutoNoneOr<T![Ident]>;
	type AutoNoneOrNumber = AutoNoneOr<T![Number]>;
	type AutoNoneOrLength = AutoNoneOr<Length>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AuroNoneOrIdent>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AuroNoneOrIdent, "auto", AuroNoneOrIdent::Auto(_));
		assert_parse!(AuroNoneOrIdent, "none", AuroNoneOrIdent::None(_));
		assert_parse!(AuroNoneOrIdent, "all", AuroNoneOrIdent::Some(_));
		assert_parse!(AuroNoneOrIdent, "some", AuroNoneOrIdent::Some(_));
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(AuroNoneOrIdent, "");
		assert_parse_error!(AuroNoneOrIdent, "0");
		assert_parse_error!(AuroNoneOrIdent, "auto none");
		assert_parse_error!(AuroNoneOrIdent, "none none");
		assert_parse_error!(AuroNoneOrIdent, "auto auto");
		assert_parse_error!(AuroNoneOrIdent, "auto all");
	}

	#[test]
	fn test_to_number_value() {
		let bump = Bump::default();
		let source_text = "47";
		let mut p = Parser::new(&bump, source_text);
		let num = p.parse_entirely::<AutoNoneOrNumber>().output.unwrap();
		assert_eq!(num.to_number_value(), Some(47.0));

		let source_text = "47px";
		let mut p = Parser::new(&bump, source_text);
		let num = p.parse_entirely::<AutoNoneOrLength>().output.unwrap();
		assert_eq!(num.to_number_value(), Some(47.0));

		let source_text = "none";
		let mut p = Parser::new(&bump, source_text);
		let num = p.parse_entirely::<AutoNoneOrLength>().output.unwrap();
		assert_eq!(num.to_number_value(), None);
	}
}
