use super::prelude::*;
use crate::{AtPosition, CalcableValue, LengthPercentage, RadialExtent};

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
pub struct CircleFunction<'a> {
	#[atom(CssAtomSet::Circle)]
	pub name: T![Function],
	pub radius: Option<CircleRadius<'a>>,
	pub at: Option<AtPosition<'a>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-images-4/#typedef-radial-size>, restricted
/// to the single-radius forms valid for `circle()`.
///
/// ```text,ignore
/// <radial-extent> | <length-percentage [0,∞]>
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CircleRadius<'a> {
	Extent(RadialExtent),
	Length(CalcableValue<'a, LengthPercentage>),
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
		assert_parse!(CssAtomSet::ATOMS, CircleFunction, "circle(calc(50% - 10px))");
		assert_parse!(CssAtomSet::ATOMS, CircleFunction, "circle(var(--r) at center)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, CircleFunction, "ellipse(10px)");
		assert_parse_error!(CssAtomSet::ATOMS, CircleFunction, "circle(50px 50px)");
	}
}
