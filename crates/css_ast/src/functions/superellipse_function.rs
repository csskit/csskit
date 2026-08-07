use super::prelude::*;
use crate::{NumberOrInfinity, NumericValue};

/// <https://drafts.csswg.org/css-borders-4/#typedef-corner-shape-value>
///
/// ```text,ignore
/// superellipse() = superellipse(<number [-∞,∞]> | infinity | -infinity)
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SuperellipseFunction<'a> {
	#[atom(CssAtomSet::Superellipse)]
	pub name: T![Function],
	pub params: NumericValue<'a, NumberOrInfinity>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SuperellipseFunction, "superellipse(2)");
		assert_parse!(CssAtomSet::ATOMS, SuperellipseFunction, "superellipse(infinity)");
	}

	#[test]
	fn test_substitution() {
		assert_parse!(CssAtomSet::ATOMS, SuperellipseFunction, "superellipse(calc(1 + 1))");
		assert_parse!(CssAtomSet::ATOMS, SuperellipseFunction, "superellipse(var(--x))");
	}
}
