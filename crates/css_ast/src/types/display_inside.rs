use super::prelude::*;

/// <https://drafts.csswg.org/css-display-4/#typedef-display-inside>
///
/// ```text,ignore
/// <display-inside> = flow | flow-root | table | flex | grid | ruby
/// ```
#[derive(Parse, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
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
