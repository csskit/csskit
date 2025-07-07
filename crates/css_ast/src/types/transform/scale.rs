use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics};

// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-scale
// scale() = scale( <number> , <number>? )
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Scale {
	scale: T![Function],
	pub x: T![Number],
	x_: Option<T![,]>,
	pub y: Option<T![Number]>,
	close: Option<T![')']>,
}

impl<'a> Peek<'a> for Scale {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "scale")
	}
}

impl<'a> Parse<'a> for Scale {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let scale = p.parse::<T![Function]>()?;
		let c: Cursor = scale.into();
		if !p.eq_ignore_ascii_case(c, "scale") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
		let x = p.parse::<T![Number]>()?;
		let x_ = p.parse_if_peek::<T![,]>()?;
		let y = p.parse_if_peek::<T![Number]>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(Scale { scale, x, x_, y, close })
	}
}

impl<'a> ToCursors for Scale {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.scale, s);
		ToCursors::to_cursors(&self.x, s);
		if let Some(ref x_) = self.x_ {
			ToCursors::to_cursors(x_, s);
		}
		if let Some(ref y) = self.y {
			ToCursors::to_cursors(y, s);
		}
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
		assert_eq!(std::mem::size_of::<Scale>(), 72);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Scale, "scale(1,2)");
		assert_parse!(Scale, "scale(0,0)");
		assert_parse!(Scale, "scale(1)");
		assert_parse!(Scale, "scale(1.5,2.5)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Scale, "scale()");
		assert_parse_error!(Scale, "scale(foo)");
	}
}
