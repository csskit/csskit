use super::prelude::*;
use crate::{NonNegative, NumericValue};

/// <https://drafts.csswg.org/css-animations/#typedef-single-animation-iteration-count>
///
/// ```text,ignore
/// <single-animation-iteration-count> = infinite | <number [0,∞]>
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, SemanticEq, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SingleAnimationIterationCount<'a> {
	#[atom(CssAtomSet::Infinite)]
	Infinite(T![Ident]),
	Number(NumericValue<'a, NonNegative<T![Number]>>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationIterationCount, "infinite");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationIterationCount, "1");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationIterationCount, "2.5");
	}

	#[test]
	fn test_substitution() {
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationIterationCount, "var(--n)");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationIterationCount, "calc(1 + 1)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, SingleAnimationIterationCount, "-1");
	}
}
