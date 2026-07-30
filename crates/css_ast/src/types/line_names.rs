use super::prelude::*;
use crate::CustomIdent;

/// <https://drafts.csswg.org/css-grid-2/#typedef-line-names>
///
/// ```text,ignore
/// <line-names> = '[' <custom-ident>* ']'
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LineNames<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub open: T!['['],
	pub idents: Vec<'a, CustomIdent>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: T![']'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineNames, "[]");
		assert_parse!(CssAtomSet::ATOMS, LineNames, "[foo]");
		assert_parse!(CssAtomSet::ATOMS, LineNames, "[foo bar]");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, LineNames, "[");
		assert_peek_false!(CssAtomSet::ATOMS, LineNames, "foo");
	}
}
