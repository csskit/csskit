use super::prelude::*;

/// <https://drafts.csswg.org/css-display-4/#typedef-display-internal>
///
/// ```text,ignore
/// <display-internal> = table-row-group | table-header-group |
///           table-footer-group | table-row | table-cell |
///           table-column-group | table-column | table-caption |
///           ruby-base | ruby-text | ruby-base-container |
///           ruby-text-container
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum DisplayInternal {
	#[atom(CssAtomSet::TableRowGroup)]
	TableRowGroup(T![Ident]),
	#[atom(CssAtomSet::TableHeaderGroup)]
	TableHeaderGroup(T![Ident]),
	#[atom(CssAtomSet::TableFooterGroup)]
	TableFooterGroup(T![Ident]),
	#[atom(CssAtomSet::TableRow)]
	TableRow(T![Ident]),
	#[atom(CssAtomSet::TableCell)]
	TableCell(T![Ident]),
	#[atom(CssAtomSet::TableColumnGroup)]
	TableColumnGroup(T![Ident]),
	#[atom(CssAtomSet::TableColumn)]
	TableColumn(T![Ident]),
	#[atom(CssAtomSet::TableCaption)]
	TableCaption(T![Ident]),
	#[atom(CssAtomSet::RubyBase)]
	RubyBase(T![Ident]),
	#[atom(CssAtomSet::RubyText)]
	RubyText(T![Ident]),
	#[atom(CssAtomSet::RubyBaseContainer)]
	RubyBaseContainer(T![Ident]),
	#[atom(CssAtomSet::RubyTextContainer)]
	RubyTextContainer(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DisplayInternal>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DisplayInternal, "table-row");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, DisplayInternal, "foo");
	}
}
