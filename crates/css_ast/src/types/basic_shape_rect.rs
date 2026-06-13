use super::prelude::*;
use crate::{InsetFunction, ShapeRectFunction, XywhFunction};

/// <https://drafts.csswg.org/css-shapes-1/#typedef-basic-shape-rect>
///
/// ```text,ignore
/// <basic-shape-rect> = <inset()> | <rect()> | <xywh()>
/// ```
#[derive(Parse, Peek, SemanticEq, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum BasicShapeRect<'a> {
	Inset(InsetFunction<'a>),
	Rect(ShapeRectFunction<'a>),
	Xywh(XywhFunction<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, BasicShapeRect, "inset(100%)");
		assert_parse!(CssAtomSet::ATOMS, BasicShapeRect, "rect(10px 20px 30px 40px)");
		assert_parse!(CssAtomSet::ATOMS, BasicShapeRect, "xywh(0 0 100% 50%)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, BasicShapeRect, "circle(10px)");
		assert_parse_error!(CssAtomSet::ATOMS, BasicShapeRect, "inset()");
	}
}
