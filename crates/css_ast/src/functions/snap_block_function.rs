use super::prelude::*;
use crate::units::LengthPercentage;

// https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-block
// snap-block() = snap-block( <length> , [ start | end | near ]? )
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct SnapBlockFunction {
	#[atom(CssAtomSet::SnapBlock)]
	pub name: T![Function],
	pub params: SnapBlockFunctionParams,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SnapBlockFunctionParams(LengthPercentage, Option<T![,]>, Option<SnapBlockKeyword>, Option<T![,]>);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum SnapBlockKeyword {
	#[atom(CssAtomSet::Start)]
	Start(T![Ident]),
	#[atom(CssAtomSet::End)]
	End(T![Ident]),
	#[atom(CssAtomSet::Near)]
	Near(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SnapBlockFunction>(), 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SnapBlockFunction, "snap-block(10%)");
		assert_parse!(CssAtomSet::ATOMS, SnapBlockFunction, "snap-block(10%,start)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, SnapBlockFunction, "snap-inline(10%)");
		assert_parse_error!(CssAtomSet::ATOMS, SnapBlockFunction, "snap-block(start)");
	}
}
