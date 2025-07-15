use css_lexer::Cursor;
use css_parse::{Build, Parse, Parser, Peek, Result as ParserResult, T, function_set, keyword_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

use crate::types::CounterStyle;

function_set!(TargetFunctionNames { Counter: "target-counter", Counters: "target-counters", Text: "target-text" });
keyword_set!(pub enum TextFunctionContent { Content: "content", Before: "before", After: "after", FirstLetter: "first-letter" });

#[derive(Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum TargetCounterKind {
	String(T![String]),
	Url(T![Url]),
}

// https://drafts.csswg.org/css-content-3/#typedef-target
// <target> = <target-counter()> | <target-counters()> | <target-text()>
#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Target<'a> {
	// https://drafts.csswg.org/css-content-3/#target-counter
	// target-counter() = target-counter( [ <string> | <url> ] , <custom-ident> , <counter-style>? )
	Counter(
		T![Function],
		TargetCounterKind,
		Option<T![,]>,
		T![Ident],
		Option<T![,]>,
		Option<CounterStyle<'a>>,
		Option<T![')']>,
	),
	// https://drafts.csswg.org/css-content-3/#target-counters
	// target-counters() = target-counters( [ <string> | <url> ] , <custom-ident> , <string> , <counter-style>? )
	Counters(
		T![Function],
		TargetCounterKind,
		Option<T![,]>,
		T![Ident],
		Option<T![,]>,
		T![String],
		Option<T![,]>,
		Option<CounterStyle<'a>>,
		Option<T![')']>,
	),
	// https://drafts.csswg.org/css-content-3/#target-text
	// target-text() = target-text( [ <string> | <url> ] , [ content | before | after | first-letter ]? )
	Text(T![Function], TargetCounterKind, Option<T![,]>, Option<TextFunctionContent>, Option<T![')']>),
}

impl<'a> Peek<'a> for Target<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		TargetFunctionNames::peek(p, c)
	}
}

impl<'a> Parse<'a> for Target<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		match p.parse::<TargetFunctionNames>()? {
			TargetFunctionNames::Counter(c) => Ok(Self::Counter(
				<T![Function]>::build(p, c),
				p.parse::<TargetCounterKind>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse::<T![Ident]>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse_if_peek::<CounterStyle<'a>>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			TargetFunctionNames::Counters(c) => Ok(Self::Counters(
				<T![Function]>::build(p, c),
				p.parse::<TargetCounterKind>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse::<T![Ident]>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse::<T![String]>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse_if_peek::<CounterStyle<'a>>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			TargetFunctionNames::Text(c) => Ok(Self::Text(
				<T![Function]>::build(p, c),
				p.parse::<TargetCounterKind>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse_if_peek::<TextFunctionContent>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Target>(), 200);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Target, "target-counter('foo',bar,lower-roman)");
		assert_parse!(Target, "target-counters('foo',bar,'baz',lower-roman)");
		assert_parse!(Target, "target-text('foo')");
		assert_parse!(Target, "target-text('foo',before)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Target, "target-counter()");
		assert_parse_error!(Target, "target-counter('foo')");
		assert_parse_error!(Target, "target-counters()");
		assert_parse_error!(Target, "target-counters('foo')");
		assert_parse_error!(Target, "target-counters('foo',bar)");
		assert_parse_error!(Target, "target-text()");
		assert_parse_error!(Target, "target-text(123)");
	}
}
