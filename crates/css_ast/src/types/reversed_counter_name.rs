use super::prelude::*;
use crate::CounterName;

/// <https://drafts.csswg.org/css-lists-3/#typedef-reversed-counter-name>
///
/// ```text,ignore
/// <reversed-counter-name> = reversed( <counter-name> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ReversedCounterName {
	#[atom(CssAtomSet::Reversed)]
	pub function: T![Function],
	pub name: CounterName,
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ReversedCounterName>(), 36);
	}

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, ReversedCounterName, "reversed(my-counter)");
		assert_parse!(CssAtomSet::ATOMS, ReversedCounterName, "reversed(foo)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ReversedCounterName, "");
		assert_parse_error!(CssAtomSet::ATOMS, ReversedCounterName, "my-counter");
		assert_parse_error!(CssAtomSet::ATOMS, ReversedCounterName, "counter(foo)");
	}
}
