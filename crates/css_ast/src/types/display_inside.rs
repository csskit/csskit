use super::prelude::*;

/// <https://drafts.csswg.org/css-display-4/#typedef-display-inside>
///
/// ```text,ignore
/// <display-inside> = flow | flow-root | table | flex | grid | ruby
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum DisplayInside {
	#[atom(CssAtomSet::Flow)]
	Flow(T![Ident]),
	#[atom(CssAtomSet::FlowRoot)]
	FlowRoot(T![Ident]),
	#[atom(CssAtomSet::Table)]
	Table(T![Ident]),
	#[atom(CssAtomSet::Flex)]
	Flex(T![Ident]),
	#[atom(CssAtomSet::Grid)]
	Grid(T![Ident]),
	#[atom(CssAtomSet::Ruby)]
	Ruby(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DisplayInside, "flow");
		assert_parse!(CssAtomSet::ATOMS, DisplayInside, "flow-root");
		assert_parse!(CssAtomSet::ATOMS, DisplayInside, "table");
		assert_parse!(CssAtomSet::ATOMS, DisplayInside, "flex");
		assert_parse!(CssAtomSet::ATOMS, DisplayInside, "grid");
		assert_parse!(CssAtomSet::ATOMS, DisplayInside, "ruby");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, DisplayInside, "block");
	}
}
