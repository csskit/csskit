use crate::units::LengthPercentage;
use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics};

// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-translatex
// translateX() = translateX( <length-percentage> )
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TranslateX {
	translate: T![Function],
	pub x: LengthPercentage,
	close: Option<T![')']>,
}

impl<'a> Peek<'a> for TranslateX {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "translatex")
	}
}

impl<'a> Parse<'a> for TranslateX {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let translate = p.parse::<T![Function]>()?;
		let c: Cursor = translate.into();
		if !p.eq_ignore_ascii_case(c, "translatex") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
		let x = p.parse::<LengthPercentage>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(TranslateX { translate, x, close })
	}
}

impl<'a> ToCursors for TranslateX {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.translate, s);
		ToCursors::to_cursors(&self.x, s);
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
		assert_eq!(std::mem::size_of::<TranslateX>(), 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TranslateX, "translateX(10px)");
		assert_parse!(TranslateX, "translateX(45%)");
		assert_parse!(TranslateX, "translateX(2rem)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(TranslateX, "translateX(auto)");
	}
}
