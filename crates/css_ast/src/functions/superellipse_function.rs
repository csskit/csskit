use super::prelude::*;
use crate::NumberOrInfinity;

/// <https://drafts.csswg.org/css-borders-4/#typedef-corner-shape-value>
///
/// ```text,ignore
/// superellipse() = superellipse(<number [-∞,∞]> | infinity | -infinity)
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct SuperellipseFunction {
	#[atom(CssAtomSet::Superellipse)]
	pub name: T![Function],
	pub params: NumberOrInfinity,
	pub close: T![')'],
}
