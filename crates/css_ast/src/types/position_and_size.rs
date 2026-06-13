use super::prelude::*;
use crate::{BgSize, Position};

/// Represents the `<position> [ / <bg-size> ]?` component shared by the `fill`
/// and `stroke` shorthands (mirrors `<bg-position> [ / <bg-size> ]?` used by
/// `background`, but using the plain `<position>` type per `fill-position`
/// and `stroke-position`).
///
/// ```text,ignore
/// <position> [ / <bg-size> ]?
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PositionAndSize<'a> {
	pub position: Position,
	#[semantic_eq(skip)]
	pub size: Option<(T![/], BgSize<'a>)>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PositionAndSize>(), 136);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PositionAndSize, "center");
		assert_parse!(CssAtomSet::ATOMS, PositionAndSize, "50% 50%");
		assert_parse!(CssAtomSet::ATOMS, PositionAndSize, "center / cover");
		assert_parse!(CssAtomSet::ATOMS, PositionAndSize, "0 0 / auto");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, PositionAndSize, "");
	}
}
