use css_lexer::Cursor;
use css_parse::{
	Build, Function, Parse, Parser, Peek, Result as ParserResult, T, diagnostics, function_set, keyword_set,
};
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, ToSpan};

use crate::types::CounterStyle;

function_set!(
	pub enum TargetFunctionNames {
		Counter: "target-counter",
		Counters: "target-counters",
		Text: "target-text"
});

keyword_set!(
	pub enum TextFunctionContent {
		Content: "content",
		Before: "before",
		After: "after",
		FirstLetter: "first-letter"
	}
);

#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum TargetCounterKind {
	String(T![String]),
	Url(T![Url]),
}

/// <https://drafts.csswg.org/css-content-3/#typedef-target>
///
/// ```text,ignore
/// <target> = <target-counter()> | <target-counters()> | <target-text()>
/// ```
#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Target<'a> {
	// https://drafts.csswg.org/css-content-3/#target-counter
	// target-counter() = target-counter( [ <string> | <url> ] , <custom-ident> , <counter-style>? )
	TargetCounter(Function<TargetFunctionNames, TargetCounterParams<'a>>),
	// https://drafts.csswg.org/css-content-3/#target-counters
	// target-counters() = target-counters( [ <string> | <url> ] , <custom-ident> , <string> , <counter-style>? )
	TargetCounters(Function<TargetFunctionNames, TargetCountersParams<'a>>),
	// https://drafts.csswg.org/css-content-3/#target-text
	// target-text() = target-text( [ <string> | <url> ] , [ content | before | after | first-letter ]? )
	TargetText(Function<TargetFunctionNames, TargetTextParams>),
}

impl<'a> Peek<'a> for Target<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		TargetFunctionNames::peek(p, c)
	}
}

impl<'a> Parse<'a> for Target<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let c = p.peek_n(1);
		if !TargetFunctionNames::peek(p, c) {
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
		match TargetFunctionNames::build(p, c) {
			TargetFunctionNames::Counter(_) => {
				p.parse::<Function<TargetFunctionNames, TargetCounterParams<'a>>>().map(Self::TargetCounter)
			}
			TargetFunctionNames::Counters(_) => {
				p.parse::<Function<TargetFunctionNames, TargetCountersParams<'a>>>().map(Self::TargetCounters)
			}
			TargetFunctionNames::Text(_) => {
				p.parse::<Function<TargetFunctionNames, TargetTextParams>>().map(Self::TargetText)
			}
		}
	}
}

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TargetCounterParams<'a>(
	TargetCounterKind,
	Option<T![,]>,
	T![Ident],
	Option<T![,]>,
	Option<CounterStyle<'a>>,
);

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TargetCountersParams<'a>(
	TargetCounterKind,
	Option<T![,]>,
	T![Ident],
	Option<T![,]>,
	T![String],
	Option<T![,]>,
	Option<CounterStyle<'a>>,
);

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TargetTextParams(TargetCounterKind, Option<T![,]>, Option<TextFunctionContent>);

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
