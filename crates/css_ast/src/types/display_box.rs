use super::prelude::*;

/// <https://drafts.csswg.org/css-display-4/#typedef-display-box>
///
/// ```text,ignore
/// <display-box> = contents | none
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum DisplayBox {
	#[atom(CssAtomSet::Contents)]
	Contents(T![Ident]),
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DisplayBox>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DisplayBox, "contents");
		assert_parse!(CssAtomSet::ATOMS, DisplayBox, "none");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, DisplayBox, "foo");
	}
}
