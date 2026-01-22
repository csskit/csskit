use super::prelude::*;
use crate::{CssAtomSet, types::CounterStyle};

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum TextFunctionContent {
	#[atom(CssAtomSet::Content)]
	Content(T![Ident]),
	#[atom(CssAtomSet::Before)]
	Before(T![Ident]),
	#[atom(CssAtomSet::After)]
	After(T![Ident]),
	#[atom(CssAtomSet::FirstLetter)]
	FirstLetter(T![Ident]),
}

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
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
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

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TargetCounterFunction<'a> {
	#[atom(CssAtomSet::TargetCounter)]
	pub name: T![Function],
	pub params: TargetCounterParams<'a>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TargetCounterParams<'a>(
	TargetCounterKind,
	Option<T![,]>,
	T![Ident],
	Option<T![,]>,
	Option<CounterStyle<'a>>,
);

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TargetCountersFunction<'a> {
	#[atom(CssAtomSet::TargetCounters)]
	pub name: T![Function],
	pub params: TargetCountersParams<'a>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TargetTextFunction {
	#[atom(CssAtomSet::TargetText)]
	pub name: T![Function],
	pub params: TargetTextParams,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TargetTextParams(TargetCounterKind, Option<T![,]>, Option<TextFunctionContent>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Target>(), 184);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Target, "target-counter('foo',bar,lower-roman)");
		assert_parse!(CssAtomSet::ATOMS, Target, "target-counters('foo',bar,'baz',lower-roman)");
		assert_parse!(CssAtomSet::ATOMS, Target, "target-text('foo')");
		assert_parse!(CssAtomSet::ATOMS, Target, "target-text('foo',before)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-counter()");
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-counter('foo')");
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-counters()");
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-counters('foo')");
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-counters('foo',bar)");
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-text()");
		assert_parse_error!(CssAtomSet::ATOMS, Target, "target-text(123)");
	}
}
