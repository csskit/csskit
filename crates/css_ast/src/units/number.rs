use super::prelude::*;
use crate::Percentage;

keyword_set!(enum InfinityKeyword {
	Infnity: "infinity",
	NegInfnity: "-infinity",
});

#[derive(ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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

impl<'a> Parse<'a> for NumberOrInfinity {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Number]>() {
			p.parse::<T![Number]>().map(Self::Number)
		} else {
			match p.parse::<InfinityKeyword>()? {
				InfinityKeyword::Infnity(t) => Ok(Self::Infinity(t)),
				InfinityKeyword::NegInfnity(t) => Ok(Self::NegInfinity(t)),
			}
		}
	}
}

#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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
