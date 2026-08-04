use super::prelude::*;
use crate::{AtPosition, CalcableValue};

/// <https://drafts.csswg.org/motion-1/#funcdef-ray>
///
/// ```text,ignore
/// ray() = ray( <angle> && <ray-size>? && contain? && [at <position>]? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RayFunction<'a> {
	#[atom(CssAtomSet::Ray)]
	pub name: T![Function],
	pub params: RayFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// The arguments of [`RayFunction`], which may appear in any order.
///
/// ```text,ignore
/// <angle> && <ray-size>? && contain? && [at <position>]?
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[parse(all_must_occur)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RayFunctionParams<'a> {
	pub angle: CalcableValue<'a, Angle>,
	pub size: Option<RaySize>,
	#[atom(CssAtomSet::Contain)]
	pub contain: Option<T![Ident]>,
	pub at: Option<AtPosition<'a>>,
}

/// <https://drafts.csswg.org/motion-1/#typedef-ray-size>
///
/// ```text,ignore
/// <ray-size> = closest-side | closest-corner | farthest-side | farthest-corner | sides
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RaySize {
	#[atom(CssAtomSet::ClosestSide)]
	ClosestSide(T![Ident]),
	#[atom(CssAtomSet::ClosestCorner)]
	ClosestCorner(T![Ident]),
	#[atom(CssAtomSet::FarthestSide)]
	FarthestSide(T![Ident]),
	#[atom(CssAtomSet::FarthestCorner)]
	FarthestCorner(T![Ident]),
	#[atom(CssAtomSet::Sides)]
	Sides(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, RayFunction, "ray(45deg)");
		assert_parse!(CssAtomSet::ATOMS, RayFunction, "ray(45deg closest-side)");
		assert_parse!(CssAtomSet::ATOMS, RayFunction, "ray(contain 45deg)");
		assert_parse!(CssAtomSet::ATOMS, RayFunction, "ray(45deg sides contain)");
		assert_parse!(CssAtomSet::ATOMS, RayFunction, "ray(farthest-corner 45deg contain at center)");
		assert_parse!(CssAtomSet::ATOMS, RayFunction, "ray(at 50% 50% 0.25turn)");
		assert_parse!(CssAtomSet::ATOMS, RayFunction, "ray(calc(45deg + 1rad))");
		assert_parse!(CssAtomSet::ATOMS, RayFunction, "ray(var(--angle))");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, RayFunction, "circle(45deg)");
		assert_parse_error!(CssAtomSet::ATOMS, RayFunction, "ray()");
		assert_parse_error!(CssAtomSet::ATOMS, RayFunction, "ray(contain)");
		assert_parse_error!(CssAtomSet::ATOMS, RayFunction, "ray(45deg 45deg)");
		assert_parse_error!(CssAtomSet::ATOMS, RayFunction, "ray(45deg sides sides)");
	}
}
