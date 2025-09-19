use super::prelude::*;

use crate::types::CounterStyle;

function_set!(pub struct CounterFunctionName "counter");
function_set!(pub struct CountersFunctionName "counters");

/// <https://drafts.csswg.org/css-lists-3/#counter-functions>
///
/// ```text,ignore
/// <counter()>  =  counter( <counter-name>, <counter-style>? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct CounterFunction<'a>(Function<CounterFunctionName, CounterFunctionParams<'a>>);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CounterFunctionParams<'a>(T![Ident], Option<T![,]>, Option<CounterStyle<'a>>);

/// <https://drafts.csswg.org/css-lists-3/#counter-functions>
///
/// ```text,ignore
/// <counters()> = counters( <counter-name>, <string>, <counter-style>? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct CountersFunction<'a>(Function<CountersFunctionName, CountersFunctionParams<'a>>);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CountersFunctionParams<'a>(T![Ident], Option<T![,]>, T![String], Option<T![,]>, Option<CounterStyle<'a>>);

// https://drafts.csswg.org/css-lists-3/#counter-functions
// <counter> = <counter()> | <counters()>
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub enum Counter<'a> {
	Counter(CounterFunction<'a>),
	Counters(CountersFunction<'a>),
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
		assert_parse_error!(Counter, "counter('bar')");
		assert_parse_error!(Counter, "counters('bar')");
	}
}
