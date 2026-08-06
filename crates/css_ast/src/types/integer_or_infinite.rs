use super::prelude::*;
use crate::{CSSInt, CalcableValue};

/// ```text,ignore
/// <integer-infinite> = infinite | <integer>
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, SemanticEq, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum IntegerOrInfinite<'a> {
	#[atom(CssAtomSet::Infinite)]
	Infinite(T![Ident]),
	Number(CalcableValue<'a, CSSInt>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, IntegerOrInfinite, "infinite");
		assert_parse!(CssAtomSet::ATOMS, IntegerOrInfinite, "1");
		assert_parse!(CssAtomSet::ATOMS, IntegerOrInfinite, "-2");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, IntegerOrInfinite, "1.0");
		assert_peek_false!(CssAtomSet::ATOMS, IntegerOrInfinite, "-1.3");
	}
}
