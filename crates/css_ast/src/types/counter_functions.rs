use css_lexer::Cursor;
use css_parse::{Build, Parse, Parser, Peek, Result as ParserResult, T, diagnostics, function_set};
use csskit_derives::{IntoSpan, ToCursors};

use crate::types::CounterStyle;

function_set!(CounterFunctionNames { Counter: "counter", Counters: "counters" });

// https://drafts.csswg.org/css-lists-3/#counter-functions
// <counter> = <counter()> | <counters()>
#[derive(IntoSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Counter<'a> {
	// <counter()>  =  counter( <counter-name>, <counter-style>? )
	Counter(T![Function], T![Ident], Option<T![,]>, Option<CounterStyle<'a>>, Option<T![')']>),
	// <counters()> = counters( <counter-name>, <string>, <counter-style>? )
	Counters(
		T![Function],
		T![Ident],
		Option<T![,]>,
		T![String],
		Option<T![,]>,
		Option<CounterStyle<'a>>,
		Option<T![')']>,
	),
}

impl<'a> Peek<'a> for Counter<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		CounterFunctionNames::peek(p, c)
	}
}

impl<'a> Counter<'a> {
	fn parse_counter_name(p: &mut Parser<'a>) -> ParserResult<T![Ident]> {
		let counter_name = p.parse::<T![Ident]>()?;
		if p.eq_ignore_ascii_case(counter_name.into(), "none") {
			let c: Cursor = counter_name.into();
			Err(diagnostics::InvalidCounterName(p.parse_str(c).into(), c.into()))?
		}
		Ok(counter_name)
	}
}

impl<'a> Parse<'a> for Counter<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		match p.parse::<CounterFunctionNames>()? {
			CounterFunctionNames::Counter(c) => Ok(Self::Counter(
				<T![Function]>::build(p, c),
				Self::parse_counter_name(p)?,
				p.parse_if_peek::<T![,]>()?,
				p.parse_if_peek::<CounterStyle<'a>>()?,
				p.parse_if_peek::<T![')']>()?,
			)),
			CounterFunctionNames::Counters(c) => Ok(Self::Counters(
				<T![Function]>::build(p, c),
				Self::parse_counter_name(p)?,
				p.parse_if_peek::<T![,]>()?,
				p.parse::<T![String]>()?,
				p.parse_if_peek::<T![,]>()?,
				p.parse_if_peek::<CounterStyle<'a>>()?,
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
		assert_eq!(std::mem::size_of::<Counter>(), 168);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Counter, "counter(foo)");
		assert_parse!(Counter, "counter(foo,upper-latin)");
		assert_parse!(Counter, "counters(foo,'bar')");
		assert_parse!(Counter, "counters(foo,'bar',upper-latin)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Counter, "counter(none)");
		assert_parse_error!(Counter, "counters(none)");
	}
}
