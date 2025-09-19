use super::prelude::*;

use crate::types::CounterStyle;

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
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum Target<'a> {
	// https://drafts.csswg.org/css-content-3/#target-counter
	// target-counter() = target-counter( [ <string> | <url> ] , <custom-ident> , <counter-style>? )
	TargetCounter(TargetCounterFunction<'a>),
	// https://drafts.csswg.org/css-content-3/#target-counters
	// target-counters() = target-counters( [ <string> | <url> ] , <custom-ident> , <string> , <counter-style>? )
	TargetCounters(TargetCountersFunction<'a>),
	// https://drafts.csswg.org/css-content-3/#target-text
	// target-text() = target-text( [ <string> | <url> ] , [ content | before | after | first-letter ]? )
	TargetText(TargetTextFunction),
}

function_set!(pub struct TargetCounterFunctionName "target-counter");

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct TargetCounterFunction<'a>(Function<TargetCounterFunctionName, TargetCounterParams<'a>>);

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TargetCounterParams<'a>(
	TargetCounterKind,
	Option<T![,]>,
	T![Ident],
	Option<T![,]>,
	Option<CounterStyle<'a>>,
);

function_set!(pub struct TargetCountersFunctionName "target-counters");

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct TargetCountersFunction<'a>(Function<TargetCountersFunctionName, TargetCountersParams<'a>>);

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

function_set!(pub struct TargetTextFunctionName "target-text");

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct TargetTextFunction(Function<TargetTextFunctionName, TargetTextParams>);

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
