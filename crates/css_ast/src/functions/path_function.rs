use super::prelude::*;
use crate::FillRuleStyleValue;

/// <https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-path>
///
/// ```text,ignore
/// <path()> = path(
///   <'fill-rule'>? ,
///   <string>
/// )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PathFunction {
	#[atom(CssAtomSet::Path)]
	pub name: T![Function],
	pub fill_rule: Option<FillRuleStyleValue>,
	#[semantic_eq(skip)]
	pub comma: Option<T![,]>,
	pub path: T![String],
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
		assert_parse!(CssAtomSet::ATOMS, PathFunction, "path('M 0 0 L 10 10')");
		assert_parse!(CssAtomSet::ATOMS, PathFunction, "path(evenodd,'M 0 0 L 10 10 Z')");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, PathFunction, "circle(10px)");
		assert_parse_error!(CssAtomSet::ATOMS, PathFunction, "path()");
	}
}
