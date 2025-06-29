use css_lexer::Cursor;
use css_parse::{diagnostics, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

use crate::CSSInt;

// https://drafts.csswg.org/css-fonts-4/#font-weight-absolute-values
// <font-weight-absolute> = [normal | bold | <number [1,1000]>]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum FontWeightAbsolute {
	Normal(T![Ident]),
	Bold(T![Ident]),
	Number(CSSInt),
}

impl<'a> Peek<'a> for FontWeightAbsolute {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<CSSInt>::peek(p, c)
			|| (<T![Ident]>::peek(p, c) && (p.eq_ignore_ascii_case(c, "normal") || p.eq_ignore_ascii_case(c, "bold")))
	}
}

impl<'a> Parse<'a> for FontWeightAbsolute {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Ident]>() {
			if p.eq_ignore_ascii_case(p.peek_n(1), "normal") {
				return Ok(Self::Normal(p.parse::<T![Ident]>()?));
			}
			if p.eq_ignore_ascii_case(p.peek_n(1), "bold") {
				return Ok(Self::Bold(p.parse::<T![Ident]>()?));
			}
		}
		let int = p.parse::<CSSInt>()?;
		let f: f32 = int.into();
		if f < 1.0 {
			let c: Cursor = int.into();
			Err(diagnostics::NumberTooSmall(f, c.into()))?
		}
		if f > 1000.0 {
			let c: Cursor = int.into();
			Err(diagnostics::NumberTooLarge(f, c.into()))?
		}
		Ok(Self::Number(int))
	}
}

impl From<FontWeightAbsolute> for Cursor {
	fn from(value: FontWeightAbsolute) -> Self {
		match value {
			FontWeightAbsolute::Normal(c) => c.into(),
			FontWeightAbsolute::Bold(c) => c.into(),
			FontWeightAbsolute::Number(c) => c.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FontWeightAbsolute>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontWeightAbsolute, "normal");
		assert_parse!(FontWeightAbsolute, "bold");
		assert_parse!(FontWeightAbsolute, "100");
		assert_parse!(FontWeightAbsolute, "500");
		assert_parse!(FontWeightAbsolute, "900");
	}
}
