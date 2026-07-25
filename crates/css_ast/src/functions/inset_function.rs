use super::prelude::*;

/// <https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-inset>
///
/// ```text,ignore
/// <inset()> = inset(
///   <length-percentage>{1,4}
///   [ round <'border-radius'> ]?
/// )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct InsetFunction<'a> {
	#[atom(CssAtomSet::Inset)]
	pub name: T![Function],
	pub params: InsetFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[syntax(" <length-percentage>{1,4} [ round <'border-radius'> ]? ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct InsetFunctionParams<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, InsetFunction, "inset(100%)");
		assert_parse!(CssAtomSet::ATOMS, InsetFunction, "inset(10px 20px)");
		assert_parse!(CssAtomSet::ATOMS, InsetFunction, "inset(10px 20px 30px)");
		assert_parse!(CssAtomSet::ATOMS, InsetFunction, "inset(10px 20px 30px 40px)");
		assert_parse!(CssAtomSet::ATOMS, InsetFunction, "inset(10px round 5px)");
		assert_parse!(CssAtomSet::ATOMS, InsetFunction, "inset(10px round 5px / 10px)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, InsetFunction, "circle(10px)");
		assert_parse_error!(CssAtomSet::ATOMS, InsetFunction, "inset()");
		assert_parse_error!(CssAtomSet::ATOMS, InsetFunction, "inset(10px 20px 30px 40px 50px)");
	}
}
