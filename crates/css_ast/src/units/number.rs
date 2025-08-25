use css_parse::{Build, Cursor, Parser, Peek, T, keyword_set};
use csskit_derives::{IntoCursor, Peek, ToCursors};

keyword_set!(enum InfinityKeyword {
	Infnity: "infinity",
	NegInfnity: "-infinity",
});

#[derive(ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum NumberOrInfinity {
	Number(T![Number]),
	Infinity(T![Ident]),
	NegInfinity(T![Ident]),
}

impl<'a> Peek<'a> for NumberOrInfinity {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Number]>::peek(p, c) || InfinityKeyword::peek(p, c)
	}
}

impl<'a> Build<'a> for NumberOrInfinity {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		debug_assert!(Self::peek(p, c));
		if <T![Number]>::peek(p, c) {
			Self::Number(<T![Number]>::build(p, c))
		} else {
			match InfinityKeyword::build(p, c) {
				InfinityKeyword::Infnity(t) => Self::Infinity(t),
				InfinityKeyword::NegInfnity(t) => Self::NegInfinity(t),
			}
		}
	}
}

#[derive(Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum NumberOrPercentage {
	Number(T![Number]),
	Percentage(T![Dimension::%]),
}

impl<'a> Build<'a> for NumberOrPercentage {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		debug_assert!(Self::peek(p, c));
		if <T![Number]>::peek(p, c) {
			Self::Number(<T![Number]>::build(p, c))
		} else {
			Self::Percentage(<T![Dimension::%]>::build(p, c))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<NumberOrInfinity>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(NumberOrInfinity, "10000000");
		assert_parse!(NumberOrInfinity, "infinity");
		assert_parse!(NumberOrInfinity, "-infinity");
	}
}
