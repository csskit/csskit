use super::prelude::*;
use crate::{FixedBreadth, MinmaxFunction};

/// <https://drafts.csswg.org/css-grid-2/#typedef-fixed-size>
///
/// ```text,ignore
/// <fixed-size> = <fixed-breadth> | minmax( <fixed-breadth> , <track-breadth> ) | minmax( <inflexible-breadth> , <fixed-breadth> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FixedSize<'a> {
	FixedBreadth(FixedBreadth<'a>),
	MinmaxFunction(MinmaxFunction<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FixedSize>(), 96);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FixedSize, "10px");
		assert_parse!(CssAtomSet::ATOMS, FixedSize, "minmax(10px,1fr)");
		assert_parse!(CssAtomSet::ATOMS, FixedSize, "minmax(min-content,10px)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, FixedSize, "none");
		assert_parse_error!(CssAtomSet::ATOMS, FixedSize, "-10px");
	}
}
