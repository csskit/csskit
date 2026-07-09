use super::prelude::*;
use crate::{AtPosition, RadialSize};

/// <https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-ellipse>
///
/// ```text,ignore
/// <ellipse()> = ellipse(
///   <radial-size>?
///   [ at <position> ]?
/// )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct EllipseFunction {
	#[atom(CssAtomSet::Ellipse)]
	pub name: T![Function],
	pub size: Option<RadialSize>,
	pub at: Option<AtPosition>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, EllipseFunction, "ellipse()");
		assert_parse!(CssAtomSet::ATOMS, EllipseFunction, "ellipse(20px 50px)");
		assert_parse!(CssAtomSet::ATOMS, EllipseFunction, "ellipse(4rem 50% at right center)");
		assert_parse!(CssAtomSet::ATOMS, EllipseFunction, "ellipse(closest-side closest-side at 5rem 6rem)");
		assert_parse!(CssAtomSet::ATOMS, EllipseFunction, "ellipse(closest-side farthest-side)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, EllipseFunction, "circle(10px)");
	}
}
