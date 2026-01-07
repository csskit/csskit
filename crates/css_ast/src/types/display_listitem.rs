use super::DisplayOutside;
use super::prelude::*;
use css_parse::parse_optionals;

/// <https://drafts.csswg.org/css-display-4/#typedef-display-listitem>
///
/// ```text,ignore
/// <display-listitem> = <display-outside>? && [ flow | flow-root ]? && list-item
/// ```
#[derive(Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
pub struct DisplayListitem {
	pub outside: Option<DisplayOutside>,
	pub inside: Option<DisplayListitemInside>,
	pub list_item: T![Ident],
}

impl<'a> Parse<'a> for DisplayListitem {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// Spec grammar: `<display-outside>? && [ flow | flow-root ]? && list-item`.
		// For parsing convenience we accept these components in any order (as if using
		// `||`), then explicitly require `list-item` so the result matches the spec.
		let (outside, inside, list_item) =
			parse_optionals!(p, outside: DisplayOutside, inside: DisplayListitemInside, list_item: T![Ident]);

		let list_item = match list_item {
			Some(li) if p.equals_atom(li.into(), &CssAtomSet::ListItem) => li,
			Some(li) => Err(Diagnostic::new(li.into(), Diagnostic::unexpected_ident))?,
			None => Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?,
		};

		Ok(Self { outside, inside, list_item })
	}
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum DisplayListitemInside {
	#[atom(CssAtomSet::Flow)]
	Flow(T![Ident]),
	#[atom(CssAtomSet::FlowRoot)]
	FlowRoot(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DisplayListitem>(), 44);
		assert_eq!(std::mem::size_of::<DisplayListitemInside>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "list-item");
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "block list-item");
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "inline list-item");
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "flow list-item");
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "flow-root list-item");
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "block flow list-item");
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "inline flow list-item");
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "block flow-root list-item");
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "inline flow-root list-item");
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "list-item block");
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "list-item flow");
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "list-item block flow");
		assert_parse!(CssAtomSet::ATOMS, DisplayListitem, "flow list-item block");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "foo");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "list-item list-item");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "flow flow list-item");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "block inline list-item");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "list-item flow flow-root");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "list item"); // missing hyphen
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "listitem"); // missing hyphen
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "block flow-root flow list-item");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitemInside, "foo");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitemInside, "list-item");

		// These tests verify that `list-item` is required (the `&&` semantics).
		// Without the explicit check for `list_item.is_some()`, these would incorrectly parse
		// because `parse_optionals!` only requires at least one component (the `||` semantics).
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "block");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "inline");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "flow");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "flow-root");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "block flow");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayListitem, "inline flow-root");
	}
}
