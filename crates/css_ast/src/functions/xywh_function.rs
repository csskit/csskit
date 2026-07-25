use super::prelude::*;

/// <https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-xywh>
///
/// ```text,ignore
/// <xywh()> = xywh(
///   <length-percentage>{2} <length-percentage [0,∞]>{2}
///   [ round <'border-radius'> ]?
/// )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct XywhFunction<'a> {
	#[atom(CssAtomSet::Xywh)]
	pub name: T![Function],
	pub params: XywhFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[syntax(" <length-percentage>{2} <length-percentage [0,∞]>{2} [ round <'border-radius'> ]? ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct XywhFunctionParams<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, XywhFunction, "xywh(10px 20px 30px 40px)");
		assert_parse!(CssAtomSet::ATOMS, XywhFunction, "xywh(0 0 100% 50%)");
		assert_parse!(CssAtomSet::ATOMS, XywhFunction, "xywh(10px 20px 30px 40px round 5px)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, XywhFunction, "inset(10px)");
		assert_parse_error!(CssAtomSet::ATOMS, XywhFunction, "xywh(10px 20px 30px)");
		assert_parse_error!(CssAtomSet::ATOMS, XywhFunction, "xywh(10px 20px -30px 40px)");
	}
}
