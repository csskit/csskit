use super::prelude::*;
use crate::{AtPosition, LengthPercentage, RadialExtent};

/// <https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-circle>
///
/// ```text,ignore
/// <circle()> = circle(
///   <radial-size>?
///   [ at <position> ]?
/// )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CircleFunction {
	#[atom(CssAtomSet::Circle)]
	pub name: T![Function],
	pub radius: Option<CircleRadius>,
	pub at: Option<AtPosition>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-images-4/#typedef-radial-size>, restricted
/// to the single-radius forms valid for `circle()`.
///
/// ```text,ignore
/// <radial-extent> | <length-percentage [0,∞]>
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CircleRadius {
	Extent(RadialExtent),
	Length(LengthPercentage),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, CircleFunction, "circle()");
		assert_parse!(CssAtomSet::ATOMS, CircleFunction, "circle(50%)");
		assert_parse!(CssAtomSet::ATOMS, CircleFunction, "circle(50px at right center)");
		assert_parse!(CssAtomSet::ATOMS, CircleFunction, "circle(closest-side at 5rem 6rem)");
		assert_parse!(CssAtomSet::ATOMS, CircleFunction, "circle(at center)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, CircleFunction, "ellipse(10px)");
		assert_parse_error!(CssAtomSet::ATOMS, CircleFunction, "circle(50px 50px)");
	}
}
