use super::prelude::*;

use super::Length;
use crate::CalcableValue;

#[node]
#[derive(Parse, Peek, ToSpan, SemanticEq, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum LineWidth<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Thin)]
	Thin(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Medium)]
	Medium(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Thick)]
	Thick(T![Ident]),
	Length(CalcableValue<'a, Length>),
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
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineWidth, "1px");
		assert_parse!(CssAtomSet::ATOMS, LineWidth, "medium");
		assert_parse!(CssAtomSet::ATOMS, LineWidth, "calc(1px + 2px)");
		assert_parse!(CssAtomSet::ATOMS, LineWidth, "max(1px, 2px)");
	}
}
