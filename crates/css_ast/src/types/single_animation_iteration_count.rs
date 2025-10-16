use super::prelude::*;

// https://drafts.csswg.org/css-animations/#typedef-single-animation-iteration-count
// <single-animation-iteration-count> = infinite | <number [0,∞]>
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum SingleAnimationIterationCount {
	#[atom(CssAtomSet::Infinite)]
	Infinite(T![Ident]),
	Number(#[in_range(0.0f32..)] T![Number]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationIterationCount, "infinite");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationIterationCount, "1");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationIterationCount, "2.5");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, SingleAnimationIterationCount, "-1");
	}
}
