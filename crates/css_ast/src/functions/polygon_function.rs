use super::prelude::*;
use crate::{CalcableValue, FillRuleStyleValue, Length, LengthPercentage};
use css_parse::Box;

/// <https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-polygon>
///
/// ```text,ignore
/// <polygon()> = polygon(
///   <'fill-rule'>?
///   [ round <length> ]? ,
///   [<length-percentage> <length-percentage>]#
/// )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PolygonFunction<'a> {
	#[atom(CssAtomSet::Polygon)]
	pub name: T![Function],
	pub fill_rule: Option<FillRuleStyleValue<'a>>,
	pub round: Option<Box<'a, PolygonRound<'a>>>,
	#[semantic_eq(skip)]
	pub comma: Option<T![,]>,
	pub points: CommaSeparated<'a, (CalcableValue<'a, LengthPercentage>, CalcableValue<'a, LengthPercentage>)>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PolygonRound<'a> {
	#[atom(CssAtomSet::Round)]
	pub keyword: T![Ident],
	pub radius: CalcableValue<'a, Length>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, PolygonFunction, "polygon(0px 0px,100% 0px,100% 100%)");
		assert_parse!(CssAtomSet::ATOMS, PolygonFunction, "polygon(nonzero,0 0,50% 0,100% 100%)");
		assert_parse!(CssAtomSet::ATOMS, PolygonFunction, "polygon(evenodd round 5px,0 0,50% 0,100% 100%)");
		assert_parse!(CssAtomSet::ATOMS, PolygonFunction, "polygon(round calc(5px),calc(0% + 1px) var(--y),100% 100%)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, PolygonFunction, "circle(10px)");
		assert_parse_error!(CssAtomSet::ATOMS, PolygonFunction, "polygon(0px)");
	}
}
