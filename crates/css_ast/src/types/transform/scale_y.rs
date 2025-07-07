use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics};

// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-scaley
// scaleY() = scaleY( <number> )
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ScaleY {
	scale: T![Function],
	pub y: T![Number],
	close: Option<T![')']>,
}

impl<'a> Peek<'a> for ScaleY {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "scaley")
	}
}

impl<'a> Parse<'a> for ScaleY {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let scale = p.parse::<T![Function]>()?;
		let c: Cursor = scale.into();
		if !p.eq_ignore_ascii_case(c, "scaley") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
		let y = p.parse::<T![Number]>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(Self { scale, y, close })
	}
}

impl<'a> ToCursors for ScaleY {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.scale, s);
		ToCursors::to_cursors(&self.y, s);
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
		assert_eq!(std::mem::size_of::<ScaleY>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ScaleY, "scaleY(1)");
		assert_parse!(ScaleY, "scaleY(0)");
		assert_parse!(ScaleY, "scaleY(1.5)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ScaleY, "scaleY()");
		assert_parse_error!(ScaleY, "scaleY(foo)");
	}
}
