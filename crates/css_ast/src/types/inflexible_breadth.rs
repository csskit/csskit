use super::prelude::*;

/// <https://drafts.csswg.org/css-grid-2/#typedef-inflexible-breadth>
///
/// ```text,ignore
/// <inflexible-breadth> = <length-percentage [0,∞]> | min-content | max-content | auto
/// ```
#[syntax(" <length-percentage [0,∞]> | min-content | max-content | auto ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum InflexibleBreadth<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<InflexibleBreadth>(), 24);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, InflexibleBreadth, "10px");
		assert_parse!(CssAtomSet::ATOMS, InflexibleBreadth, "50%");
		assert_parse!(CssAtomSet::ATOMS, InflexibleBreadth, "min-content");
		assert_parse!(CssAtomSet::ATOMS, InflexibleBreadth, "max-content");
		assert_parse!(CssAtomSet::ATOMS, InflexibleBreadth, "auto");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, InflexibleBreadth, "1fr");
		assert_peek_false!(CssAtomSet::ATOMS, InflexibleBreadth, "none");
	}
}
