use super::prelude::*;
use crate::{BasicShapeRect, CircleFunction, EllipseFunction, PathFunction, PolygonFunction, ShapeFunction};
use css_parse::Box;

/// <https://drafts.csswg.org/css-shapes-1/#typedef-basic-shape>
///
/// ```text,ignore
/// <basic-shape> = <basic-shape-rect> | <circle()> | <ellipse()> |  <polygon()> | <path()> | <shape()>
/// ```
#[node]
#[derive(Parse, Peek, SemanticEq, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum BasicShape<'a> {
	Rect(Box<'a, BasicShapeRect<'a>>),
	Circle(CircleFunction<'a>),
	Ellipse(EllipseFunction<'a>),
	Polygon(PolygonFunction<'a>),
	Path(PathFunction<'a>),
	Shape(ShapeFunction<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, BasicShape, "inset(100%)");
		assert_parse!(CssAtomSet::ATOMS, BasicShape, "circle(50% at center)");
		assert_parse!(CssAtomSet::ATOMS, BasicShape, "ellipse(20px 50px)");
		assert_parse!(CssAtomSet::ATOMS, BasicShape, "polygon(0px 0px,100% 0px,100% 100%)");
		assert_parse!(CssAtomSet::ATOMS, BasicShape, "path('M 0 0 L 10 10')");
		assert_parse!(CssAtomSet::ATOMS, BasicShape, "shape(from 5% 0%,hline to 95%,close)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, BasicShape, "none");
		assert_parse_error!(CssAtomSet::ATOMS, BasicShape, "inset()");
	}
}
