use crate::units::LengthPercentage;
use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics};

// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translatey
// translateY() = translateY( <length-percentage> )
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TranslateY {
	translate: T![Function],
	pub y: LengthPercentage,
	close: Option<T![')']>,
}

impl<'a> Peek<'a> for TranslateY {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "translatey")
	}
}

impl<'a> Parse<'a> for TranslateY {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let translate = p.parse::<T![Function]>()?;
		let c: Cursor = translate.into();
		if !p.eq_ignore_ascii_case(c, "translatey") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
		let y = p.parse::<LengthPercentage>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(TranslateY { translate, y, close })
	}
}

impl<'a> ToCursors for TranslateY {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.translate, s);
		ToCursors::to_cursors(&self.y, s);
		if let Some(ref close) = self.close {
			ToCursors::to_cursors(close, s)
		};
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TranslateY>(), 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TranslateY, "translateY(10px)");
		assert_parse!(TranslateY, "translateY(45%)");
		assert_parse!(TranslateY, "translateY(2rem)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(TranslateY, "translateY(auto)");
	}
}
