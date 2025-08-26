use css_parse::{Build, Cursor, Parser, Peek, T, keyword_set};
use csskit_derives::{IntoCursor, ToCursors, Visitable};

use super::Length;

keyword_set!(pub enum LineWidthKeyword { Thin: "thin", Medium: "medium", Thick: "thick" });

#[derive(IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub enum LineWidth {
	#[visit(skip)]
	Thin(T![Ident]),
	#[visit(skip)]
	Medium(T![Ident]),
	#[visit(skip)]
	Thick(T![Ident]),
	Length(Length),
}

impl<'a> Peek<'a> for LineWidth {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		Length::peek(p, c) || LineWidthKeyword::peek(p, c)
	}
}

impl<'a> Build<'a> for LineWidth {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if Length::peek(p, c) {
			Self::Length(Length::build(p, c))
		} else {
			match LineWidthKeyword::build(p, c) {
				LineWidthKeyword::Medium(_) => Self::Medium(<T![Ident]>::build(p, c)),
				LineWidthKeyword::Thin(_) => Self::Thin(<T![Ident]>::build(p, c)),
				LineWidthKeyword::Thick(_) => Self::Thick(<T![Ident]>::build(p, c)),
			}
		}
	}
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LineWidth>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(LineWidth, "1px");
		assert_parse!(LineWidth, "medium");
	}
}
