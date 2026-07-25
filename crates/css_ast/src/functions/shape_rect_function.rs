use super::prelude::*;

/// <https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-rect>
///
/// ```text,ignore
/// <rect()> = rect(
///   [ <length-percentage> | auto ]{4}
///   [ round <'border-radius'> ]?
/// )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ShapeRectFunction<'a> {
	#[atom(CssAtomSet::Rect)]
	pub name: T![Function],
	pub params: ShapeRectFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[syntax(" [ <length-percentage> | auto ]{4} [ round <'border-radius'> ]? ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ShapeRectFunctionParams<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, ShapeRectFunction, "rect(10px 20px 30px 40px)");
		assert_parse!(CssAtomSet::ATOMS, ShapeRectFunction, "rect(auto auto auto auto)");
		assert_parse!(CssAtomSet::ATOMS, ShapeRectFunction, "rect(10px auto 30px auto)");
		assert_parse!(CssAtomSet::ATOMS, ShapeRectFunction, "rect(10px 20px 30px 40px round 5px)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, ShapeRectFunction, "inset(10px)");
		assert_parse_error!(CssAtomSet::ATOMS, ShapeRectFunction, "rect(10px 20px 30px)");
	}
}
