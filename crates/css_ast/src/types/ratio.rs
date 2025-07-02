use crate::units::CSSInt;
use css_lexer::{Cursor, SourceOffset};
use css_parse::{Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

// https://drafts.csswg.org/css-values-4/#ratios
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Ratio {
	start: SourceOffset,
	pub numerator: CSSInt,
	pub slash: Option<T![/]>,
	pub denominator: Option<CSSInt>,
}

impl<'a> Peek<'a> for Ratio {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		CSSInt::peek(p, c)
	}
}

impl<'a> Parse<'a> for Ratio {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		let numerator = p.parse::<CSSInt>()?;
		let slash = p.parse_if_peek::<T![/]>()?;
		let denominator = if slash.is_some() { Some(p.parse::<CSSInt>()?) } else { None };
		Ok(Self { start, numerator, slash, denominator })
	}
}

impl<'a> ToCursors for Ratio {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		s.append(self.numerator.into());
		if let Some(t) = self.slash {
			s.append(t.into());
		}
		if let Some(t) = self.denominator {
			s.append(t.into());
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Ratio>(), 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Ratio, "1/1");
		assert_parse!(Ratio, "5/3");
		assert_parse!(Ratio, "5");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Ratio, "5 : 3");
		assert_parse_error!(Ratio, "5 / 1 / 1");
	}

	// #[cfg(feature = "serde")]
	// #[test]
	// fn test_serializes() {
	// 	assert_json!(Ratio, "5/3", {
	// 		"node": [5, 3],
	// 		"start": 0,
	// 		"end": 5
	// 	});
	// }
}
