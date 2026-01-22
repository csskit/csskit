use super::prelude::*;
use crate::LengthPercentage;

/// <https://drafts.csswg.org/css-grid-2/#funcdef-grid-template-columns-fit-content>
///
/// ```text
/// fit-content( <length-percentage> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FitContentFunction {
	#[atom(CssAtomSet::FitContent)]
	pub name: T![Function],
	pub params: LengthPercentage,
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FitContentFunction>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FitContentFunction, "fit-content(1px)");
		assert_parse!(CssAtomSet::ATOMS, FitContentFunction, "fit-content(10%)");
	}
}
