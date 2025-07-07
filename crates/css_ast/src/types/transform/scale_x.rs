use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics};

// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-scalex
// scaleX() = scaleX( <number> )
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ScaleX {
	scale: T![Function],
	pub x: T![Number],
	close: Option<T![')']>,
}

impl<'a> Peek<'a> for ScaleX {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "scalex")
	}
}

impl<'a> Parse<'a> for ScaleX {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let scale = p.parse::<T![Function]>()?;
		let c: Cursor = scale.into();
		if !p.eq_ignore_ascii_case(c, "scalex") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
		let x = p.parse::<T![Number]>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(Self { scale, x, close })
	}
}

impl<'a> ToCursors for ScaleX {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.scale, s);
		ToCursors::to_cursors(&self.x, s);
		if let Some(ref close) = self.close {
			ToCursors::to_cursors(close, s);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ScaleX>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ScaleX, "scaleX(1)");
		assert_parse!(ScaleX, "scaleX(0)");
		assert_parse!(ScaleX, "scaleX(1.5)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ScaleX, "scaleX()");
		assert_parse_error!(ScaleX, "scaleX(foo)");
	}
}
