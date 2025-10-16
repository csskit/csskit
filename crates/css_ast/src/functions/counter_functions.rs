use super::prelude::*;
use crate::types::CounterStyle;

/// <https://drafts.csswg.org/css-lists-3/#counter-functions>
///
/// ```text,ignore
/// <counter()>  =  counter( <counter-name>, <counter-style>? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct CounterFunction<'a> {
	#[atom(CssAtomSet::Counter)]
	pub name: T![Function],
	pub params: CounterFunctionParams<'a>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CounterFunctionParams<'a>(T![Ident], Option<T![,]>, Option<CounterStyle<'a>>);

/// <https://drafts.csswg.org/css-lists-3/#counter-functions>
///
/// ```text,ignore
/// <counters()> = counters( <counter-name>, <string>, <counter-style>? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct CountersFunction<'a> {
	#[atom(CssAtomSet::Counters)]
	pub name: T![Function],
	pub params: CountersFunctionParams<'a>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CountersFunctionParams<'a>(T![Ident], Option<T![,]>, T![String], Option<T![,]>, Option<CounterStyle<'a>>);

// https://drafts.csswg.org/css-lists-3/#counter-functions
// <counter> = <counter()> | <counters()>
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
pub enum Counter<'a> {
	Counter(CounterFunction<'a>),
	Counters(CountersFunction<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Counter>(), 152);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Counter, "counter(foo)");
		assert_parse!(CssAtomSet::ATOMS, Counter, "counter(foo,upper-latin)");
		assert_parse!(CssAtomSet::ATOMS, Counter, "counters(foo,'bar')");
		assert_parse!(CssAtomSet::ATOMS, Counter, "counters(foo,'bar',upper-latin)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Counter, "counter('bar')");
		assert_parse_error!(CssAtomSet::ATOMS, Counter, "counters('bar')");
	}
}
