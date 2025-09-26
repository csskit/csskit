use super::prelude::*;

use super::Length;

#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum LineWidth {
	#[visit(skip)]
	#[atom(CssAtomSet::Thin)]
	Thin(T![Ident]),
	#[visit(skip)]
	#[atom(CssAtomSet::Medium)]
	Medium(T![Ident]),
	#[visit(skip)]
	#[atom(CssAtomSet::Thick)]
	Thick(T![Ident]),
	Length(Length),
}

// impl From<LineWidth> for Length {
// 	fn from(value: LineWidth) -> Self {
// 		match value {
// 			LineWidth::Thin => Length::Px(1.0.into()),
// 			LineWidth::Medium => Length::Px(3.0.into()),
// 			LineWidth::Thick => Length::Px(3.0.into()),
// 			LineWidth::Length(length) => length,
// 		}
// 	}
// }

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LineWidth>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineWidth, "1px");
		assert_parse!(CssAtomSet::ATOMS, LineWidth, "medium");
	}
}
