use super::prelude::*;
use crate::{CalcableValue, LengthPercentage};

/// <https://drafts.csswg.org/css-grid-2/#funcdef-grid-template-columns-fit-content>
///
/// ```text
/// fit-content( <length-percentage> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FitContentFunction<'a> {
	#[atom(CssAtomSet::FitContent)]
	pub name: T![Function],
	pub params: CalcableValue<'a, LengthPercentage>,
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
		assert_parse!(CssAtomSet::ATOMS, FitContentFunction, "fit-content(1px)");
		assert_parse!(CssAtomSet::ATOMS, FitContentFunction, "fit-content(10%)");
		assert_parse!(CssAtomSet::ATOMS, FitContentFunction, "fit-content(var(--x))");
		assert_parse!(CssAtomSet::ATOMS, FitContentFunction, "fit-content(calc(100% - 20px))");
	}
}
