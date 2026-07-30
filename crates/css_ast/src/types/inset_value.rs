use super::prelude::*;
use crate::{CalcableValue, LengthPercentage};

/// <https://drafts.csswg.org/css-gaps-1/#typedef-inset-value>
///
/// ```text,ignore
/// <inset-value> = <length-percentage> | overlap-join
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, SemanticEq, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum InsetValue<'a> {
	LengthPercentage(CalcableValue<'a, LengthPercentage>),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::OverlapJoin)]
	OverlapJoin(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, InsetValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, InsetValue, "50%");
		assert_parse!(CssAtomSet::ATOMS, InsetValue, "overlap-join");
	}

	#[test]
	fn test_substitution() {
		assert_parse!(CssAtomSet::ATOMS, InsetValue, "calc(10px + 5%)");
		assert_parse!(CssAtomSet::ATOMS, InsetValue, "var(--x)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, InsetValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, InsetValue, "foo");
	}
}
