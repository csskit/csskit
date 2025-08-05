use css_lexer::Cursor;
use css_parse::{Parse, Parser, Peek, Result as ParserResult, T, keyword_set};
use csskit_derives::{IntoCursor, ToCursors};

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

impl<'a> Parse<'a> for NumberOrInfinity {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		dbg!(p.peek_n(1));
		if p.peek::<T![Number]>() {
			p.parse::<T![Number]>().map(Self::Number)
		} else {
			Ok(match p.parse::<InfinityKeyword>()? {
				InfinityKeyword::Infnity(t) => Self::Infinity(t),
				InfinityKeyword::NegInfnity(t) => Self::NegInfinity(t),
			})
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
