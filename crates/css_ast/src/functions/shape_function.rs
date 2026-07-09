use super::prelude::*;
use crate::{FillRuleStyleValue, Position, ShapeCommand};

/// <https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-shape>
///
/// ```text,ignore
/// <shape()> = shape(
///   <'fill-rule'>?
///   from <position> ,
///   <shape-command>#
/// )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ShapeFunction<'a> {
	#[atom(CssAtomSet::Shape)]
	pub name: T![Function],
	pub fill_rule: Option<FillRuleStyleValue>,
	#[atom(CssAtomSet::From)]
	pub from: T![Ident],
	pub position: Position,
	#[semantic_eq(skip)]
	pub comma: T![,],
	pub commands: CommaSeparated<'a, ShapeCommand>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, ShapeFunction, "shape(from 5% 0%,hline to 95%,close)");
		assert_parse!(
			CssAtomSet::ATOMS,
			ShapeFunction,
			"shape(nonzero from 5px 0%,curve to 100% 5% with 100% 0%,line by -2px 3px)"
		);
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, ShapeFunction, "circle(10px)");
		assert_parse_error!(CssAtomSet::ATOMS, ShapeFunction, "shape(from 5px 0px)");
		assert_parse_error!(CssAtomSet::ATOMS, ShapeFunction, "shape(hline to 5px)");
	}
}
