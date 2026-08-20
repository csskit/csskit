use super::prelude::*;
use crate::Percentage;

#[node]
#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum NumberOrInfinity {
	Number(T![Number]),
	#[atom(CssAtomSet::Infinity)]
	Infinity(T![Ident]),
	#[atom(CssAtomSet::_NegInfinity)]
	NegInfinity(T![Ident]),
}

#[node]
#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum NumberOrPercentage {
	Number(T![Number]),
	Percentage(Percentage),
}

impl From<NumberOrPercentage> for f32 {
	fn from(val: NumberOrPercentage) -> Self {
		match val {
			NumberOrPercentage::Number(f) => f.into(),
			NumberOrPercentage::Percentage(f) => f.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, NumberOrInfinity, "10000000");
		assert_parse!(CssAtomSet::ATOMS, NumberOrInfinity, "infinity");
		assert_parse!(CssAtomSet::ATOMS, NumberOrInfinity, "-infinity");
	}
}
