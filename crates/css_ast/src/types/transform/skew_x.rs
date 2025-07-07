use super::SkewKind;
use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics};

// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skewx
// skewX() = skewX( [ <angle> | <zero> ] )
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SkewX {
	skew: T![Function],
	pub x: SkewKind,
	close: Option<T![')']>,
}

impl<'a> Peek<'a> for SkewX {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "skewx")
	}
}

impl<'a> Parse<'a> for SkewX {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let skew = p.parse::<T![Function]>()?;
		let c: Cursor = skew.into();
		if !p.eq_ignore_ascii_case(c, "skewx") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
		let x = p.parse::<SkewKind>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(Self { skew, x, close })
	}
}

impl<'a> ToCursors for SkewX {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.skew, s);
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
		assert_eq!(std::mem::size_of::<SkewX>(), 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SkewX, "skewX(1deg");
		assert_parse!(SkewX, "skewX(0)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(SkewX, "skewX()");
		assert_parse_error!(SkewX, "skewX(foo)");
	}
}
