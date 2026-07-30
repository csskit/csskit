use super::prelude::*;
use crate::{InflexibleBreadth, TrackBreadth};

/// <https://drafts.csswg.org/css-grid-2/#funcdef-grid-template-columns-minmax>
///
/// ```text,ignore
/// minmax( <fixed-breadth> , <track-breadth> )
/// minmax( <inflexible-breadth> , <track-breadth> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MinmaxFunction<'a> {
	#[atom(CssAtomSet::Minmax)]
	pub name: T![Function],
	pub min: InflexibleBreadth<'a>,
	#[semantic_eq(skip)]
	pub comma: T![,],
	pub max: TrackBreadth<'a>,
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
		assert_parse!(CssAtomSet::ATOMS, MinmaxFunction, "minmax(100px,1fr)");
		assert_parse!(CssAtomSet::ATOMS, MinmaxFunction, "minmax(min-content,50%)");
	}
}
